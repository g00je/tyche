use crate::{
    parser::{MemberType, Model},
    utils::array_index,
};
use proc_macro2::TokenStream;
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

            fn __bool__(&self) -> ::pyo3::PyResult<bool> {
                let value = <#c_ident>::try_from(self.clone())?;
                Ok(!value.is_none())
            }
        };
    }

    let dict_fields = model.members.iter().map(|m| {
        if m.private {
            return None;
        }

        let key = m.ident.to_string();
        let ident = &m.ident;

        let v = match &m.ty {
            MemberType::Ipv4 => {
                let gen = |idx: TokenStream| {
                    quote! {
                        self.#ident #idx .iter().enumerate().map(|(i, x)| {
                            let x = x.to_string();
                            if i < self.#ident.len()-1 {x + "."} else {x}
                        }).collect::<String>()
                    }
                };

                array_index(&m.arr, &gen)
            }
            MemberType::BigInt { .. } => {
                let gen = |idx: TokenStream| {
                    quote! {
                        ::num_bigint::BigUint::from_bytes_le(&self.#ident #idx).to_string()
                    }
                };

                array_index(&m.arr, &gen)
            }
            MemberType::Bytes { .. } => {
                let gen = |idx: TokenStream| {
                    quote! {
                        self.#ident #idx .iter()
                            .map(|x| format!("{x:02x}"))
                            .collect::<String>()
                    }
                };

                array_index(&m.arr, &gen)
            }
            MemberType::Number { .. } => quote! {
                self.#ident
            },
            MemberType::Model { optional, .. } => {
                let gen_opt = |idx: TokenStream| {
                    quote! {
                        if let Some(v) = & self.#ident #idx {
                            Some(v.try_borrow(py)?.dict()?)
                        } else {
                            None
                        }
                    }
                };
                let gen = |idx: TokenStream| {
                    quote! {
                        self.#ident #idx .try_borrow(py)?.dict()?
                    }
                };
                if *optional {
                    array_index(&m.arr, &gen_opt)
                } else {
                    array_index(&m.arr, &gen)
                }
            }
            MemberType::String { .. } => {
                quote! { self.#ident.clone() }
            }
            MemberType::Flag { fl } => {
                quote! { (self.flag & #fl) == #fl }
            }
        };

        Some(quote! {
            dict.set_item(#key, #v)?;
        })
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
