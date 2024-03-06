use crate::parser::{Model, MemberType};
use proc_macro2::TokenStream;
use quote::quote;
use quote_into::quote_into;

pub fn default(model: &Model) -> TokenStream {
    let default_fields = model.members.iter().map(|m| {
        if m.private {return None;}

        let array = |ty: TokenStream| {
            let arr = match &m.arr {
                Some(a) => a,
                None => return ty,
            };

            arr.iter().rev().fold(ty, |a, i| {
                let mut s = TokenStream::new();
                quote_into!(s += [#{
                    for _ in 0..*i { quote_into!(s += #a,) }
                }]);
                s
            })
        };

        let ident = &m.ident;
        let mut s = TokenStream::new();

        let v = match &m.ty {
            MemberType::Bytes { len } => Some(quote!([0; #len])),
            MemberType::Ipv4 => Some(quote!([0, 0, 0, 0])),
            MemberType::String { .. } => Some(quote!(String::default())),
            MemberType::Model { ty, optional, .. } => Some(
                if *optional {
                    quote!(None)
                } else {
                    quote!(::pyo3::Py::new(py, <#ty>::default()?)?)
                }
            ),
            MemberType::Number { is_float, .. } => Some(
                if *is_float { quote!(0.0) } else { quote!(0) }
            ),
            MemberType::Flag { .. } => None
        };

        if let Some(v) = v {
            quote_into!{s += #ident: #(array(v)),}
        }

        Some(s)
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
