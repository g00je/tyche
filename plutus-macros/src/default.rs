use crate::parser::{Model, MemberType};
use proc_macro2::TokenStream;
use quote::quote;
use quote_into::quote_into;

pub fn default(model: &Model) -> TokenStream {
    let default_fields = model.members.iter().map(|m| {
        if m.private {
            return None;
        }

        fn arr(ty: &MemberType) -> TokenStream {
            match ty {
                MemberType::Array { ty, len } => {
                    let ty = arr(&ty);
                    let mut s = TokenStream::new();
                    quote_into!(s += [
                        #{(0..*len).for_each(|_| quote_into!(s += #ty,))}
                    ]);
                    s
                }
                MemberType::Number { is_float, .. } => {
                    if *is_float {
                        quote!(0.0)
                    } else {
                        quote!(0)
                    }
                }
                MemberType::Bytes { len } => quote!( [0; #len] ),
                MemberType::Ipv4 => quote!([0, 0, 0, 0]),
                MemberType::String { len, .. } => quote!( [0; #len] ),
                MemberType::Model { ty, optional, .. } => {
                    if *optional {
                        quote!(None)
                    } else {
                        quote!( ::pyo3::Py::new(py, <#ty>::default()?)? )
                    }
                }
                MemberType::Flag { .. } => quote!(),
            }
        }

        let ident = &m.ident;
        Some(match &m.ty {
            MemberType::Array { ty, len } => {
                let ty = arr(ty);
                let mut s = TokenStream::new();
                quote_into!(s += #ident: [
                    #{(0..*len).for_each(|_| quote_into!(s += #ty,))}
                ],);
                s
            }
            MemberType::Bytes { len } => quote!(#ident: [0; #len], ),
            MemberType::Ipv4 => quote!(#ident: [0, 0, 0, 0],),
            MemberType::String { .. } => quote!(#ident: String::default(), ),
            MemberType::Model { ty, optional, .. } => {
                if *optional {
                    quote!(#ident: None,)
                } else {
                    quote!(#ident: ::pyo3::Py::new(py, <#ty>::default()?)?, )
                }
            }
            MemberType::Number { is_float, .. } => {
                if *is_float {
                    quote!(#ident: 0.0,)
                } else {
                    quote!(#ident: 0,)
                }
            }
            MemberType::Flag { .. } => quote!(),
        })
    });

    let mut s = TokenStream::new();
    quote_into! { s += #{
        if model.has_models {
            quote_into!{s +=
                ::pyo3::Python::with_gil(|py| {
                    Ok(Self {
                        #{default_fields.for_each(|i| quote_into!(s += #i))}
                    })
                })
            }
        } else {
            quote_into!{s +=
                Ok(Self {
                    #{default_fields.for_each(|i| quote_into!(s += #i))}
                })
            }
        }
    }};

    s
}
