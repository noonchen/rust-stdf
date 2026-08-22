//! Internal derive macros for `rust-stdf`.
//!
//! `#[derive(StdfRecordCodec)]` generates, from a single record `struct`
//! definition:
//!  - `new()` + `read_from_bytes()` — the eager parser.
//!  - A zero-copy `*View` struct and getter methods.
//!  - `StdfRecordWrite` trait implementation.
//!
//! The two parsing paths are therefore generated from one source (the struct's
//! field list) and cannot drift apart.
//!
//! ## Views
//! A generated `*View` borrows the raw record bytes and exposes one getter per
//! field. Instead of eagerly parsing every field into an owned `StdfRecord`
//! (which allocates a `String`/`Vec` for every `Cn`/`Bn`/`Kx*` field), a view
//! scans the record once and stores the byte offset of each field, the field is
//! only read when its getter is called.
//!
//! ## Attributes
//!  - `#[stdf(count = <field>)]` (field) — required on `Kx*` arrays; names the
//!    (earlier) field holding the element count (`k`).
//!  - `#[stdf(width = <field>)]` (field) — required on `KxUf`/`KxCf` arrays;
//!    names the (earlier) field holding the per-element byte width (`f`).

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, Data, DeriveInput, Fields, GenericArgument, Ident, PathArguments, Type,
};

/// `StdfRecordCodec` derive is for STDF record types, it implements the
/// eager parser, zero-copy view and `StdfRecordWrite` trait.
///
/// ## Attributes
///  - `#[stdf(count = <field>)]`: required on `Kx*` arrays; names the
///    field holding the element count (`k`).
///  - `#[stdf(width = <field>)]`: required on `KxUf`/`KxCf` arrays;
///    names the field holding the element byte width (`f`).
#[proc_macro_derive(StdfRecordCodec, attributes(stdf))]
pub fn derive_stdf_record_codec(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// Classifying record field types into kinds.
enum Kind {
    /// Single-byte scalar: `U1`, `I1`.
    Scalar1B { read_fn: Ident, size: usize },
    /// Multi-byte scalar: `U2 I2 U4 I4 U8 R4 R8`.
    ScalarMB {
        read_fn: Ident,
        size: usize,
        is_float: bool,
    },
    /// `B1` = `[u8; 1]`.
    B1,
    /// `C1` = single byte read as a `char`.
    C1,
    /// `Cn` length-prefixed string (1-byte length).
    Cn,
    /// `Sn` length-prefixed string (2-byte, byte-order-dependent length).
    Sn,
    /// `Bn` length-prefixed byte array (1-byte length).
    Bn,
    /// `Dn` bit-field byte array (2-byte, byte-order-dependent bit length).
    Dn,
    /// `KxN1` nibble-packed array.
    KxN1,
    /// Fixed-width counted array. `is_mb == false`: `KxU1`;
    /// `is_mb == true`: `KxU2 KxU4 KxU8 KxR4`.
    KxFixed {
        read_fn: Ident,
        elem_sz: usize,
        is_mb: bool,
    },
    /// Variable-length counted string array. `is_mb == false`: `KxCn`;
    /// `is_mb == true`: `KxSn`.
    KxStr { read_fn: Ident, is_mb: bool },
    /// Counted array of `f`-byte integers.
    KxUf,
    /// Counted array of `f`-byte strings.
    KxCf,
    /// `Vn` generic-data array.
    Vn,
}

/// Information about a record field, used to generate the eager parser and view.
struct FieldInfo {
    ident: Ident,
    inner_ty: Type,
    is_optional: bool,
    kind: Kind,
    count: Option<Ident>,
    /// Byte width (1 or 2) of the `count` field, resolved in a second pass.
    count_bytes: Option<usize>,
    width: Option<Ident>,
    /// Byte width (1 or 2) of the `width` field, resolved in a second pass.
    width_bytes: Option<usize>,
    /// The `smart_default` sentinel (`#[default = ..]` / `#[default(..)]`), used
    /// as the view getter's fallback when the field is absent from a short buffer.
    default_expr: Option<TokenStream2>,
}

fn expand(input: DeriveInput) -> syn::Result<TokenStream2> {
    let name = input.ident.clone();

    let view = format_ident!("{}View", name);

    let fields = match &input.data {
        Data::Struct(s) => match &s.fields {
            Fields::Named(n) => &n.named,
            _ => {
                return Err(syn::Error::new_spanned(
                    &input.ident,
                    "StdfRecordCodec requires named fields",
                ))
            }
        },
        _ => {
            return Err(syn::Error::new_spanned(
                &input.ident,
                "StdfRecordCodec can only be derived for structs",
            ))
        }
    };

    let mut infos: Vec<FieldInfo> = Vec::with_capacity(fields.len());
    for f in fields {
        let ident = f.ident.clone().unwrap();
        let (is_optional, inner_ty) = unwrap_option(&f.ty);
        let leaf = leaf_ident(&inner_ty).ok_or_else(|| {
            syn::Error::new_spanned(&f.ty, "unsupported field type for StdfRecordCodec")
        })?;

        // field-level `#[stdf(count = ident)]` / `#[stdf(width = ident)]`
        let mut count: Option<Ident> = None;
        let mut width: Option<Ident> = None;
        for attr in &f.attrs {
            if attr.path().is_ident("stdf") {
                attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("count") {
                        count = Some(meta.value()?.parse()?);
                        Ok(())
                    } else if meta.path.is_ident("width") {
                        width = Some(meta.value()?.parse()?);
                        Ok(())
                    } else {
                        Err(meta
                            .error("unknown `stdf` field attribute (expected `count` or `width`)"))
                    }
                })?;
            }
        }

        let kind = stdf_field_type_to_kind(&leaf).ok_or_else(|| {
            syn::Error::new_spanned(
                &f.ty,
                format!("StdfRecordCodec: unsupported field type `{leaf}`"),
            )
        })?;

        let needs_count = matches!(
            kind,
            Kind::KxN1
                | Kind::KxFixed { .. }
                | Kind::KxStr { .. }
                | Kind::Vn
                | Kind::KxUf
                | Kind::KxCf
        );
        let needs_width = matches!(kind, Kind::KxUf | Kind::KxCf);

        if needs_count && count.is_none() {
            return Err(syn::Error::new_spanned(
                &f.ty,
                "array field requires `#[stdf(count = <field>)]`",
            ));
        }
        if needs_width && width.is_none() {
            return Err(syn::Error::new_spanned(
                &f.ty,
                "`KxUf`/`KxCf` field requires `#[stdf(width = <field>)]`",
            ));
        }
        if width.is_some() && !needs_width {
            return Err(syn::Error::new_spanned(
                &f.ty,
                "`#[stdf(width = <field>)]` is only valid on `KxUf`/`KxCf` fields",
            ));
        }

        infos.push(FieldInfo {
            ident,
            inner_ty,
            is_optional,
            kind,
            count,
            count_bytes: None,
            width,
            width_bytes: None,
            default_expr: field_default(f),
        });
    }

    // Second pass: resolve each `count` field's byte width so array readers can
    // read the count with the correct leaf reader and cast it to `u16`.
    let width_map: std::collections::HashMap<String, usize> = infos
        .iter()
        .filter_map(|fi| match &fi.kind {
            Kind::Scalar1B { size, .. } | Kind::ScalarMB { size, .. } => {
                Some((fi.ident.to_string(), *size))
            }
            _ => None,
        })
        .collect();
    for fi in &mut infos {
        if let Some(c) = &fi.count {
            fi.count_bytes = width_map.get(&c.to_string()).copied();
        }
        if let Some(w) = &fi.width {
            fi.width_bytes = width_map.get(&w.to_string()).copied();
        }
    }

    let (rec_typ, rec_sub) = record_typ_sub(&name).ok_or_else(|| {
        syn::Error::new_spanned(
            &name,
            "StdfRecordCodec cannot find current record in the RECORDS table",
        )
    })?;

    // Reject a struct definition where a non-optional field follows an optional one.
    let mut seen_optional = false;
    for fi in &infos {
        if fi.is_optional {
            seen_optional = true;
        } else if seen_optional {
            return Err(syn::Error::new_spanned(
                &fi.inner_ty,
                "non-optional field may not follow an optional field",
            ));
        }
    }

    // record [view] related
    let view_fields = infos.iter().map(|fi| {
        let id = &fi.ident;
        // record view uses same field names as the record,
        // but stores the byte offset (u16) of each field.
        quote! { #id: u16, }
    });
    let view_field_idents = infos.iter().map(|fi| fi.ident.clone()).collect::<Vec<_>>();
    let scan_stmts = infos.iter().map(gen_scan);
    let eager_stmts = infos.iter().map(gen_eager);
    let getters = infos.iter().map(gen_getter);

    // record write related
    let validate_stmts = infos.iter().map(|fi| gen_validate(fi, &name));
    let write_stmts = infos.iter().map(gen_write);
    let payload_len_exprs = infos.iter().map(gen_payload_len);
    let optional_order_stmts = gen_optional_order(&infos, &name);

    // check `byteOrder` is needed for eager parser function `read_from_bytes`.
    let eager_uses_order = infos.iter().any(|fi| {
        matches!(
            &fi.kind,
            Kind::ScalarMB { .. }
                | Kind::Sn
                | Kind::Dn
                | Kind::Vn
                | Kind::KxFixed { is_mb: true, .. }
                | Kind::KxStr { is_mb: true, .. }
                | Kind::KxUf
        )
    });
    let eager_order = if eager_uses_order {
        format_ident!("order")
    } else {
        // add `_order` to avoid unused variable warning
        // when `byteOrder` is not needed.
        format_ident!("_order")
    };

    let view_doc = format!(
        "Zero-copy view over the field data of a `{name}` record.\n\
         \n\
         Borrows the raw bytes and reads fields on demand via getters. Create one \
         with [`Self::new`] or via [`StdfRecordView::read_from_bytes`]; convert \
         back to an owned [`{name}`] with [`Self::to_owned`]."
    );

    Ok(quote! {
        impl #name {
            #[inline(always)]
            pub fn new() -> Self {
                Self::default()
            }

            #[inline(always)]
            #[allow(clippy::int_plus_one)]
            pub fn read_from_bytes(&mut self, raw_data: &[u8], #eager_order: &ByteOrder) {
                let pos = &mut 0usize;
                #(#eager_stmts)*
            }
        }

        #[doc = #view_doc]
        #[derive(Debug, Clone, Copy)]
        pub struct #view<'a> {
            raw: &'a [u8],
            order: ByteOrder,
            #(#view_fields)*
        }

        impl<'a> #view<'a> {
            /// Scan the given raw data and store byte offset of each field.
            #[inline]
            #[allow(clippy::int_plus_one)]
            pub fn new(raw: &'a [u8], order: &ByteOrder) -> Self {
                let pos = &mut 0usize;
                #(#scan_stmts)*
                #view { raw, order: *order, #(#view_field_idents),* }
            }

            /// Re-parse the borrowed payload into the owned record.
            #[inline]
            pub fn to_owned(&self) -> #name {
                let mut rec = #name::new();
                rec.read_from_bytes(self.raw, &self.order);
                rec
            }

            /// Return the borrowed payload, used in `stdf_match_expr!(view_write)`.
            #[inline]
            pub(crate) fn raw_payload(&self) -> &'a [u8] {
                self.raw
            }

            /// Return the byte order of borrowed payload.
            #[inline]
            pub(crate) fn byte_order(&self) -> ByteOrder {
                self.order
            }

            #(#getters)*
        }

        impl StdfRecordWrite for #name {
            const REC_TYP: u8 = #rec_typ;
            const REC_SUB: u8 = #rec_sub;

            #[inline]
            fn validate(&self) -> Result<(), crate::StdfError> {
                #(#validate_stmts)*
                #optional_order_stmts
                Ok(())
            }

            #[inline]
            fn payload_len(&self) -> usize {
                0usize #(+ #payload_len_exprs)*
            }

            #[inline]
            fn write_payload<W: std::io::Write>(
                &self,
                w: &mut W,
                order: &ByteOrder,
            ) -> Result<(), crate::StdfError> {
                #(#write_stmts)*
                Ok(())
            }
        }
    })
}

