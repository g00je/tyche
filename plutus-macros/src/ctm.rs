use crate::parser::{Model, MemberType};
use proc_macro2::{Ident, TokenStream};
use quote::quote;

pub fn ctm(model: &Model) -> TokenStream {
    let ctm_fields = model.members.iter().map(|m| {
        if m.private {return None}
        fn arr(ident: &Ident, ty: &MemberType, lvl: &Vec<usize>) -> Option<TokenStream> {
            let index = lvl.iter().map(|x| quote!([#x])).collect::<Vec<_>>();
            match ty {
                MemberType::Array { ty, len } => {
                    let x = (0..*len).map(|f| {
                        let mut x = lvl.clone();
                        x.push(f);
                        arr(ident, ty, &x)
                    }).collect::<Vec<_>>();

                    Some(quote! {
                        [  #(#x)* ],
                    })


                //     match arr(&ty) {
                //     Some((a, b, c)) => Some((
                //         quote!{ x.map(|x| #a ) },
                //         quote!{ x.iter().find_map(|x| #b ) },
                //         quote!{ x.map(|x| #c ) },
                //     )),
                //     None => None,
                // }
                },
                MemberType::Model { optional, ty, .. } => {
                    if *optional {
                        Some(quote! {{
                            let v = &value.#ident #(#index)*;
                            if v.is_none() {None} else {
                            Some(::pyo3::Py::new(py, <#ty>::try_from(v)?)?)
                            }
                        },})
                    } else {
                        Some(quote! {
                            ::pyo3::Py::new(py, <#ty>::try_from(&(
                                value.#ident #(#index)*
                            ))?)?,
                        })
                    }
                },
                // MemberType::Model { ty, optional, .. } => {
                //     if *optional {
                //         Some((
                //             quote!{ 
                //                 if x.is_none() {None} else{
                //                 Some(::pyo3::Py::new(py, <#ty>::try_from(x)?))
                //                 }
                //             },
                //             quote!{ if x.is_err() {Some(())} else {None} },
                //             quote!{x.unwrap()}
                //         ))
                //     } else {
                //         Some((
                //             quote!{ ::pyo3::Py::new(py, <#ty>::try_from(x)?) },
                //             quote!{ if x.is_err() {Some(())} else {None} },
                //             quote!{x.unwrap()}
                //         ))
                //     }
                // },
                _ => None,
            }
        }

        let ident = &m.ident;
        Some(match &m.ty {
            MemberType::Array { ty, len } => {
                let x = (0..*len).map(|f| {
                    let v = vec![f];
                    arr(ident, ty, &v)
                }).collect::<Vec<_>>();

                quote!{
                    #ident: [ #(#x)* ],
                }

            //     match arr(ty) {
            //     Some((a, b, c)) => quote! { #ident: {
            //         let x = value.#ident.map(|x| #a );
            //         if let Some(_) = x.iter().find_map(|x| #b ) {
            //             return Err(::pyo3::exceptions::PyValueError::new_err(
            //                 "could not convert the value"
            //             ));
            //         }
            //         x.map(|x| #c )
            //     }, },
            //     None => quote! { #ident: value.#ident, },
            // }
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
            MemberType::Model { ty, optional, .. } => {
                if *optional {
                    quote!{
                        #ident : {
                            if value.#ident.is_none() {None} else {
                                Some(::pyo3::Py::new(py, <#ty>::try_from(&value.#ident)?)?)
                            }
                        },
                    }
                } else {
                    quote!{
                        #ident : ::pyo3::Py::new(py, <#ty>::try_from(&value.#ident)?)?,
                    }
                }
            }
            MemberType::Number { .. } => quote!(#ident: value.#ident, ),
            MemberType::Flag { .. } => quote!(),
        })
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
