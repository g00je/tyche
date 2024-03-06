use crate::parser::{MemberType, Model};
use proc_macro2::TokenStream;
use quote::quote;
use quote_into::quote_into;

pub fn mtc(model: &Model) -> TokenStream {
    let mtc_fields = model.members.iter().map(|m| {
        let ident = &m.ident;

        if m.private {
            return match m.ty {
                MemberType::Number {..} => quote!(#ident: 0,),
                MemberType::Bytes {len} => quote!(#ident: [0; #len],),
                _ => panic!("invalid private field type. only numbers and bytes are accepted")
            };
        }

        let mut s = TokenStream::new();
        match &m.ty {
            MemberType::Bytes { .. } => quote_into!{s += #ident: value.#ident,},
            MemberType::Number { .. } => quote_into!{s+= #ident: value.#ident,},
            MemberType::Ipv4 => quote_into!{s += #ident: value.#ident, },
            MemberType::String { len, .. } => {
                let gen = |idx: TokenStream| {
                    quote! {{
                        let mut data = value.#ident #idx .as_bytes().to_vec();
                        data.resize(#len, 0);
                        data.as_slice().try_into().unwrap()
                    }}
                };
                quote_into! {s +=
                    #ident: #(crate::utils::array_index(&m.arr, &gen)),
                }
            },
            MemberType::Model { optional, cty, .. } => {
                let g1 = |idx: TokenStream| {
                    quote! {
                        if let Some(v) = &value.#ident #idx {
                            v.try_borrow(py)?.clone().try_into()?
                        } else {
                            <#cty>::default()
                        },
                    }
                };
                let g2 = |idx: TokenStream| {
                    quote! {
                        value.#ident #idx.try_borrow(py)?.clone().try_into()?,
                    }
                };

                let output = if *optional {
                    crate::utils::array_index(&m.arr, &g1)
                } else {
                    crate::utils::array_index(&m.arr, &g2)
                };
                quote_into!{s += #ident: #output}
            }
            MemberType::Flag { .. } => (),
        }

        s
    });

    let mtc_tokens = if model.has_models {
        quote! {
            ::pyo3::Python::with_gil(|py| {
                Ok(Self {
                    #(#mtc_fields)*
                })
            })
        }
    } else {
        quote! {
            Ok(Self {
                #(#mtc_fields)*
            })
        }
    };

    mtc_tokens
}
