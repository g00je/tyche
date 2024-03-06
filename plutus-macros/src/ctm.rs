use crate::parser::{Model, MemberType};
use proc_macro2::TokenStream;
use quote::quote;
use quote_into::quote_into;

pub fn ctm(model: &Model) -> TokenStream {
    let ctm_fields = model.members.iter().map(|m| {
        if m.private {return None}

        let ident = &m.ident;
        let mut s = TokenStream::new();
        match &m.ty {
            MemberType::Bytes { .. } => quote_into!(s += #ident: value.#ident, ),
            MemberType::Ipv4 => quote_into!(s += #ident: value.#ident,),
            MemberType::String { .. } => {
                let gen = |idx: TokenStream| {
                    quote! {
                        ::std::string::String::from_utf8(value.#ident #idx.to_vec())
                        .unwrap_or_else(|e| {
                            ::std::string::String::from_utf8(
                                value.#ident #idx [..e.utf8_error().valid_up_to()].into()
                            ).unwrap_or(::std::string::String::new())
                        })
                    }
                };

                quote_into! {s +=
                    // #ident: string_to_array(value.#ident, #len),
                    #ident: #(crate::utils::array_index(&m.arr, &gen)),
                }
            },
            MemberType::Model { ty, optional, .. } => {
                let g1 = |idx: TokenStream| {
                    quote! {
                        if value.#ident #idx .is_none() {None} else {
                            Some(::pyo3::Py::new(py, <#ty>::try_from(&value.#ident #idx )?)?)
                        },
                    }
                };
                let g2 = |idx: TokenStream| {
                    quote! {
                        ::pyo3::Py::new(py, <#ty>::try_from(&value.#ident #idx )?)?,
                    }
                };

                let output = if *optional {
                    crate::utils::array_index(&m.arr, &g1)
                } else {
                    crate::utils::array_index(&m.arr, &g2)
                };
                quote_into!{s += #ident: #output}
            }
            MemberType::Number { .. } => quote_into!{s += #ident: value.#ident, },
            MemberType::Flag { .. } => ()
        }

        Some(s)
    });

    let ctm_tokens = if model.has_models {
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