/// Split `Option<T>` into `(true, T)`; anything else into `(false, ty)`.
fn unwrap_option(ty: &Type) -> (bool, Type) {
    if let Type::Path(tp) = ty {
        if let Some(seg) = tp.path.segments.last() {
            if seg.ident == "Option" {
                if let PathArguments::AngleBracketed(ab) = &seg.arguments {
                    if let Some(GenericArgument::Type(inner)) = ab.args.first() {
                        return (true, inner.clone());
                    }
                }
            }
        }
    }
    (false, ty.clone())
}

/// Last path segment ident of a type, e.g. `Cn` from `crate::Cn`.
fn leaf_ident(ty: &Type) -> Option<Ident> {
    match ty {
        Type::Path(tp) => tp.path.segments.last().map(|s| s.ident.clone()),
        _ => None,
    }
}

/// `read_*` leaf ident → corresponding `write_*` leaf ident.
fn write_ident(read_fn: &Ident) -> Ident {
    format_ident!("{}", read_fn.to_string().replacen("read_", "write_", 1))
}

/// Look up the wire `(typ, sub)` pair for a derived record from the same
/// `RECORDS` table used by the enum macros.
fn record_typ_sub(name: &Ident) -> Option<(u8, u8)> {
    RECORDS
        .iter()
        .find(|(n, _, _)| name == *n)
        .map(|(_, t, s)| (*t, *s))
}

