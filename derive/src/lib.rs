//! Internal derive macros for `rust-stdf`.
//!
//! `#[derive(StdfRecordCodec)]` generates, from a single record `struct`
//! definition:
//!  - `new()` + `read_from_bytes()` — the eager parser, and
//!  - a zero-copy `*View` struct with a per-field byte offset and typed getters.
//!
//! The two parsing paths are therefore generated from one source (the struct's
//! field list) and cannot drift apart.
//!
//! ## Views
//! A generated `*View` borrows the raw record bytes and exposes one getter per
//! field. Instead of eagerly parsing every field into an owned `StdfRecord`
//! (which allocates a `String`/`Vec` for every `Cn`/`Bn`/`Kx*` field), a view
//! scans the record once to record each field's byte offset, then reads a field
//! only when its getter is called. Scalar fields cost O(1) and zero allocation;
//! string fields allocate only when you call `to_owned()`.
//!
//! The per-field byte offsets are stored in **private** fields of the view, so
//! users only ever interact with the typed getters — never with the raw layout.
//!
//! ## Field kinds
//! Each field's kind is inferred from its type alias: `U1 I1 U2 I2 U4 I4 U8 R4
//! R8 B1 C1 Cn Sn Bn Dn` and the arrays `KxN1 KxU1 KxU2 KxU4 KxU8 KxR4 KxCn KxSn
//! KxUf KxCf Vn`. Optional trailing fields are recognized by `Option<..>`.
//!
//! ## Attributes
//!  - `#[stdf(count = <field>)]` (field) — required on `Kx*` arrays; names the
//!    (earlier) field holding the element count (`k`).
//!  - `#[stdf(width = <field>)]` (field) — required on `KxUf`/`KxCf` arrays;
//!    names the (earlier) field holding the per-element byte width (`f`).
//!
//! The generated code refers to items that must be in scope at the derive site
//! (i.e. inside `rust_stdf::stdf_types`): the `read_*` leaf readers, `ByteOrder`,
//! `CnRef`/`SnRef`/`BnRef`/`DnRef`, `validate_offset`, and `VIEW_ABSENT_OFT`.

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, Data, DeriveInput, Fields, GenericArgument, Ident, PathArguments, Type,
};

