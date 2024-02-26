use crate::parser::{Member, MemberType};
use proc_macro2::{TokenStream, Ident};
use quote::quote;

pub fn dict_method(members: &Vec<Member>) -> TokenStream {
    let dict_fields = members.iter().map(|m| {
        if m.private {return None}
        let key = m.ident.to_string();
        let ident = &m.ident;

        fn arr(ident: &Ident, ty: &MemberType, lvl: &Vec<usize>) -> Option<TokenStream> {
            let index = lvl.iter().map(|x| quote!([#x])).collect::<Vec<_>>();
            match ty {
                MemberType::Array { ty, len  } => {
                    let x = (0..*len).map(|f| {
                        let mut x = lvl.clone();
                        x.push(f);
                        arr(ident, ty, &x)
                    }).collect::<Vec<_>>();

                    Some(quote! {
                        [  #(#x)* ],
                    })
                },
                MemberType::Bytes { .. } => Some(quote!{
                    self.#ident #(#index)* .iter().map(|x| format!("{x:02x}")).collect::<String>(),
                }),
                MemberType::Number { .. } => Some(quote! {
                    self.#ident #(#index)*,
                }),
                MemberType::Model { .. } => Some(quote! {
                    self.#ident #(#index)* .try_borrow(py)?.dict()?,
                }),
                MemberType::String { .. } => Some(quote!{
                    self.#ident #(#index)* .clone(),
                }),
                MemberType::Flag { .. } => None
            }
        }

        match &m.ty {
            MemberType::Array { ty, len } => {
                let x = (0..*len).map(|f| {
                    let v = vec![f];
                    arr(ident, ty, &v)
                }).collect::<Vec<_>>();

                Some(quote!{
                    dict.set_item( #key, [ #(#x)* ] )?; 
                })
            },
            MemberType::Bytes { .. } => Some(quote!{
                dict.set_item(
                    #key, 
                    self.#ident.iter().map(|x| format!("{x:02x}")).collect::<String>()
                )?; 
            }),
            MemberType::Number { .. } => Some(quote! {
                dict.set_item(#key, self.#ident)?; 
            }),
            MemberType::Model {..} => Some(quote! {
                dict.set_item(#key, self.#ident.try_borrow(py)?.dict()?)?; 
            }),
            MemberType::String {..} => Some(quote!{
                dict.set_item(#key, self.#ident.clone())?; 
            }),
            MemberType::Flag {..} => None,
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