/// Extract a `smart_default` sentinel from a field: `#[default = <expr>]` or
/// `#[default(<expr>)]`. A bare `#[default]` (type default) yields `None`.
fn field_default(f: &syn::Field) -> Option<TokenStream2> {
    for attr in &f.attrs {
        if attr.path().is_ident("default") {
            return match &attr.meta {
                syn::Meta::NameValue(nv) => {
                    let e = &nv.value;
                    Some(quote!(#e))
                }
                syn::Meta::List(list) => {
                    let ts = &list.tokens;
                    Some(quote!(#ts))
                }
                syn::Meta::Path(_) => None,
            };
        }
    }
    None
}

fn stdf_field_type_to_kind(leaf: &Ident) -> Option<Kind> {
    Some(match leaf.to_string().as_str() {
        "U1" => Kind::Scalar1B {
            read_fn: format_ident!("read_uint8"),
            size: 1,
        },
        "I1" => Kind::Scalar1B {
            read_fn: format_ident!("read_i1"),
            size: 1,
        },
        "U2" => Kind::ScalarMB {
            read_fn: format_ident!("read_u2"),
            size: 2,
            is_float: false,
        },
        "I2" => Kind::ScalarMB {
            read_fn: format_ident!("read_i2"),
            size: 2,
            is_float: false,
        },
        "U4" => Kind::ScalarMB {
            read_fn: format_ident!("read_u4"),
            size: 4,
            is_float: false,
        },
        "I4" => Kind::ScalarMB {
            read_fn: format_ident!("read_i4"),
            size: 4,
            is_float: false,
        },
        "U8" => Kind::ScalarMB {
            read_fn: format_ident!("read_u8"),
            size: 8,
            is_float: false,
        },
        "R4" => Kind::ScalarMB {
            read_fn: format_ident!("read_r4"),
            size: 4,
            is_float: true,
        },
        "R8" => Kind::ScalarMB {
            read_fn: format_ident!("read_r8"),
            size: 8,
            is_float: true,
        },
        "B1" => Kind::B1,
        "C1" => Kind::C1,
        "Cn" => Kind::Cn,
        "Sn" => Kind::Sn,
        "Bn" => Kind::Bn,
        "Dn" => Kind::Dn,
        "KxN1" => Kind::KxN1,
        "KxU1" => Kind::KxFixed {
            read_fn: format_ident!("read_kx_u1"),
            elem_sz: 1,
            is_mb: false,
        },
        "KxU2" => Kind::KxFixed {
            read_fn: format_ident!("read_kx_u2"),
            elem_sz: 2,
            is_mb: true,
        },
        "KxU4" => Kind::KxFixed {
            read_fn: format_ident!("read_kx_u4"),
            elem_sz: 4,
            is_mb: true,
        },
        "KxU8" => Kind::KxFixed {
            read_fn: format_ident!("read_kx_u8"),
            elem_sz: 8,
            is_mb: true,
        },
        "KxR4" => Kind::KxFixed {
            read_fn: format_ident!("read_kx_r4"),
            elem_sz: 4,
            is_mb: true,
        },
        "KxCn" => Kind::KxStr {
            read_fn: format_ident!("read_kx_cn"),
            is_mb: false,
        },
        "KxSn" => Kind::KxStr {
            read_fn: format_ident!("read_kx_sn"),
            is_mb: true,
        },
        "KxUf" => Kind::KxUf,
        "KxCf" => Kind::KxCf,
        "Vn" => Kind::Vn,
        _ => return None,
    })
}

/// return an expression that reads the count (K) for Kx* fields,
/// used in `gen_scan()`.
fn scan_count_expr(fi: &FieldInfo) -> TokenStream2 {
    let c = fi.count.as_ref().unwrap();
    match fi.count_bytes {
        Some(1) => quote! {
            validate_offset(#c).map(|mut cp| read_uint8(raw, &mut cp) as u16).unwrap_or(0)
        },
        _ => quote! {
            validate_offset(#c).map(|mut cp| read_u2(raw, &mut cp, &order)).unwrap_or(0)
        },
    }
}

/// return an expression that yields the count (K) for Kx* fields,
/// read via the count field's getter, used in `gen_getter()`.
fn getter_count_expr(fi: &FieldInfo) -> TokenStream2 {
    let c = fi.count.as_ref().unwrap();
    match fi.count_bytes {
        Some(1) => quote!(self.#c() as u16),
        _ => quote!(self.#c()),
    }
}

/// return an expression that yields the count (K) for Kx* fields,
/// taken from the already-parsed field, used in `gen_eager()`.
fn eager_count_expr(fi: &FieldInfo) -> TokenStream2 {
    let c = fi.count.as_ref().unwrap();
    match fi.count_bytes {
        Some(1) => quote!(self.#c as u16),
        _ => quote!(self.#c),
    }
}

/// return an expression that reads the width (f) for KxCf/KxUf fields,
/// used in `gen_scan()`.
fn scan_width_expr(fi: &FieldInfo) -> TokenStream2 {
    let w = fi.width.as_ref().unwrap();
    match fi.width_bytes {
        Some(1) => quote! {
            validate_offset(#w).map(|mut cp| read_uint8(raw, &mut cp)).unwrap_or(0)
        },
        _ => quote! {
            validate_offset(#w).map(|mut cp| read_u2(raw, &mut cp, &order) as u8).unwrap_or(0)
        },
    }
}

/// return an expression that yields the width (f) for KxCf/KxUf fields,
/// read via the width field's getter, used in `gen_getter()`.
fn getter_width_expr(fi: &FieldInfo) -> TokenStream2 {
    let w = fi.width.as_ref().unwrap();
    match fi.width_bytes {
        Some(1) => quote!(self.#w()),
        _ => quote!(self.#w() as u8),
    }
}

/// return an expression that yields the width (f) for KxCf/KxUf fields,
/// taken from the already-parsed field, used in `gen_eager()`.
fn eager_width_expr(fi: &FieldInfo) -> TokenStream2 {
    let w = fi.width.as_ref().unwrap();
    match fi.width_bytes {
        Some(1) => quote!(self.#w),
        _ => quote!(self.#w as u8),
    }
}

/// Generate field offset scanning statement in `View::new` for record view types.
fn gen_scan(fi: &FieldInfo) -> TokenStream2 {
    let id = &fi.ident;
    match &fi.kind {
        Kind::Scalar1B { size, .. } => scan_fixed(id, *size, fi.is_optional),
        Kind::ScalarMB { size, .. } => scan_fixed(id, *size, fi.is_optional),
        Kind::B1 => scan_fixed(id, 1, fi.is_optional),
        Kind::C1 => scan_fixed(id, 1, fi.is_optional),
        // `Cn` and `Bn` share the 1-byte length prefix. For an optional field
        // the length byte must be present (eager returns `None` otherwise).
        Kind::Cn | Kind::Bn => {
            let else_body = if fi.is_optional {
                quote! { *pos = raw.len() + 1; VIEW_ABSENT_OFT }
            } else {
                quote! { VIEW_ABSENT_OFT }
            };
            quote! {
                let #id: u16 = if *pos < raw.len() {
                    let off = *pos as u16;
                    *pos += 1 + raw[*pos] as usize;
                    off
                } else {
                    #else_body
                };
            }
        }
        Kind::Sn => {
            let guard = if fi.is_optional {
                quote! { *pos + 2 <= raw.len() }
            } else {
                quote! { *pos < raw.len() }
            };
            let else_body = if fi.is_optional {
                quote! { *pos = raw.len() + 1; VIEW_ABSENT_OFT }
            } else {
                quote! { VIEW_ABSENT_OFT }
            };
            quote! {
                let #id: u16 = if #guard {
                    let off = *pos as u16;
                    let __l = read_u2(raw, pos, &order) as usize;
                    *pos += __l;
                    off
                } else {
                    #else_body
                };
            }
        }
        Kind::Dn => {
            let guard = if fi.is_optional {
                quote! { *pos + 2 <= raw.len() }
            } else {
                quote! { *pos < raw.len() }
            };
            let else_body = if fi.is_optional {
                quote! { *pos = raw.len() + 1; VIEW_ABSENT_OFT }
            } else {
                quote! { VIEW_ABSENT_OFT }
            };
            quote! {
                let #id: u16 = if #guard {
                    let off = *pos as u16;
                    let __bits = read_u2(raw, pos, &order) as usize;
                    *pos += __bits.div_ceil(8);
                    off
                } else {
                    #else_body
                };
            }
        }
        Kind::KxN1 => {
            let count = scan_count_expr(fi);
            quote! {
                let #id: u16 = if *pos < raw.len() {
                    let off = *pos as u16;
                    let __k = #count;
                    *pos += (__k / 2 + __k % 2) as usize;
                    off
                } else {
                    VIEW_ABSENT_OFT
                };
            }
        }
        Kind::KxFixed { elem_sz, .. } => {
            let elem_sz = *elem_sz;
            let count = scan_count_expr(fi);
            if fi.is_optional {
                quote! {
                    let #id: u16 = {
                        let __k = #count;
                        if *pos + (#elem_sz as usize) * __k as usize <= raw.len() {
                            let off = *pos as u16;
                            *pos += (#elem_sz as usize) * __k as usize;
                            off
                        } else {
                            *pos = raw.len() + 1;
                            VIEW_ABSENT_OFT
                        }
                    };
                }
            } else {
                quote! {
                    let #id: u16 = if *pos < raw.len() {
                        let off = *pos as u16;
                        let __k = #count;
                        *pos += (#elem_sz as usize) * __k as usize;
                        off
                    } else {
                        VIEW_ABSENT_OFT
                    };
                }
            }
        }
        Kind::KxStr { is_mb, .. } => {
            let count = scan_count_expr(fi);
            let advance = if *is_mb {
                // `KxSn`: each element is a 2-byte length prefix + payload.
                quote! {
                    if *pos + 2 <= raw.len() {
                        let __l = read_u2(raw, pos, &order) as usize;
                        *pos += __l;
                    }
                }
            } else {
                // `KxCn`: each element is a 1-byte length prefix + payload.
                quote! {
                    if *pos < raw.len() {
                        *pos += 1 + raw[*pos] as usize;
                    }
                }
            };
            quote! {
                let #id: u16 = if *pos < raw.len() {
                    let off = *pos as u16;
                    let __k = #count;
                    for _ in 0..__k {
                        #advance
                    }
                    off
                } else {
                    VIEW_ABSENT_OFT
                };
            }
        }
        // `KxUf`/`KxCf`: fixed `f`-byte elements; skip `f * k` bytes.
        Kind::KxUf | Kind::KxCf => {
            let count = scan_count_expr(fi);
            let width = scan_width_expr(fi);
            quote! {
                let #id: u16 = if *pos < raw.len() {
                    let off = *pos as u16;
                    let __k = #count;
                    let __f = #width;
                    *pos += (__f as usize) * __k as usize;
                    off
                } else {
                    VIEW_ABSENT_OFT
                };
            }
        }
        // Terminal generic-data array: only its start offset is needed.
        Kind::Vn => quote! {
            let #id: u16 = if *pos < raw.len() { *pos as u16 } else { VIEW_ABSENT_OFT };
        },
    }
}

fn scan_fixed(id: &Ident, size: usize, is_optional: bool) -> TokenStream2 {
    if is_optional {
        quote! {
            let #id: u16 = if *pos + #size <= raw.len() {
                let off = *pos as u16;
                *pos += #size;
                off
            } else {
                *pos = raw.len() + 1;
                VIEW_ABSENT_OFT
            };
        }
    } else {
        quote! {
            let #id: u16 = if *pos < raw.len() {
                let off = *pos as u16;
                *pos += #size;
                off
            } else {
                VIEW_ABSENT_OFT
            };
        }
    }
}

/// Generate getter methods for record view types that return the field value.
fn gen_getter(fi: &FieldInfo) -> TokenStream2 {
    let id = &fi.ident;
    let ity = &fi.inner_ty;
    match (&fi.kind, fi.is_optional) {
        (Kind::Scalar1B { read_fn, .. }, false) => {
            let dflt = fi.default_expr.clone().unwrap_or_else(|| quote!(0));
            quote! {
                #[inline]
                pub fn #id(&self) -> #ity {
                    validate_offset(self.#id).map(|mut p| #read_fn(self.raw, &mut p)).unwrap_or(#dflt)
                }
            }
        }
        (Kind::Scalar1B { read_fn, .. }, true) => quote! {
            #[inline]
            pub fn #id(&self) -> Option<#ity> {
                validate_offset(self.#id).map(|mut p| #read_fn(self.raw, &mut p))
            }
        },
        (
            Kind::ScalarMB {
                read_fn, is_float, ..
            },
            false,
        ) => {
            let dflt = fi.default_expr.clone().unwrap_or_else(|| {
                if *is_float {
                    quote!(0.0)
                } else {
                    quote!(0)
                }
            });
            quote! {
                #[inline]
                pub fn #id(&self) -> #ity {
                    validate_offset(self.#id)
                        .map(|mut p| #read_fn(self.raw, &mut p, &self.order))
                        .unwrap_or(#dflt)
                }
            }
        }
        (Kind::ScalarMB { read_fn, .. }, true) => quote! {
            #[inline]
            pub fn #id(&self) -> Option<#ity> {
                validate_offset(self.#id).map(|mut p| #read_fn(self.raw, &mut p, &self.order))
            }
        },
        (Kind::B1, false) => quote! {
            #[inline]
            pub fn #id(&self) -> B1 {
                validate_offset(self.#id)
                    .map(|p| [self.raw.get(p).copied().unwrap_or(0)])
                    .unwrap_or([0])
            }
        },
        (Kind::B1, true) => quote! {
            #[inline]
            pub fn #id(&self) -> Option<B1> {
                validate_offset(self.#id).map(|p| [self.raw.get(p).copied().unwrap_or(0)])
            }
        },
        (Kind::Cn, false) => quote! {
            #[inline]
            pub fn #id(&self) -> CnRef<'a> {
                CnRef::read_at(self.raw, self.#id).unwrap_or_default()
            }
        },
        (Kind::Cn, true) => quote! {
            #[inline]
            pub fn #id(&self) -> Option<CnRef<'a>> {
                CnRef::read_at(self.raw, self.#id)
            }
        },
        (Kind::Sn, false) => quote! {
            #[inline]
            pub fn #id(&self) -> SnRef<'a> {
                SnRef::read_at(self.raw, self.#id, &self.order).unwrap_or_default()
            }
        },
        (Kind::Sn, true) => quote! {
            #[inline]
            pub fn #id(&self) -> Option<SnRef<'a>> {
                SnRef::read_at(self.raw, self.#id, &self.order)
            }
        },
        (Kind::Bn, false) => quote! {
            #[inline]
            pub fn #id(&self) -> BnRef<'a> {
                BnRef::read_at(self.raw, self.#id).unwrap_or_default()
            }
        },
        (Kind::Bn, true) => quote! {
            #[inline]
            pub fn #id(&self) -> Option<BnRef<'a>> {
                BnRef::read_at(self.raw, self.#id)
            }
        },
        (Kind::Dn, false) => quote! {
            #[inline]
            pub fn #id(&self) -> DnRef<'a> {
                DnRef::read_at(self.raw, self.#id, &self.order).unwrap_or_default()
            }
        },
        (Kind::Dn, true) => quote! {
            #[inline]
            pub fn #id(&self) -> Option<DnRef<'a>> {
                DnRef::read_at(self.raw, self.#id, &self.order)
            }
        },
        (Kind::C1, false) => {
            let dflt = fi.default_expr.clone().unwrap_or_else(|| quote!('\u{0}'));
            quote! {
                #[inline]
                pub fn #id(&self) -> C1 {
                    validate_offset(self.#id)
                        .map(|p| self.raw.get(p).copied().unwrap_or(0) as char)
                        .unwrap_or(#dflt)
                }
            }
        }
        (Kind::C1, true) => quote! {
            #[inline]
            pub fn #id(&self) -> Option<C1> {
                validate_offset(self.#id).map(|p| self.raw.get(p).copied().unwrap_or(0) as char)
            }
        },
        (Kind::KxN1, false) => {
            let count = getter_count_expr(fi);
            quote! {
                #[inline]
                pub fn #id(&self) -> #ity {
                    validate_offset(self.#id)
                        .map(|mut p| read_kx_n1(self.raw, &mut p, #count))
                        .unwrap_or_default()
                }
            }
        }
        (Kind::KxN1, true) => {
            syn::Error::new_spanned(id, "optional KxN1 is not supported").to_compile_error()
        }
        (Kind::KxFixed { read_fn, is_mb, .. }, false) => {
            let count = getter_count_expr(fi);
            let call = if *is_mb {
                quote!(#read_fn(self.raw, &mut p, &self.order, #count))
            } else {
                quote!(#read_fn(self.raw, &mut p, #count))
            };
            quote! {
                #[inline]
                pub fn #id(&self) -> #ity {
                    validate_offset(self.#id).map(|mut p| #call).unwrap_or_default()
                }
            }
        }
        (Kind::KxFixed { read_fn, is_mb, .. }, true) => {
            let count = getter_count_expr(fi);
            let call = if *is_mb {
                quote!(#read_fn(self.raw, &mut p, &self.order, #count))
            } else {
                quote!(#read_fn(self.raw, &mut p, #count))
            };
            quote! {
                #[inline]
                pub fn #id(&self) -> Option<#ity> {
                    validate_offset(self.#id).map(|mut p| #call)
                }
            }
        }
        (Kind::KxStr { is_mb, .. }, false) => {
            let count = getter_count_expr(fi);
            let (ref_ty, ctor) = if *is_mb {
                (
                    quote!(KxSnRef<'a>),
                    quote!(KxSnRef::new(self.raw, p, #count as usize, self.order)),
                )
            } else {
                (
                    quote!(KxCnRef<'a>),
                    quote!(KxCnRef::new(self.raw, p, #count as usize)),
                )
            };
            quote! {
                #[inline]
                pub fn #id(&self) -> #ref_ty {
                    validate_offset(self.#id).map(|p| #ctor).unwrap_or_default()
                }
            }
        }
        (Kind::KxStr { .. }, true) => {
            syn::Error::new_spanned(id, "optional KxCn/KxSn is not supported").to_compile_error()
        }
        (Kind::KxUf, false) => {
            let count = getter_count_expr(fi);
            let width = getter_width_expr(fi);
            quote! {
                #[inline]
                pub fn #id(&self) -> #ity {
                    validate_offset(self.#id)
                        .map(|mut p| read_kx_uf(self.raw, &mut p, &self.order, #count, #width))
                        .unwrap_or_default()
                }
            }
        }
        (Kind::KxUf, true) => {
            syn::Error::new_spanned(id, "optional KxUf is not supported").to_compile_error()
        }
        (Kind::KxCf, false) => {
            let count = getter_count_expr(fi);
            let width = getter_width_expr(fi);
            quote! {
                #[inline]
                pub fn #id(&self) -> KxCfRef<'a> {
                    validate_offset(self.#id)
                        .map(|p| KxCfRef::new(self.raw, p, #count as usize, #width as usize))
                        .unwrap_or_default()
                }
            }
        }
        (Kind::KxCf, true) => {
            syn::Error::new_spanned(id, "optional KxCf is not supported").to_compile_error()
        }
        (Kind::Vn, false) => {
            let count = getter_count_expr(fi);
            quote! {
                #[inline]
                pub fn #id(&self) -> #ity {
                    validate_offset(self.#id)
                        .map(|mut p| read_vn(self.raw, &mut p, &self.order, #count))
                        .unwrap_or_default()
                }
            }
        }
        (Kind::Vn, true) => {
            syn::Error::new_spanned(id, "optional Vn is not supported").to_compile_error()
        }
    }
}

/// Generate statement for eager parser `read_from_bytes`
/// for record types.
///
/// If buffer runs out when reading optional fields,
/// set current optional field to `None`, then return.
fn gen_eager(fi: &FieldInfo) -> TokenStream2 {
    let id = &fi.ident;
    match (&fi.kind, fi.is_optional) {
        (Kind::Scalar1B { read_fn, size }, false) => {
            if fi.default_expr.is_some() {
                let b = *size;
                quote! {
                    if *pos + #b <= raw_data.len() {
                        self.#id = #read_fn(raw_data, pos);
                    }
                }
            } else {
                quote! { self.#id = #read_fn(raw_data, pos); }
            }
        }
        (Kind::Scalar1B { read_fn, size }, true) => {
            let b = *size;
            quote! {
                if *pos + #b > raw_data.len() {
                    self.#id = None;
                    return;
                } else {
                    self.#id = Some(#read_fn(raw_data, pos));
                }
            }
        }
        (Kind::ScalarMB { read_fn, size, .. }, false) => {
            if fi.default_expr.is_some() {
                let b = *size;
                quote! {
                    if *pos + #b <= raw_data.len() {
                        self.#id = #read_fn(raw_data, pos, order);
                    }
                }
            } else {
                quote! { self.#id = #read_fn(raw_data, pos, order); }
            }
        }
        (Kind::ScalarMB { read_fn, size, .. }, true) => {
            let b = *size;
            quote! {
                if *pos + #b > raw_data.len() {
                    self.#id = None;
                    return;
                } else {
                    self.#id = Some(#read_fn(raw_data, pos, order));
                }
            }
        }
        (Kind::B1, false) => quote! {
            self.#id = [read_uint8(raw_data, pos)];
        },
        (Kind::B1, true) => quote! {
            if *pos + 1 > raw_data.len() {
                self.#id = None;
                return;
            } else {
                self.#id = Some([read_uint8(raw_data, pos)]);
            }
        },
        (Kind::C1, false) => {
            if fi.default_expr.is_some() {
                quote! {
                    if *pos + 1 <= raw_data.len() {
                        self.#id = read_uint8(raw_data, pos) as char;
                    }
                }
            } else {
                quote! { self.#id = read_uint8(raw_data, pos) as char; }
            }
        }
        (Kind::C1, true) => quote! {
            if *pos + 1 > raw_data.len() {
                self.#id = None;
                return;
            } else {
                self.#id = Some(read_uint8(raw_data, pos) as char);
            }
        },
        (Kind::Cn, false) => quote! {
            self.#id = read_cn(raw_data, pos);
        },
        (Kind::Cn, true) => quote! {
            if *pos + 1 > raw_data.len() {
                self.#id = None;
                return;
            } else {
                self.#id = Some(read_cn(raw_data, pos));
            }
        },
        (Kind::Sn, false) => quote! {
            self.#id = read_sn(raw_data, pos, order);
        },
        (Kind::Sn, true) => quote! {
            if *pos + 2 > raw_data.len() {
                self.#id = None;
                return;
            } else {
                self.#id = Some(read_sn(raw_data, pos, order));
            }
        },
        (Kind::Bn, false) => quote! {
            self.#id = read_bn(raw_data, pos);
        },
        (Kind::Bn, true) => quote! {
            if *pos + 1 > raw_data.len() {
                self.#id = None;
                return;
            } else {
                self.#id = Some(read_bn(raw_data, pos));
            }
        },
        (Kind::Dn, false) => quote! {
            self.#id = read_dn(raw_data, pos, order);
        },
        (Kind::Dn, true) => quote! {
            if *pos + 2 > raw_data.len() {
                self.#id = None;
                return;
            } else {
                self.#id = Some(read_dn(raw_data, pos, order));
            }
        },
        (Kind::KxN1, false) => {
            let count = eager_count_expr(fi);
            quote! { self.#id = read_kx_n1(raw_data, pos, #count); }
        }
        (Kind::KxN1, true) => {
            syn::Error::new_spanned(id, "optional KxN1 is not supported").to_compile_error()
        }
        (Kind::KxFixed { read_fn, is_mb, .. }, false) => {
            let count = eager_count_expr(fi);
            if *is_mb {
                quote! { self.#id = #read_fn(raw_data, pos, order, #count); }
            } else {
                quote! { self.#id = #read_fn(raw_data, pos, #count); }
            }
        }
        (
            Kind::KxFixed {
                read_fn,
                elem_sz,
                is_mb,
            },
            true,
        ) => {
            let elem_sz = *elem_sz;
            let count = eager_count_expr(fi);
            if *is_mb {
                quote! {
                    if *pos + (#elem_sz as usize) * (#count) as usize > raw_data.len() {
                        self.#id = None;
                        return;
                    } else {
                        self.#id = Some(#read_fn(raw_data, pos, order, #count));
                    }
                }
            } else {
                quote! {
                    if *pos + (#elem_sz as usize) * (#count) as usize > raw_data.len() {
                        self.#id = None;
                        return;
                    } else {
                        self.#id = Some(#read_fn(raw_data, pos, #count));
                    }
                }
            }
        }
        (Kind::KxStr { read_fn, is_mb }, false) => {
            let count = eager_count_expr(fi);
            if *is_mb {
                quote! { self.#id = #read_fn(raw_data, pos, order, #count); }
            } else {
                quote! { self.#id = #read_fn(raw_data, pos, #count); }
            }
        }
        (Kind::KxStr { .. }, true) => {
            syn::Error::new_spanned(id, "optional KxCn/KxSn is not supported").to_compile_error()
        }
        (Kind::KxUf, false) => {
            let count = eager_count_expr(fi);
            let width = eager_width_expr(fi);
            quote! { self.#id = read_kx_uf(raw_data, pos, order, #count, #width); }
        }
        (Kind::KxUf, true) => {
            syn::Error::new_spanned(id, "optional KxUf is not supported").to_compile_error()
        }
        (Kind::KxCf, false) => {
            let count = eager_count_expr(fi);
            let width = eager_width_expr(fi);
            quote! { self.#id = read_kx_cf(raw_data, pos, #count, #width); }
        }
        (Kind::KxCf, true) => {
            syn::Error::new_spanned(id, "optional KxCf is not supported").to_compile_error()
        }
        (Kind::Vn, false) => {
            let count = eager_count_expr(fi);
            quote! { self.#id = read_vn(raw_data, pos, order, #count); }
        }
        (Kind::Vn, true) => {
            syn::Error::new_spanned(id, "optional Vn is not supported").to_compile_error()
        }
    }
}

// Writer related helpers

fn count_usize_expr(fi: &FieldInfo) -> TokenStream2 {
    let c = fi.count.as_ref().unwrap();
    quote!(self.#c as usize)
}

fn width_usize_expr(fi: &FieldInfo) -> TokenStream2 {
    let w = fi.width.as_ref().unwrap();
    quote!(self.#w as usize)
}

/// Validation leaf call for one field, return `None` when
/// field types don't need validation, e.g. `U1 U2`.
fn validation_call(
    fi: &FieldInfo,
    target_ref: TokenStream2,
    target_val: TokenStream2,
) -> Option<TokenStream2> {
    match &fi.kind {
        Kind::Scalar1B { .. } | Kind::ScalarMB { .. } | Kind::B1 => None,
        Kind::KxFixed { .. } => {
            let k = count_usize_expr(fi);
            Some(quote!(validate_kx_count((#target_ref).len(), #k)))
        }
        Kind::C1 => Some(quote!(validate_c1(#target_val))),
        Kind::Cn => Some(quote!(validate_cn(#target_ref))),
        Kind::Sn => Some(quote!(validate_sn(#target_ref))),
        Kind::Bn => Some(quote!(validate_bn(#target_ref))),
        Kind::Dn => Some(quote!(validate_dn(#target_ref))),
        Kind::KxN1 => {
            let k = count_usize_expr(fi);
            Some(quote!(validate_kx_n1(#target_ref, #k)))
        }
        Kind::KxStr { is_mb, .. } => {
            let k = count_usize_expr(fi);
            if *is_mb {
                Some(quote!(validate_kx_sn(#target_ref, #k)))
            } else {
                Some(quote!(validate_kx_cn(#target_ref, #k)))
            }
        }
        Kind::KxUf => {
            let k = count_usize_expr(fi);
            let f = width_usize_expr(fi);
            Some(quote!(validate_kx_uf(#target_ref, #k, #f)))
        }
        Kind::KxCf => {
            let k = count_usize_expr(fi);
            let f = width_usize_expr(fi);
            Some(quote!(validate_kx_cf(#target_ref, #k, #f)))
        }
        Kind::Vn => {
            let k = count_usize_expr(fi);
            Some(quote!(validate_vn(#target_ref, #k)))
        }
    }
}

/// Generate validation statement for one field in `validate()`.
fn gen_validate(fi: &FieldInfo, record: &Ident) -> TokenStream2 {
    let id = &fi.ident;
    let field_msg = format!("{record}.{id} failed validation");

    let (target_ref, target_val) = if fi.is_optional {
        (quote!(v), quote!(*v))
    } else {
        (quote!(&self.#id), quote!(self.#id))
    };

    let Some(call) = validation_call(fi, target_ref, target_val) else {
        return quote!();
    };

    let stmt = quote! {
        #call.map_err(|kind| crate::StdfError::new(kind, #field_msg))?;
    };

    if fi.is_optional {
        quote! {
            if let Some(v) = &self.#id {
                #stmt
            }
        }
    } else {
        stmt
    }
}

/// Generate optional field order check statement in `validate()`.
fn gen_optional_order(infos: &[FieldInfo], record: &Ident) -> TokenStream2 {
    let optionals = infos
        .iter()
        .filter(|fi| fi.is_optional)
        .map(|fi| fi.ident.clone())
        .collect::<Vec<_>>();

    if optionals.is_empty() {
        return TokenStream2::new();
    }

    let arms = optionals.iter().map(|id| {
        let msg = format!("{record}.{id} appears after a None optional field");
        quote! {
            if self.#id.is_none() {
                __optional_ended = true;
            }
            if __optional_ended && self.#id.is_some() {
                return Err(crate::StdfError::new(
                    crate::stdf_error::StdfErrorKind::InvalidOptionalOrder,
                    #msg,
                ));
            }
        }
    });

    quote! {
        let mut __optional_ended = false;
        #(#arms)*
    }
}

fn gen_payload_len(fi: &FieldInfo) -> TokenStream2 {
    let id = &fi.ident;

    fn inner(fi: &FieldInfo, target: TokenStream2) -> TokenStream2 {
        match &fi.kind {
            Kind::Scalar1B { size, .. } | Kind::ScalarMB { size, .. } => {
                let b = *size;
                quote!(#b as usize)
            }
            Kind::B1 | Kind::C1 => quote!(1usize),
            Kind::Cn => quote!(1usize + (#target).len()),
            Kind::Sn => quote!(2usize + (#target).len()),
            Kind::Bn => quote!(1usize + (#target).len()),
            Kind::Dn => quote!(2usize + (#target).bit_data.len()),
            Kind::KxN1 => {
                let k = count_usize_expr(fi);
                quote!((#k).div_ceil(2))
            }
            Kind::KxFixed { elem_sz, .. } => {
                let e_sz = *elem_sz;
                let k = count_usize_expr(fi);
                quote!(#e_sz * #k)
            }
            Kind::KxStr { is_mb, .. } => {
                if *is_mb {
                    quote!((#target).iter().map(|s| 2usize + s.len()).sum::<usize>())
                } else {
                    quote!((#target).iter().map(|s| 1usize + s.len()).sum::<usize>())
                }
            }
            Kind::KxUf | Kind::KxCf => {
                let k = count_usize_expr(fi);
                let f = width_usize_expr(fi);
                quote!(#k * #f)
            }
            Kind::Vn => quote!(vn_payload_len(#target)),
        }
    }

    if fi.is_optional {
        let e = inner(fi, quote!(v));
        quote!(self.#id.as_ref().map(|v| #e).unwrap_or(0))
    } else {
        inner(fi, quote!(&self.#id))
    }
}

fn gen_write(fi: &FieldInfo) -> TokenStream2 {
    let id = &fi.ident;

    fn call(fi: &FieldInfo, target: TokenStream2) -> TokenStream2 {
        match &fi.kind {
            Kind::Scalar1B { read_fn, .. } => {
                let write = write_ident(read_fn);
                quote!(#write(w, #target)?;)
            }
            Kind::ScalarMB { read_fn, .. } => {
                let write = write_ident(read_fn);
                quote!(#write(w, #target, order)?;)
            }
            Kind::B1 => quote!(write_b1(w, #target)?;),
            Kind::C1 => quote!(write_c1(w, #target)?;),
            Kind::Cn => quote!(write_cn(w, #target)?;),
            Kind::Sn => quote!(write_sn(w, #target, order)?;),
            Kind::Bn => quote!(write_bn(w, #target)?;),
            Kind::Dn => quote!(write_dn(w, #target, order)?;),
            Kind::KxN1 => quote!(write_kx_n1(w, #target)?;),
            Kind::KxFixed {
                read_fn,
                is_mb: needs_order,
                ..
            } => {
                let write = write_ident(read_fn);
                if *needs_order {
                    quote!(#write(w, #target, order)?;)
                } else {
                    quote!(#write(w, #target)?;)
                }
            }
            Kind::KxStr { is_mb, .. } => {
                if *is_mb {
                    quote!(write_kx_sn(w, #target, order)?;)
                } else {
                    quote!(write_kx_cn(w, #target)?;)
                }
            }
            Kind::KxUf => quote!(write_kx_uf(w, #target, order)?;),
            Kind::KxCf => quote!(write_kx_cf(w, #target)?;),
            Kind::Vn => quote!(write_vn(w, #target, order)?;),
        }
    }

    if fi.is_optional {
        let target = match &fi.kind {
            Kind::Scalar1B { .. } | Kind::ScalarMB { .. } | Kind::C1 => quote!(*v),
            _ => quote!(v),
        };
        let e = call(fi, target);
        quote! {
            if let Some(v) = &self.#id {
                #e
            }
        }
    } else {
        let target = match &fi.kind {
            Kind::Scalar1B { .. } | Kind::ScalarMB { .. } | Kind::C1 => quote!(self.#id),
            _ => quote!(&self.#id),
        };
        call(fi, target)
    }
}

// ----------------------------------------------------------------------------
// `stdf_records!` / `stdf_match_expr!` — single-source STDF record table.
//
// `stdf_records!` emits the `REC_*` constants (`rec_codes`) or the two record
// enums (`rec_enums`). `stdf_match_expr!` expands to a `match` expression body
// inside a hand-written function, so every function keeps its visible
// signature/doc.
// ----------------------------------------------------------------------------

/// (name, typ, sub). Order determines the `REC_*` bit index.
const RECORDS: &[(&str, u8, u8)] = &[
    // rec type 0
    ("FAR", 0, 10),
    ("ATR", 0, 20),
    ("VUR", 0, 30),
    // rec type 1
    ("MIR", 1, 10),
    ("MRR", 1, 20),
    ("PCR", 1, 30),
    ("HBR", 1, 40),
    ("SBR", 1, 50),
    ("PMR", 1, 60),
    ("PGR", 1, 62),
    ("PLR", 1, 63),
    ("RDR", 1, 70),
    ("SDR", 1, 80),
    ("PSR", 1, 90),
    ("NMR", 1, 91),
    ("CNR", 1, 92),
    ("SSR", 1, 93),
    ("CDR", 1, 94),
    // rec type 2
    ("WIR", 2, 10),
    ("WRR", 2, 20),
    ("WCR", 2, 30),
    // rec type 5
    ("PIR", 5, 10),
    ("PRR", 5, 20),
    // rec type 10
    ("TSR", 10, 30),
    // rec type 15
    ("PTR", 15, 10),
    ("MPR", 15, 15),
    ("FTR", 15, 20),
    ("STR", 15, 30),
    // rec type 20
    ("BPS", 20, 10),
    ("EPS", 20, 20),
    // rec type 50
    ("GDR", 50, 10),
    ("DTR", 50, 30),
];

fn rec_name(s: &str) -> Ident {
    format_ident!("{}", s)
}

fn rec_code(s: &str) -> Ident {
    format_ident!("REC_{}", s)
}

fn rec_view(s: &str) -> Ident {
    format_ident!("{}View", s)
}

fn rec_lit(s: &str) -> syn::LitStr {
    syn::LitStr::new(s, proc_macro2::Span::call_site())
}

/// `EPS` is the only record without any fields,
/// so it doesn't have a view or read_* method,
/// treat it as special case.
fn is_eps(name: &str) -> bool {
    name == "EPS"
}

#[proc_macro]
pub fn stdf_records(input: TokenStream) -> TokenStream {
    let mode = parse_macro_input!(input as Ident);
    match mode.to_string().as_str() {
        "rec_codes" => {
            let codes: Vec<Ident> = RECORDS.iter().map(|r| rec_code(r.0)).collect();
            let bits: Vec<u32> = (0..RECORDS.len() as u32).collect();
            let reserve_bit = RECORDS.len() as u32;
            let invalid_bit = reserve_bit + 1;
            quote! {
                #( pub const #codes: u64 = 1 << #bits; )*
                // rec type 180: Reserved
                // rec type 181: Reserved
                pub const REC_RESERVE: u64 = 1u64 << #reserve_bit;
                pub const REC_UNKNOWN: u64 = 1u64 << #invalid_bit;
            }
            .into()
        }
        "rec_enums" => {
            let names: Vec<Ident> = RECORDS.iter().map(|r| rec_name(r.0)).collect();
            let view_variants: Vec<TokenStream2> = RECORDS
                .iter()
                .map(|r| {
                    let name = rec_name(r.0);
                    if is_eps(r.0) {
                        quote! { #name }
                    } else {
                        let view = rec_view(r.0);
                        quote! { #name(#view<'a>) }
                    }
                })
                .collect();
            quote! {
                /// `StdfRecord` is the data that returned from StdfReader iterator.
                ///
                /// it contains the actually structs
                /// that contain STDF data.
                ///
                /// use `match` structure to access the nested data.
                ///
                /// # Example
                ///
                /// ```
                /// use rust_stdf::{StdfRecord, stdf_record_type::*};
                ///
                /// let mut rec = StdfRecord::new(REC_PTR);
                /// if let StdfRecord::PTR(ref mut ptr_data) = rec {
                ///     ptr_data.result = 100.0;
                /// }
                /// println!("{:?}", rec);
                /// ```
                #[derive(Debug, Clone, PartialEq)]
                pub enum StdfRecord {
                    #( #names(#names), )*
                    // rec type 180: Reserved
                    // rec type 181: Reserved
                    ReservedRec(ReservedRec),
                    UnknownRec(ReservedRec),
                }

                /// Zero-copy view over the field data of a single STDF record.
                ///
                /// Unlike [`StdfRecord`], which eagerly parses every field into owned
                /// values, a `StdfRecordView` only records each field's byte offset and
                /// reads a field on demand. It borrows the raw bytes it was built from
                /// and never allocates for scalar fields.
                ///
                /// A view can be built from:
                /// 1. A borrowed [`RawDataElement`] (`From<&RawDataElement>`),
                /// 2. A header plus raw field data ([`StdfRecordView::read_from_bytes`]).
                /// 3. A buffer that includes the 4-byte record header and the raw field
                ///    data ([`StdfRecordView::read_from_bytes_with_header`]).
                #[derive(Debug, Clone, Copy)]
                pub enum StdfRecordView<'a> {
                    #( #view_variants, )*
                    // rec type 180: Reserved
                    // rec type 181: Reserved
                    ReservedRec(ReservedRecView<'a>),
                    UnknownRec(ReservedRecView<'a>),
                }
            }
            .into()
        }
        other => syn::Error::new(
            mode.span(),
            format!("unknown mode `{other}` (expected `rec_codes` or `rec_enums`)"),
        )
        .to_compile_error()
        .into(),
    }
}

#[proc_macro]
pub fn stdf_match_expr(input: TokenStream) -> TokenStream {
    let kind = parse_macro_input!(input as Ident);
    let kind_str = kind.to_string();

    let names: Vec<Ident> = RECORDS.iter().map(|r| rec_name(r.0)).collect();
    let codes: Vec<Ident> = RECORDS.iter().map(|r| rec_code(r.0)).collect();
    let typs: Vec<u8> = RECORDS.iter().map(|r| r.1).collect();
    let subs: Vec<u8> = RECORDS.iter().map(|r| r.2).collect();
    let lits: Vec<syn::LitStr> = RECORDS.iter().map(|r| rec_lit(r.0)).collect();
    let qcodes: Vec<TokenStream2> = codes
        .iter()
        .map(|c| quote! { stdf_record_type::#c })
        .collect();

    let out = match kind_str.as_str() {
        // --- inside `stdf_record_type` (bare `REC_*`) ---
        "typ_sub_from_code" => quote! {
            match code {
                #( #codes => Ok((#typs, #subs)), )*
                _ => Err(StdfError { code: StdfErrorKind::InvalidRecordType as u8, msg: "unknown type constant".to_string() }),
            }
        },
        "code_from_typ_sub" => quote! {
            match (typ, sub) {
                #( (#typs, #subs) => #codes, )*
                // rec type 180: Reserved
                // rec type 181: Reserved
                (180 | 181, _) => REC_RESERVE,
                // not matched
                _ => REC_UNKNOWN,
            }
        },
        "name_from_code" => quote! {
            match rec_type {
                #( #codes => #lits, )*
                // rec type 180: Reserved
                // rec type 181: Reserved
                REC_RESERVE => "ReservedRec",
                // not matched
                _ => "UnknownRec",
            }
        },
        "code_from_name" => quote! {
            match rec_name {
                #( #lits => #codes, )*
                "ReservedRec" => REC_RESERVE,
                _ => REC_UNKNOWN,
            }
        },

        // --- top level (`stdf_record_type::REC_*`) ---
        "record_new" => quote! {
            match rec_type {
                #( #qcodes => StdfRecord::#names(#names::new()), )*
                stdf_record_type::REC_RESERVE => StdfRecord::ReservedRec(ReservedRec::new()),
                // not matched
                _ => StdfRecord::UnknownRec(ReservedRec::new()),
            }
        },
        "record_type" => quote! {
            match self {
                #( StdfRecord::#names(_) => #qcodes, )*
                // rec type 180: Reserved
                // rec type 181: Reserved
                StdfRecord::ReservedRec(_) => stdf_record_type::REC_RESERVE,
                // not matched
                StdfRecord::UnknownRec(_) => stdf_record_type::REC_UNKNOWN,
            }
        },
        "record_read" => {
            let arms: Vec<TokenStream2> = RECORDS
                .iter()
                .map(|r| {
                    let name = rec_name(r.0);
                    if is_eps(r.0) {
                        quote! { StdfRecord::#name(_) => () }
                    } else {
                        quote! { StdfRecord::#name(rec) => rec.read_from_bytes(raw_data, order) }
                    }
                })
                .collect();
            quote! {
                match self {
                    #( #arms, )*
                    // rec type 180: Reserved
                    // rec type 181: Reserved
                    StdfRecord::ReservedRec(rec) => rec.read_from_bytes(raw_data, order),
                    // not matched
                    StdfRecord::UnknownRec(rec) => rec.read_from_bytes(raw_data, order),
                }
            }
        }
        "view_read" => {
            let arms: Vec<TokenStream2> = RECORDS
                .iter()
                .map(|r| {
                    let name = rec_name(r.0);
                    let code = rec_code(r.0);
                    if is_eps(r.0) {
                        quote! { stdf_record_type::#code => StdfRecordView::#name }
                    } else {
                        let view = rec_view(r.0);
                        quote! { stdf_record_type::#code => StdfRecordView::#name(#view::new(raw_data, byte_order)) }
                    }
                })
                .collect();
            quote! {
                match stdf_record_type::get_code_from_typ_sub(header.typ, header.sub) {
                    #( #arms, )*
                    // rec type 180: Reserved
                    // rec type 181: Reserved
                    stdf_record_type::REC_RESERVE => StdfRecordView::ReservedRec(ReservedRecView {
                        typ: header.typ,
                        sub: header.sub,
                        byte_order: *byte_order,
                        raw_data,
                    }),
                    // not matched
                    _ => StdfRecordView::UnknownRec(ReservedRecView {
                        typ: header.typ,
                        sub: header.sub,
                        byte_order: *byte_order,
                        raw_data,
                    }),
                }
            }
        }
        "view_type" => {
            let arms: Vec<TokenStream2> = RECORDS
                .iter()
                .map(|r| {
                    let name = rec_name(r.0);
                    let code = rec_code(r.0);
                    if is_eps(r.0) {
                        quote! { StdfRecordView::#name => stdf_record_type::#code }
                    } else {
                        quote! { StdfRecordView::#name(_) => stdf_record_type::#code }
                    }
                })
                .collect();
            quote! {
                match self {
                    #( #arms, )*
                    // rec type 180: Reserved
                    // rec type 181: Reserved
                    StdfRecordView::ReservedRec(_) => stdf_record_type::REC_RESERVE,
                    // not matched
                    StdfRecordView::UnknownRec(_) => stdf_record_type::REC_UNKNOWN,
                }
            }
        }
        "view_to_owned" => {
            let arms: Vec<TokenStream2> = RECORDS
                .iter()
                .map(|r| {
                    let name = rec_name(r.0);
                    if is_eps(r.0) {
                        quote! { StdfRecordView::#name => StdfRecord::#name(#name::new()) }
                    } else {
                        quote! { StdfRecordView::#name(v) => StdfRecord::#name(v.to_owned()) }
                    }
                })
                .collect();
            quote! {
                match self {
                    #( #arms, )*
                    // rec type 180: Reserved
                    // rec type 181: Reserved
                    StdfRecordView::ReservedRec(v) => {
                        let mut rec = ReservedRec::new();
                        rec.typ = v.typ;
                        rec.sub = v.sub;
                        rec.read_from_bytes(v.raw_data, &v.byte_order);
                        StdfRecord::ReservedRec(rec)
                    }
                    // not matched
                    StdfRecordView::UnknownRec(v) => {
                        let mut rec = ReservedRec::new();
                        rec.typ = v.typ;
                        rec.sub = v.sub;
                        rec.read_from_bytes(v.raw_data, &v.byte_order);
                        StdfRecord::UnknownRec(rec)
                    },
                }
            }
        }
        "record_write" => {
            let arms: Vec<TokenStream2> = RECORDS
                .iter()
                .map(|rec| {
                    let name = rec_name(rec.0);
                    let typ = rec.1;
                    let sub = rec.2;
                    if is_eps(rec.0) {
                        quote! { StdfRecord::#name(_) => self.write_header_and_payload(#typ, #sub, &[]) }
                    } else {
                        quote! { StdfRecord::#name(rec) => self.write_record(rec) }
                    }
                })
                .collect();
            quote! {
                match r {
                    #( #arms, )*
                    // rec type 180: Reserved
                    // rec type 181: Reserved
                    StdfRecord::ReservedRec(rec) => {
                        self.write_guarded_record(rec.typ, rec.sub, &rec.raw_data, rec.byte_order)
                    }
                    // not matched
                    StdfRecord::UnknownRec(rec) => {
                        self.write_guarded_record(rec.typ, rec.sub, &rec.raw_data, rec.byte_order)
                    }
                }
            }
        }
        "view_write" => {
            let arms: Vec<TokenStream2> = RECORDS
                .iter()
                .map(|rec| {
                    let name = rec_name(rec.0);
                    let typ = rec.1;
                    let sub = rec.2;
                    if is_eps(rec.0) {
                        quote! { StdfRecordView::#name => self.write_header_and_payload(#typ, #sub, &[]) }
                    } else {
                        // Known record: passthrough the raw bytes when the view's
                        // byte order matches the writer, otherwise re-encode.
                        quote! { StdfRecordView::#name(v) => if v.byte_order() == self.byte_order {
                            self.write_header_and_payload(#typ, #sub, v.raw_payload())
                        } else {
                            self.write_record(&v.to_owned())
                        } }
                    }
                })
                .collect();
            quote! {
                match v {
                    #( #arms, )*
                    // rec type 180: Reserved
                    // rec type 181: Reserved
                    StdfRecordView::ReservedRec(v) => {
                        self.write_guarded_record(v.typ, v.sub, v.raw_data, v.byte_order)
                    }
                    // not matched
                    StdfRecordView::UnknownRec(v) => {
                        self.write_guarded_record(v.typ, v.sub, v.raw_data, v.byte_order)
                    }
                }
            }
        }
        other => {
            return syn::Error::new(
                kind.span(),
                format!("unknown `stdf_match_expr!` kind `{other}`"),
            )
            .to_compile_error()
            .into();
        }
    };

    out.into()
}
