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
//! ## Field kinds
//! Each field's kind is inferred from its type alias: `U1 I1 U2 I2 U4 I4 U8 R4
//! R8 B1 Cn` and the arrays `KxN1 KxR4 KxU2`. Optional trailing fields are
//! recognized by `Option<..>`.
//!
//! ## Attributes
//!  - `#[stdf(view = <Ident>)]` (struct) — name of the generated view; defaults
//!    to `<Name>View`.
//!  - `#[stdf(count = <field>)]` (field) — required on `Kx*` arrays; names the
//!    (earlier) field holding the element count.
//!
//! The generated code refers to items that must be in scope at the derive site
//! (i.e. inside `rust_stdf::stdf_types`): the `read_*` leaf readers, `ByteOrder`,
//! `CnRef`, `stdf_view_opt`, and `STDF_VIEW_ABSENT`.

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
    ScalarOrder { read: Ident, bytes: usize, float: bool },
    /// `B1` = `[u8; 1]`.
    B1,
    /// `C1` = single byte read as a `char`.
    C1,
    /// `Cn` length-prefixed string.
    Cn,
    /// `KxN1` nibble-packed array (`fn(raw, pos, k)`).
    KxN1,
    /// `Kx*` array read with `fn(raw, pos, order, k)`: `KxR4`, `KxU2`.
    KxOrder { read: Ident, elem: usize },
}

struct FieldInfo {
    ident: Ident,
    inner_ty: Type,
    optional: bool,
    kind: Kind,
    count: Option<Ident>,
    /// The `smart_default` sentinel (`#[default = ..]` / `#[default(..)]`), used
    /// as the view getter's fallback when the field is absent from a short buffer.
    default_expr: Option<TokenStream2>,
}

