use crate::parser::{Member, MemberType};
use proc_macro2::TokenStream;
use quote::quote;

pub fn mtc(has_models: bool, members: &Vec<Member>) -> TokenStream {
    let mtc_fields = members.iter().map(|m| {
        let ident = &m.ident;

        if m.private {
            return match m.ty {
                MemberType::Number {..} => quote!(#ident: 0,),
                MemberType::Bytes {len} => quote!(#ident: [0; #len],),
                _ => panic!("invalid private field type. only numbers and bytes are accepted")
            };
        }

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
                MemberType::Model { .. } => Some((
                    quote!{x.try_borrow(py)?.clone().try_into()},
                    quote!{ if x.is_err() {Some(())} else {None} },
                    quote!{x.unwrap()}
                )),
                _ => None,
            }
        }
        match &m.ty {
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
            MemberType::Ipv4 => quote!(#ident: value.#ident, ),
            MemberType::String { len, .. } => quote! {
                // #ident: string_to_array(value.#ident, #len),
                #ident: {
                    let mut data = value.#ident.as_bytes().to_vec();
                    data.resize(#len, 0);
                    data.as_slice().try_into().unwrap()
                },
            },
            MemberType::Model { .. } => {
                quote!(#ident: value.#ident.try_borrow(py)?.clone().try_into()?, )
            }
            MemberType::Number { .. } => quote!(#ident: value.#ident, ),
            MemberType::Flag { .. } => quote!(),
        }
    });

    let mtc_tokens = if has_models {
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
