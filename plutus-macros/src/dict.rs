use crate::parser::{Model, MemberType};
use proc_macro2::{TokenStream, Ident};
use quote::quote;

pub fn dict_method(model: &Model) -> TokenStream {
    let c_ident = &model.c_ident;

    if model.hexable {
        return quote! {
            fn dict(&self) -> ::pyo3::PyResult<::std::string::String> {
                let data: Vec<u8> = <#c_ident>::try_from(self.clone())?.into();
                Ok(
                    data.iter().map(|x| format!("{x:02x}"))
                        .collect::<::std::string::String>()
                )
            }
        };
    }


    let dict_fields = model.members.iter().map(|m| {
        if m.private {return None}

        

        // fn arr(ident: &Ident, ty: &MemberType, lvl: &Vec<usize>) -> Option<TokenStream> {
        //     let index = lvl.iter().map(|x| quote!([#x])).collect::<Vec<_>>();
        //     match ty {
        //         MemberType::Array { ty, len  } => {
        //             let x = (0..*len).map(|f| {
        //                 let mut x = lvl.clone();
        //                 x.push(f);
        //                 arr(ident, ty, &x)
        //             }).collect::<Vec<_>>();
        //
        //             Some(quote! {
        //                 [  #(#x)* ],
        //             })
        //         },
        //         MemberType::Bytes { .. } => Some(quote!{
        //             self.#ident #(#index)* .iter().map(|x| format!("{x:02x}")).collect::<String>(),
        //         }),
        //         MemberType::Ipv4 => Some(quote!{{
        //             let value = self.#ident #(#index)*;
        //             value.iter().enumerate().map(|(i, x)| {
        //                 let x = x.to_string();
        //                 if i < value.len()-1 {x + "."} else {x}
        //             }).collect::<String>()
        //         }}),
        //         MemberType::Number { .. } => Some(quote! {
        //             self.#ident #(#index)*,
        //         }),
        //         MemberType::Model { optional, .. } => {
        //             if *optional {
        //                 Some(quote! {
        //                     if let Some(v) = & self.#ident #(#index)* {
        //                         Some(v.try_borrow(py)?.dict()?)
        //                     } else {None},
        //                     //self.#ident #(#index)* .try_borrow(py)?.dict()?,
        //                 })
        //             } else {
        //                 Some(quote! {
        //                     self.#ident #(#index)* .try_borrow(py)?.dict()?,
        //                 })
        //             }},
        //         MemberType::String { .. } => Some(quote!{
        //             self.#ident #(#index)* .clone(),
        //         }),
        //         MemberType::Flag { .. } => None
        //     }
        // }

        let key = m.ident.to_string();
        let ident = &m.ident;

        match &m.ty {
            // MemberType::Array { ty, len } => {
            //     let x = (0..*len).map(|f| {
            //         let v = vec![f];
            //         arr(ident, ty, &v)
            //     }).collect::<Vec<_>>();
            //
            //     Some(quote!{
            //         dict.set_item( #key, [ #(#x)* ] )?; 
            //     })
            // },
            MemberType::Ipv4 => Some(quote!{
                dict.set_item(
                    #key, 
                    self.#ident.iter().enumerate().map(|(i, x)| {
                        let x = x.to_string();
                        if i < self.#ident.len()-1 {x + "."} else {x}
                    }).collect::<String>()
                )?;
            }),
            MemberType::Bytes { .. } => Some(quote!{
                dict.set_item(
                    #key, 
                    self.#ident.iter().map(|x| format!("{x:02x}")).collect::<String>()
                )?;
            }),
            MemberType::Number { .. } => Some(quote! {
                dict.set_item(#key, self.#ident)?; 
            }),
            MemberType::Model {optional, ..} => {
                if *optional {
                    Some(quote! {
                        dict.set_item(#key, 
                            if let Some(v) = & self.#ident{
                                Some(v.try_borrow(py)?.dict()?)
                            } else {None}
                        )?; 
                    })
                } else {
                    Some(quote! {
                        dict.set_item(#key, self.#ident.try_borrow(py)?.dict()?)?; 
                    })
                }
            },
            MemberType::String {..} => Some(quote!{
                dict.set_item(#key, self.#ident.clone())?; 
            }),
            MemberType::Flag { fl } => Some(quote!{
                dict.set_item(#key, (self.flag & #fl) == #fl)?; 
            })
        }
    });

    quote! {
        fn dict(&self) -> ::pyo3::PyResult<::pyo3::Py<::pyo3::types::PyDict>> {
            ::pyo3::Python::with_gil(|py| {
                let dict = ::pyo3::types::PyDict::new(py);
                #(#dict_fields)*
                Ok(dict.into())
            })
        }
    }
}