fn expand(input: DeriveInput) -> syn::Result<TokenStream2> {
    let name = input.ident.clone();

    // struct-level `#[stdf(view = Ident)]`
    let mut view_name: Option<Ident> = None;
    for attr in &input.attrs {
        if attr.path().is_ident("stdf") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("view") {
                    view_name = Some(meta.value()?.parse()?);
                    Ok(())
                } else {
                    Err(meta.error("unknown `stdf` struct attribute (expected `view`)"))
                }
            })?;
        }
    }
    let view = view_name.unwrap_or_else(|| format_ident!("{}View", name));

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

        // field-level `#[stdf(count = ident)]`
        let mut count: Option<Ident> = None;
        for attr in &f.attrs {
            if attr.path().is_ident("stdf") {
                attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("count") {
                        count = Some(meta.value()?.parse()?);
                        Ok(())
                    } else {
                        Err(meta.error("unknown `stdf` field attribute (expected `count`)"))
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

        if matches!(kind, Kind::KxN1 | Kind::KxOrder { .. }) && count.is_none() {
            return Err(syn::Error::new_spanned(
                &f.ty,
                "array field requires `#[stdf(count = <field>)]`",
            ));
        }

        infos.push(FieldInfo {
            ident,
            inner_ty,
            optional,
            kind,
            count,
            default_expr: field_default(f),
        });
    }

    let view_fields = infos.iter().map(|fi| {
        let id = &fi.ident;
        quote! { #id: u16, }
    });
    let view_field_idents = infos.iter().map(|fi| fi.ident.clone()).collect::<Vec<_>>();
    let scan_stmts = infos.iter().map(gen_scan);
    let eager_stmts = infos.iter().map(gen_eager);
    let getters = infos.iter().map(gen_getter);

    let view_doc = format!("Zero-copy view over a raw `{name}` record.");

    Ok(quote! {
        impl #name {
            #[inline(always)]
            pub fn new() -> Self {
                Self::default()
            }

            #[inline(always)]
            pub fn read_from_bytes(&mut self, raw_data: &[u8], order: &ByteOrder) {
                let pos = &mut 0usize;
                #(#eager_stmts)*
            }
        }

        #[doc = #view_doc]
        pub struct #view<'a> {
            raw: &'a [u8],
            order: ByteOrder,
            #(#view_fields)*
        }

        impl<'a> #view<'a> {
            /// Scan the raw record once, recording each field's byte offset.
            #[inline]
            pub fn new(raw: &'a [u8], order: ByteOrder) -> Self {
                let pos = &mut 0usize;
                #(#scan_stmts)*
                #view { raw, order, #(#view_field_idents),* }
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
        "U1" => Kind::ScalarNoOrder { read: format_ident!("read_uint8"), bytes: 1 },
        "I1" => Kind::ScalarNoOrder { read: format_ident!("read_i1"), bytes: 1 },
        "U2" => Kind::ScalarOrder { read: format_ident!("read_u2"), bytes: 2, float: false },
        "I2" => Kind::ScalarOrder { read: format_ident!("read_i2"), bytes: 2, float: false },
        "U4" => Kind::ScalarOrder { read: format_ident!("read_u4"), bytes: 4, float: false },
        "I4" => Kind::ScalarOrder { read: format_ident!("read_i4"), bytes: 4, float: false },
        "U8" => Kind::ScalarOrder { read: format_ident!("read_u8"), bytes: 8, float: false },
        "R4" => Kind::ScalarOrder { read: format_ident!("read_r4"), bytes: 4, float: true },
        "R8" => Kind::ScalarOrder { read: format_ident!("read_r8"), bytes: 8, float: true },
        "B1" => Kind::B1,
        "C1" => Kind::C1,
        "Cn" => Kind::Cn,
        "KxN1" => Kind::KxN1,
        "KxR4" => Kind::KxOrder { read: format_ident!("read_kx_r4"), elem: 4 },
        "KxU2" => Kind::KxOrder { read: format_ident!("read_kx_u2"), elem: 2 },
        _ => return None,
    })
}

/// One `let <field>: u16 = ...;` offset-recording statement for `View::new`.
fn gen_scan(fi: &FieldInfo) -> TokenStream2 {
    let id = &fi.ident;
    match &fi.kind {
        Kind::ScalarNoOrder { bytes, .. } => scan_fixed(id, *bytes),
        Kind::ScalarOrder { bytes, .. } => scan_fixed(id, *bytes),
        Kind::B1 => scan_fixed(id, 1),
        Kind::C1 => scan_fixed(id, 1),
        Kind::Cn => quote! {
            let #id: u16 = if *pos < raw.len() {
                let off = *pos as u16;
                *pos += 1 + raw[*pos] as usize;
                off
            } else {
                STDF_VIEW_ABSENT
            };
        },
        Kind::KxN1 => {
            let c = fi.count.as_ref().unwrap();
            quote! {
                let #id: u16 = if *pos < raw.len() {
                    let off = *pos as u16;
                    let __k = stdf_view_opt(#c)
                        .map(|mut cp| read_u2(raw, &mut cp, &order))
                        .unwrap_or(0);
                    *pos += (__k / 2 + __k % 2) as usize;
                    off
                } else {
                    STDF_VIEW_ABSENT
                };
            }
        }
        Kind::KxOrder { elem, .. } => {
            let c = fi.count.as_ref().unwrap();
            let elem = *elem;
            quote! {
                let #id: u16 = if *pos < raw.len() {
                    let off = *pos as u16;
                    let __k = stdf_view_opt(#c)
                        .map(|mut cp| read_u2(raw, &mut cp, &order))
                        .unwrap_or(0);
                    *pos += (#elem as usize) * __k as usize;
                    off
                } else {
                    STDF_VIEW_ABSENT
                };
            }
        }
    }
}

fn scan_fixed(id: &Ident, bytes: usize) -> TokenStream2 {
    quote! {
        let #id: u16 = if *pos < raw.len() {
            let off = *pos as u16;
            *pos += #bytes;
            off
        } else {
            STDF_VIEW_ABSENT
        };
    }
}

/// One statement of the eager `read_from_bytes` body (writes `self.<field>`).
///
/// Optionality mirrors the historical `read_optional!` semantics: readers that
/// take no byte order (`B1`, `U1`, `I1`, `Cn`) `return` on truncation; readers
/// that take a byte order (and `Kx*`) set `None` and continue.
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
        (Kind::KxN1, false) => {
            let c = fi.count.as_ref().unwrap();
            quote! { self.#id = read_kx_n1(raw_data, pos, self.#c); }
        }
        (Kind::KxN1, true) => {
            syn::Error::new_spanned(id, "optional KxN1 is not supported").to_compile_error()
        }
        (Kind::KxOrder { read, .. }, false) => {
            let c = fi.count.as_ref().unwrap();
            quote! { self.#id = #read(raw_data, pos, order, self.#c); }
        }
        (Kind::KxOrder { read, elem }, true) => {
            let c = fi.count.as_ref().unwrap();
            let elem = *elem;
            quote! {
                if *pos + (#elem as usize) * self.#c as usize > raw_data.len() {
                    self.#id = None;
                } else {
                    self.#id = Some(#read(raw_data, pos, order, self.#c));
                }
            }
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
                    stdf_view_opt(self.#id).map(|mut p| #read(self.raw, &mut p)).unwrap_or(#dflt)
                }
            }
        }
        (Kind::ScalarNoOrder { read, .. }, true) => quote! {
            #[inline]
            pub fn #id(&self) -> Option<#ity> {
                stdf_view_opt(self.#id).map(|mut p| #read(self.raw, &mut p))
            }
        },
        (Kind::ScalarOrder { read, float, .. }, false) => {
            let dflt = fi
                .default_expr
                .clone()
                .unwrap_or_else(|| if *float { quote!(0.0) } else { quote!(0) });
            quote! {
                #[inline]
                pub fn #id(&self) -> #ity {
                    stdf_view_opt(self.#id)
                        .map(|mut p| #read(self.raw, &mut p, &self.order))
                        .unwrap_or(#dflt)
                }
            }
        }
        (Kind::ScalarOrder { read, .. }, true) => quote! {
            #[inline]
            pub fn #id(&self) -> Option<#ity> {
                stdf_view_opt(self.#id).map(|mut p| #read(self.raw, &mut p, &self.order))
            }
        },
        (Kind::B1, false) => quote! {
            #[inline]
            pub fn #id(&self) -> B1 {
                stdf_view_opt(self.#id)
                    .map(|p| [self.raw.get(p).copied().unwrap_or(0)])
                    .unwrap_or([0])
            }
        },
        (Kind::B1, true) => quote! {
            #[inline]
            pub fn #id(&self) -> Option<B1> {
                stdf_view_opt(self.#id).map(|p| [self.raw.get(p).copied().unwrap_or(0)])
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
        (Kind::C1, false) => {
            let dflt = fi.default_expr.clone().unwrap_or_else(|| quote!('\u{0}'));
            quote! {
                #[inline]
                pub fn #id(&self) -> C1 {
                    stdf_view_opt(self.#id)
                        .map(|p| self.raw.get(p).copied().unwrap_or(0) as char)
                        .unwrap_or(#dflt)
                }
            }
        }
        (Kind::C1, true) => quote! {
            #[inline]
            pub fn #id(&self) -> Option<C1> {
                stdf_view_opt(self.#id).map(|p| self.raw.get(p).copied().unwrap_or(0) as char)
            }
        },
        (Kind::KxN1, false) => {
            let c = fi.count.as_ref().unwrap();
            quote! {
                #[inline]
                pub fn #id(&self) -> #ity {
                    stdf_view_opt(self.#id)
                        .map(|mut p| read_kx_n1(self.raw, &mut p, self.#c()))
                        .unwrap_or_default()
                }
            }
        }
        (Kind::KxN1, true) => {
            syn::Error::new_spanned(id, "optional KxN1 is not supported").to_compile_error()
        }
        (Kind::KxOrder { read, .. }, false) => {
            let c = fi.count.as_ref().unwrap();
            quote! {
                #[inline]
                pub fn #id(&self) -> #ity {
                    stdf_view_opt(self.#id)
                        .map(|mut p| #read(self.raw, &mut p, &self.order, self.#c()))
                        .unwrap_or_default()
                }
            }
        }
        (Kind::KxOrder { read, .. }, true) => {
            let c = fi.count.as_ref().unwrap();
            quote! {
                #[inline]
                pub fn #id(&self) -> Option<#ity> {
                    stdf_view_opt(self.#id)
                        .map(|mut p| #read(self.raw, &mut p, &self.order, self.#c()))
                }
            }
        }
    }
}
