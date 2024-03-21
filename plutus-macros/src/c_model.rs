use crate::parser::{MemberType, Model};
use proc_macro2::TokenStream;
use quote::quote;
use quote_into::quote_into;

pub fn c_model(model: &Model) -> TokenStream {
    let c_ident = &model.c_ident;
    let c_fields = model.members.iter().map(|m| {
        let ident = &m.ident;

        let array = |ty: TokenStream| {
            let arr = match &m.arr {
                Some(a) => a,
                None => return ty,
            };

            arr.iter().rev().fold(ty, |a, i| quote!([#a; #i]))
        };

        let mut s = TokenStream::new();

        match &m.ty {
            MemberType::Number { ty, .. } => quote_into! {s +=
                #ident: #(array(quote!(#ty))),
            },
            MemberType::BigInt { len } => quote_into! {s +=
                #ident: #(array(quote!([u8; #len]))),
            },
            MemberType::Bytes { len } => quote_into! {s +=
                #ident: #(array(quote!([u8; #len]))),
            },
            MemberType::String { len, .. } => quote_into! {s +=
                #ident: #(array(quote!([u8; #len]))),
            },
            MemberType::Model { cty, .. } => quote_into! { s +=
                #ident: #(array(quote!(#cty))),
            },
            MemberType::Ipv4 => quote_into! {s +=
                #ident: #(array(quote!([u8; 4]))),
            },
            MemberType::Flag { .. } => (),
        }

        s
    });

    let default_tokens = default(model);

    let mut s = TokenStream::new();
    quote_into! {s +=
        #[repr(C)]
        #[derive(Debug)]
        struct #c_ident {
            #{for f in c_fields { quote_into!(s += #f) }}
        }

        impl #c_ident {
            const SIZE: usize = ::core::mem::size_of::<#c_ident>();
        }

        #default_tokens

        impl ::std::convert::From<&#c_ident> for Vec<u8> {
            fn from(value: &#c_ident) -> Self {
                unsafe {
                    ::core::slice::from_raw_parts(
                        value as *const #c_ident as *const u8,
                        <#c_ident>::SIZE
                    ).iter().map(|x| *x).collect::<Vec<u8>>()
                }
            }
        }

        impl ::std::convert::From<#c_ident> for Vec<u8> {
            fn from(value: #c_ident) -> Self {
                unsafe {
                    ::core::slice::from_raw_parts(
                        &value as *const #c_ident as *const u8,
                        <#c_ident>::SIZE
                    ).iter().map(|x| *x).collect::<Vec<u8>>()
                }
            }
        }

        impl ::std::convert::TryFrom<&[u8]> for #c_ident {
            type Error = ::pyo3::PyErr;

            fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
                unsafe {
                    let value: Result<[u8; <#c_ident>::SIZE], _> = value.try_into();
                    match value {
                        Err(_) => Err(::pyo3::exceptions::PyValueError::new_err("invalid input length")),
                        Ok(v) => Ok(::core::mem::transmute_copy(&v))
                    }
                }
            }
        }
    };

    s
}

fn default(model: &Model) -> TokenStream {
    let c_ident = &model.c_ident;
    let default_fields = model.members.iter().map(|m| {
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
        match &m.ty {
            // MemberType::Array { ty, len } => {
            //     let ty = arr(ty);
            //     let mut s = TokenStream::new();
            //     quote_into!(s += #ident: [
            //         #{(0..*len).for_each(|_| quote_into!(s += #ty,))}
            //     ],);
            //     s
            // }
            MemberType::BigInt { len } => quote_into! {s += 
                #ident: #(array(quote!([0; #len]))),
            },
            MemberType::Bytes { len } => quote_into! {s += 
                #ident: #(array(quote!([0; #len]))),
            },
            MemberType::Ipv4 => quote_into! { s +=
                #ident: #(array(quote!([0, 0, 0, 0]))),
            },
            MemberType::String { len, .. } => quote_into! { s += 
                #ident: #(array(quote!([0; #len]))),
            },
            MemberType::Model { cty, .. } => quote_into! { s +=
                #ident: #(array(quote!(<#cty>::default()))),
            },
            MemberType::Number { is_float, .. } => {
                let def = if *is_float {quote!(0.0)} else {quote!(0)};
                quote_into! { s += #ident: #(array(def)),}
            }
            MemberType::Flag { .. } => (),
        }

        s
    });

    let mut s = TokenStream::new();
    quote_into! { s +=
        impl Default for #c_ident {
            fn default() -> Self {
                Self {
                    #{default_fields.for_each(|i| quote_into!(s += #i))}
                }
            }
        }
    };

    s
}
