use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Attribute, DeriveInput, LitStr};

fn find_name_attr(attrs: &[Attribute]) -> Option<LitStr> {
    // Accept: #[janet_abstract(name = "target/any-target")]
    for a in attrs {
        if !a.path().is_ident("janet_abstract") {
            continue;
        }
        let mut out: Option<LitStr> = None;
        let _ = a.parse_nested_meta(|meta| {
            if meta.path.is_ident("name") {
                let v: LitStr = meta.value()?.parse()?;
                out = Some(v);
            }
            Ok(())
        });
        if out.is_some() {
            return out;
        }
    }
    None
}

/// Derive macro:
///   #[derive(JanetAbstract)]
///   #[janet_abstract(name = "target/any-target")]
///   struct AnyTargetSelector;
#[proc_macro_derive(JanetAbstract, attributes(janet_abstract))]
pub fn derive_janet_abstract(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ty_ident = input.ident;
    let name_lit = find_name_attr(&input.attrs).unwrap_or_else(|| {
        // default: "module/TypeName" style, but you can change this
        LitStr::new(&format!("abstract/{}", ty_ident), ty_ident.span())
    });

    // static name: <TYPE>_ABSTRACT_TYPE
    let static_ident = format_ident!("{}_ABSTRACT_TYPE", ty_ident.to_string().to_uppercase());

    // Optional accessor fn name: <TypeName>::abstract_type()
    let expanded = quote! {
        #[allow(non_upper_case_globals)]
        pub static mut #static_ident: ::janetrs::types::JanetAbstractType =
            ::janetrs::types::JanetAbstractType::new(
                ::core::ffi::CStr::from_bytes_with_nul_unchecked(
                    ::core::concat!(#name_lit, "\0").as_bytes()
                ),
                ::janetrs::types::JanetAbstractType::gc::<#ty_ident>,
            );

        impl #ty_ident {
            #[inline]
            pub fn abstract_type() -> &'static mut ::janetrs::types::JanetAbstractType {
                unsafe { &mut #static_ident }
            }
        }
    };

    expanded.into()
}

