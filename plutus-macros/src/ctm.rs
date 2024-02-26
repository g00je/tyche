use crate::parser::{Member, MemberType};
use proc_macro2::TokenStream;
use quote::quote;

pub fn ctm(has_models: bool, members: &Vec<Member>) -> TokenStream {
    let ctm_fields = members.iter().map(|m| {
        if m.private {return None}
        fn arr(ty: &MemberType) -> Option<(TokenStream, TokenStream, TokenStream)> {
            match ty {
                MemberType::Array { ty, .. } => match arr(&ty) {
                    Some((a, b, c)) => Some((
                        quote!{ x.map(|x| #a ) },
                        quote!{ x.iter().find_map(|x| #b ) },
                        quote!{ x.map(|x| #c ) },
                    )),
                    None => None,
                },
                MemberType::Model { ty, .. } => Some((
                    quote!{ ::pyo3::Py::new(py, <#ty>::try_from(x)?) },
                    quote!{ if x.is_err() {Some(())} else {None} },
                    quote!{x.unwrap()}
                )),
                _ => None,
            }
        }

        let ident = &m.ident;
        Some(match &m.ty {
            MemberType::Array { ty, .. } => match arr(ty) {
                Some((a, b, c)) => quote! { #ident: {
                    let x = value.#ident.map(|x| #a );
                    if let Some(_) = x.iter().find_map(|x| #b ) {
                        return Err(::pyo3::exceptions::PyValueError::new_err(
                            "could not convert the value"
                        ));
                    }
                    x.map(|x| #c )
                }, },
                None => quote! { #ident: value.#ident, },
            },
            MemberType::Bytes { .. } => quote!(#ident: value.#ident, ),
            MemberType::Ipv4 => quote!(#ident: value.#ident,),
            MemberType::String { .. } => quote! {
                // #ident: string_to_array(value.#ident, #len),
                #ident: ::std::string::String::from_utf8(value.#ident.to_vec())
                    .unwrap_or_else(|e| {
                        ::std::string::String::from_utf8(
                            value.#ident[..e.utf8_error().valid_up_to()].into()
                        ).unwrap_or(::std::string::String::new())
                    }),
            },
            MemberType::Model { ty, .. } => {
                quote!{
                    #ident : ::pyo3::Py::new(py, <#ty>::try_from(value.#ident)?)?,
                }
            }
            MemberType::Number { .. } => quote!(#ident: value.#ident, ),
            MemberType::Flag { .. } => quote!(),
        })
    });

    let ctm_tokens = if has_models {
        quote! {
            ::pyo3::Python::with_gil(|py| {
                Ok(Self {
                    #(#ctm_fields)*
                })
            })
        }
    } else {
        quote! {
            Ok(Self {
                #(#ctm_fields)*
            })
        }
    };

    ctm_tokens
}