#[proc_macro_derive(StdfRecordCodec, attributes(stdf))]
pub fn derive_stdf_record_codec(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// How a single field is parsed / laid out.
enum Kind {
    /// Scalar read with `fn(raw, pos)` (no byte order): `U1`, `I1`.
    ScalarNoOrder { read: Ident, bytes: usize },
    /// Scalar read with `fn(raw, pos, order)`: `U2 I2 U4 I4 U8 R4 R8`.
    ScalarOrder {
        read: Ident,
        bytes: usize,
        float: bool,
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
    /// `KxN1` nibble-packed array (`fn(raw, pos, k)`).
    KxN1,
    /// Fixed-width counted array. `order == false`: `fn(raw, pos, k)` (`KxU1`);
    /// `order == true`: `fn(raw, pos, order, k)` (`KxU2 KxU4 KxU8 KxR4`).
    KxFixed {
        read: Ident,
        elem: usize,
        order: bool,
    },
    /// Variable-length counted string array. `order == false`: `KxCn`;
    /// `order == true`: `KxSn`.
    KxStr { read: Ident, order: bool },
    /// `KxUf` counted array of `f`-byte integers (`fn(raw, pos, order, k, f)`).
    KxUf,
    /// `KxCf` counted array of `f`-byte strings (`fn(raw, pos, k, f)`).
    KxCf,
    /// `Vn` generic-data array (terminal, `fn(raw, pos, order, k)`).
    Vn,
}

struct FieldInfo {
    ident: Ident,
    inner_ty: Type,
    optional: bool,
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
        let (optional, inner_ty) = unwrap_option(&f.ty);
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

        let kind = classify(&leaf).ok_or_else(|| {
            syn::Error::new_spanned(
                &f.ty,
                format!("StdfRecordCodec: unsupported field kind `{leaf}`"),
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
            optional,
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
            Kind::ScalarNoOrder { bytes, .. } | Kind::ScalarOrder { bytes, .. } => {
                Some((fi.ident.to_string(), *bytes))
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

    let view_fields = infos.iter().map(|fi| {
        let id = &fi.ident;
        quote! { #id: u16, }
    });
    let view_field_idents = infos.iter().map(|fi| fi.ident.clone()).collect::<Vec<_>>();
    let scan_stmts = infos.iter().map(gen_scan);
    let eager_stmts = infos.iter().map(gen_eager);
    let getters = infos.iter().map(gen_getter);

    // The eager `read_from_bytes` only touches `order` when a field is read with
    // a byte-order-dependent leaf reader; otherwise name the param `_order`.
    let eager_uses_order = infos.iter().any(|fi| {
        matches!(
            &fi.kind,
            Kind::ScalarOrder { .. }
                | Kind::Sn
                | Kind::Dn
                | Kind::Vn
                | Kind::KxFixed { order: true, .. }
                | Kind::KxStr { order: true, .. }
                | Kind::KxUf
        )
    });
    let eager_order = if eager_uses_order {
        format_ident!("order")
    } else {
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
            /// Scan the raw record once, recording each field's byte offset.
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

            #(#getters)*
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

fn classify(leaf: &Ident) -> Option<Kind> {
    Some(match leaf.to_string().as_str() {
        "U1" => Kind::ScalarNoOrder {
            read: format_ident!("read_uint8"),
            bytes: 1,
        },
        "I1" => Kind::ScalarNoOrder {
            read: format_ident!("read_i1"),
            bytes: 1,
        },
        "U2" => Kind::ScalarOrder {
            read: format_ident!("read_u2"),
            bytes: 2,
            float: false,
        },
        "I2" => Kind::ScalarOrder {
            read: format_ident!("read_i2"),
            bytes: 2,
            float: false,
        },
        "U4" => Kind::ScalarOrder {
            read: format_ident!("read_u4"),
            bytes: 4,
            float: false,
        },
        "I4" => Kind::ScalarOrder {
            read: format_ident!("read_i4"),
            bytes: 4,
            float: false,
        },
        "U8" => Kind::ScalarOrder {
            read: format_ident!("read_u8"),
            bytes: 8,
            float: false,
        },
        "R4" => Kind::ScalarOrder {
            read: format_ident!("read_r4"),
            bytes: 4,
            float: true,
        },
        "R8" => Kind::ScalarOrder {
            read: format_ident!("read_r8"),
            bytes: 8,
            float: true,
        },
        "B1" => Kind::B1,
        "C1" => Kind::C1,
        "Cn" => Kind::Cn,
        "Sn" => Kind::Sn,
        "Bn" => Kind::Bn,
        "Dn" => Kind::Dn,
        "KxN1" => Kind::KxN1,
        "KxU1" => Kind::KxFixed {
            read: format_ident!("read_kx_u1"),
            elem: 1,
            order: false,
        },
        "KxU2" => Kind::KxFixed {
            read: format_ident!("read_kx_u2"),
            elem: 2,
            order: true,
        },
        "KxU4" => Kind::KxFixed {
            read: format_ident!("read_kx_u4"),
            elem: 4,
            order: true,
        },
        "KxU8" => Kind::KxFixed {
            read: format_ident!("read_kx_u8"),
            elem: 8,
            order: true,
        },
        "KxR4" => Kind::KxFixed {
            read: format_ident!("read_kx_r4"),
            elem: 4,
            order: true,
        },
        "KxCn" => Kind::KxStr {
            read: format_ident!("read_kx_cn"),
            order: false,
        },
        "KxSn" => Kind::KxStr {
            read: format_ident!("read_kx_sn"),
            order: true,
        },
        "KxUf" => Kind::KxUf,
        "KxCf" => Kind::KxCf,
        "Vn" => Kind::Vn,
        _ => return None,
    })
}

/// `__k` expression (a `u16`) that reads the count *value* from its stored
/// offset var `#count` during the view scan, using the count field's own width.
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

/// Count value (`u16`) for the eager reader, taken from the already-parsed field.
fn eager_count_val(fi: &FieldInfo) -> TokenStream2 {
    let c = fi.count.as_ref().unwrap();
    match fi.count_bytes {
        Some(1) => quote!(self.#c as u16),
        _ => quote!(self.#c),
    }
}

/// Count value (`u16`) for a view getter, read via the count field's getter.
fn getter_count_val(fi: &FieldInfo) -> TokenStream2 {
    let c = fi.count.as_ref().unwrap();
    match fi.count_bytes {
        Some(1) => quote!(self.#c() as u16),
        _ => quote!(self.#c()),
    }
}

/// `__f` expression (a `u8`) that reads the width *value* from its stored
/// offset var `#width` during the view scan, using the width field's own width.
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

/// Width value (`u8`) for the eager reader, taken from the already-parsed field.
fn eager_width_val(fi: &FieldInfo) -> TokenStream2 {
    let w = fi.width.as_ref().unwrap();
    match fi.width_bytes {
        Some(1) => quote!(self.#w),
        _ => quote!(self.#w as u8),
    }
}

/// Width value (`u8`) for a view getter, read via the width field's getter.
fn getter_width_val(fi: &FieldInfo) -> TokenStream2 {
    let w = fi.width.as_ref().unwrap();
    match fi.width_bytes {
        Some(1) => quote!(self.#w()),
        _ => quote!(self.#w() as u8),
    }
}

/// One `let <field>: u16 = ...;` offset-recording statement for `View::new`.
fn gen_scan(fi: &FieldInfo) -> TokenStream2 {
    let id = &fi.ident;
    match &fi.kind {
        Kind::ScalarNoOrder { bytes, .. } => scan_fixed(id, *bytes, fi.optional),
        Kind::ScalarOrder { bytes, .. } => scan_fixed(id, *bytes, fi.optional),
        Kind::B1 => scan_fixed(id, 1, fi.optional),
        Kind::C1 => scan_fixed(id, 1, fi.optional),
        // `Cn` and `Bn` share the 1-byte length prefix. For an optional field
        // the length byte must be present (eager returns `None` otherwise).
        Kind::Cn | Kind::Bn => {
            let else_body = if fi.optional {
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
            let guard = if fi.optional {
                quote! { *pos + 2 <= raw.len() }
            } else {
                quote! { *pos < raw.len() }
            };
            let else_body = if fi.optional {
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
            let guard = if fi.optional {
                quote! { *pos + 2 <= raw.len() }
            } else {
                quote! { *pos < raw.len() }
            };
            let else_body = if fi.optional {
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
        Kind::KxFixed { elem, .. } => {
            let elem = *elem;
            let count = scan_count_expr(fi);
            if fi.optional {
                quote! {
                    let #id: u16 = {
                        let __k = #count;
                        if *pos + (#elem as usize) * __k as usize <= raw.len() {
                            let off = *pos as u16;
                            *pos += (#elem as usize) * __k as usize;
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
                        *pos += (#elem as usize) * __k as usize;
                        off
                    } else {
                        VIEW_ABSENT_OFT
                    };
                }
            }
        }
        Kind::KxStr { order, .. } => {
            let count = scan_count_expr(fi);
            let advance = if *order {
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

fn scan_fixed(id: &Ident, bytes: usize, optional: bool) -> TokenStream2 {
    if optional {
        quote! {
            let #id: u16 = if *pos + #bytes <= raw.len() {
                let off = *pos as u16;
                *pos += #bytes;
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
                *pos += #bytes;
                off
            } else {
                VIEW_ABSENT_OFT
            };
        }
    }
}

/// One statement of the eager `read_from_bytes` body (writes `self.<field>`).
///
/// Optional fields are read greedily, in declaration order, until the buffer
/// runs out: the first field that does not fit is set to `None` and parsing
/// stops (`return`), so every later optional field keeps its `new()` default
/// (`None`).
fn gen_eager(fi: &FieldInfo) -> TokenStream2 {
    let id = &fi.ident;
    match (&fi.kind, fi.optional) {
        (Kind::ScalarNoOrder { read, bytes }, false) => {
            if fi.default_expr.is_some() {
                let b = *bytes;
                quote! {
                    if *pos + #b <= raw_data.len() {
                        self.#id = #read(raw_data, pos);
                    }
                }
            } else {
                quote! { self.#id = #read(raw_data, pos); }
            }
        }
        (Kind::ScalarNoOrder { read, bytes }, true) => {
            let b = *bytes;
            quote! {
                if *pos + #b > raw_data.len() {
                    self.#id = None;
                    return;
                } else {
                    self.#id = Some(#read(raw_data, pos));
                }
            }
        }
        (Kind::ScalarOrder { read, bytes, .. }, false) => {
            if fi.default_expr.is_some() {
                let b = *bytes;
                quote! {
                    if *pos + #b <= raw_data.len() {
                        self.#id = #read(raw_data, pos, order);
                    }
                }
            } else {
                quote! { self.#id = #read(raw_data, pos, order); }
            }
        }
        (Kind::ScalarOrder { read, bytes, .. }, true) => {
            let b = *bytes;
            quote! {
                if *pos + #b > raw_data.len() {
                    self.#id = None;
                    return;
                } else {
                    self.#id = Some(#read(raw_data, pos, order));
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
            let count = eager_count_val(fi);
            quote! { self.#id = read_kx_n1(raw_data, pos, #count); }
        }
        (Kind::KxN1, true) => {
            syn::Error::new_spanned(id, "optional KxN1 is not supported").to_compile_error()
        }
        (Kind::KxFixed { read, order, .. }, false) => {
            let count = eager_count_val(fi);
            if *order {
                quote! { self.#id = #read(raw_data, pos, order, #count); }
            } else {
                quote! { self.#id = #read(raw_data, pos, #count); }
            }
        }
        (Kind::KxFixed { read, elem, order }, true) => {
            let elem = *elem;
            let count = eager_count_val(fi);
            if *order {
                quote! {
                    if *pos + (#elem as usize) * (#count) as usize > raw_data.len() {
                        self.#id = None;
                        return;
                    } else {
                        self.#id = Some(#read(raw_data, pos, order, #count));
                    }
                }
            } else {
                quote! {
                    if *pos + (#elem as usize) * (#count) as usize > raw_data.len() {
                        self.#id = None;
                        return;
                    } else {
                        self.#id = Some(#read(raw_data, pos, #count));
                    }
                }
            }
        }
        (Kind::KxStr { read, order }, false) => {
            let count = eager_count_val(fi);
            if *order {
                quote! { self.#id = #read(raw_data, pos, order, #count); }
            } else {
                quote! { self.#id = #read(raw_data, pos, #count); }
            }
        }
        (Kind::KxStr { .. }, true) => {
            syn::Error::new_spanned(id, "optional KxCn/KxSn is not supported").to_compile_error()
        }
        (Kind::KxUf, false) => {
            let count = eager_count_val(fi);
            let width = eager_width_val(fi);
            quote! { self.#id = read_kx_uf(raw_data, pos, order, #count, #width); }
        }
        (Kind::KxUf, true) => {
            syn::Error::new_spanned(id, "optional KxUf is not supported").to_compile_error()
        }
        (Kind::KxCf, false) => {
            let count = eager_count_val(fi);
            let width = eager_width_val(fi);
            quote! { self.#id = read_kx_cf(raw_data, pos, #count, #width); }
        }
        (Kind::KxCf, true) => {
            syn::Error::new_spanned(id, "optional KxCf is not supported").to_compile_error()
        }
        (Kind::Vn, false) => {
            let count = eager_count_val(fi);
            quote! { self.#id = read_vn(raw_data, pos, order, #count); }
        }
        (Kind::Vn, true) => {
            syn::Error::new_spanned(id, "optional Vn is not supported").to_compile_error()
        }
    }
}

/// One `pub fn <field>(&self) -> ...` getter on the view.
fn gen_getter(fi: &FieldInfo) -> TokenStream2 {
    let id = &fi.ident;
    let ity = &fi.inner_ty;
    match (&fi.kind, fi.optional) {
        (Kind::ScalarNoOrder { read, .. }, false) => {
            let dflt = fi.default_expr.clone().unwrap_or_else(|| quote!(0));
            quote! {
                #[inline]
                pub fn #id(&self) -> #ity {
                    validate_offset(self.#id).map(|mut p| #read(self.raw, &mut p)).unwrap_or(#dflt)
                }
            }
        }
        (Kind::ScalarNoOrder { read, .. }, true) => quote! {
            #[inline]
            pub fn #id(&self) -> Option<#ity> {
                validate_offset(self.#id).map(|mut p| #read(self.raw, &mut p))
            }
        },
        (Kind::ScalarOrder { read, float, .. }, false) => {
            let dflt = fi.default_expr.clone().unwrap_or_else(|| {
                if *float {
                    quote!(0.0)
                } else {
                    quote!(0)
                }
            });
            quote! {
                #[inline]
                pub fn #id(&self) -> #ity {
                    validate_offset(self.#id)
                        .map(|mut p| #read(self.raw, &mut p, &self.order))
                        .unwrap_or(#dflt)
                }
            }
        }
        (Kind::ScalarOrder { read, .. }, true) => quote! {
            #[inline]
            pub fn #id(&self) -> Option<#ity> {
                validate_offset(self.#id).map(|mut p| #read(self.raw, &mut p, &self.order))
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
            let count = getter_count_val(fi);
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
        (Kind::KxFixed { read, order, .. }, false) => {
            let count = getter_count_val(fi);
            let call = if *order {
                quote!(#read(self.raw, &mut p, &self.order, #count))
            } else {
                quote!(#read(self.raw, &mut p, #count))
            };
            quote! {
                #[inline]
                pub fn #id(&self) -> #ity {
                    validate_offset(self.#id).map(|mut p| #call).unwrap_or_default()
                }
            }
        }
        (Kind::KxFixed { read, order, .. }, true) => {
            let count = getter_count_val(fi);
            let call = if *order {
                quote!(#read(self.raw, &mut p, &self.order, #count))
            } else {
                quote!(#read(self.raw, &mut p, #count))
            };
            quote! {
                #[inline]
                pub fn #id(&self) -> Option<#ity> {
                    validate_offset(self.#id).map(|mut p| #call)
                }
            }
        }
        (Kind::KxStr { order, .. }, false) => {
            let count = getter_count_val(fi);
            let (ref_ty, ctor) = if *order {
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
            let count = getter_count_val(fi);
            let width = getter_width_val(fi);
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
            let count = getter_count_val(fi);
            let width = getter_width_val(fi);
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
            let count = getter_count_val(fi);
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
                pub const REC_INVALID: u64 = 1u64 << #invalid_bit;
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
                    InvalidRec(RecordHeader),
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
                    ReservedRec { raw_data: &'a [u8] },
                    InvalidRec(RecordHeader),
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
                _ => Err(StdfError { code: 2, msg: "unknown type constant".to_string() }),
            }
        },
        "code_from_typ_sub" => quote! {
            match (typ, sub) {
                #( (#typs, #subs) => #codes, )*
                // rec type 180: Reserved
                // rec type 181: Reserved
                (180 | 181, _) => REC_RESERVE,
                // not matched
                _ => REC_INVALID,
            }
        },
        "name_from_code" => quote! {
            match rec_type {
                #( #codes => #lits, )*
                // rec type 180: Reserved
                // rec type 181: Reserved
                REC_RESERVE => "ReservedRec",
                // not matched
                _ => "InvalidRec",
            }
        },
        "code_from_name" => quote! {
            match rec_name {
                #( #lits => #codes, )*
                _ => REC_INVALID,
            }
        },

        // --- top level (`stdf_record_type::REC_*`) ---
        "record_new" => quote! {
            match rec_type {
                #( #qcodes => StdfRecord::#names(#names::new()), )*
                stdf_record_type::REC_RESERVE => StdfRecord::ReservedRec(ReservedRec::new()),
                // not matched
                _ => StdfRecord::InvalidRec(RecordHeader::new()),
            }
        },
        "record_type" => quote! {
            match self {
                #( StdfRecord::#names(_) => #qcodes, )*
                // rec type 180: Reserved
                // rec type 181: Reserved
                StdfRecord::ReservedRec(_) => stdf_record_type::REC_RESERVE,
                // not matched
                StdfRecord::InvalidRec(_) => stdf_record_type::REC_INVALID,
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
                    // not matched, invalid rec do not parse anything
                    StdfRecord::InvalidRec(_) => (),
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
                    stdf_record_type::REC_RESERVE => StdfRecordView::ReservedRec { raw_data },
                    // not matched
                    _ => StdfRecordView::InvalidRec(header),
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
                    StdfRecordView::ReservedRec { .. } => stdf_record_type::REC_RESERVE,
                    // not matched
                    StdfRecordView::InvalidRec(_) => stdf_record_type::REC_INVALID,
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
                    StdfRecordView::ReservedRec { raw_data } => {
                        let mut rec = ReservedRec::new();
                        rec.read_from_bytes(raw_data, &ByteOrder::LittleEndian);
                        StdfRecord::ReservedRec(rec)
                    }
                    // not matched
                    StdfRecordView::InvalidRec(h) => StdfRecord::InvalidRec(*h),
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
