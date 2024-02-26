use crate::parser::{Member, MemberType};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub fn getset(members: &Vec<Member>) -> TokenStream {
    let gs = members.iter().map(|m| {
        let ident = &m.ident;
        match &m.ty {
            MemberType::Bytes { len } => {
                let get_ident = format_ident!("get_{}", ident);
                let set_ident = format_ident!("set_{}", ident);

                quote!{
                    #[getter]
                    fn #get_ident(&self) -> &[u8] {
                        &self.#ident
                    }

                    #[setter]
                    fn #set_ident(&mut self, value: &[u8]) -> ::pyo3::PyResult<()> {
                        if value.len() != #len {
                            return Err(::pyo3::exceptions::PyValueError::new_err(
                                format!("input length must be {}", #len)
                            ));
                        }

                        self.#ident = match value.try_into() {
                            Err(_) => return Err(
                                ::pyo3::exceptions::PyValueError::new_err(
                                    "invalid input"
                                )),
                            Ok(v) => v
                        };

                        Ok(())
                    }
                }
            },
            MemberType::String { len, validator } => {
                let mut validation: Option<TokenStream> = None;
                if let Some(v) = validator {
                    validation = Some(quote! {
                        let value = match #v(value) {
                            Ok(v) => v,
                            Err(e) => return Err(e)
                        };
                    });
                }

                quote! {
                    #[setter]
                    fn #ident(&mut self, mut value: String) -> ::pyo3::PyResult<()> {
                        let mut idx = #len;
                        loop {
                            if value.is_char_boundary(idx) {
                                break;
                            }
                            idx -= 1;
                        }
                        value.truncate(idx);

                        #validation

                        self.#ident = value;

                        // let mut value = value.as_bytes().to_vec();
                        // value.resize(#len, 0);
                        //
                        // self.#ident = #string_from_utf8;

                        Ok(())
                    }
                }
            },
            MemberType::Model { .. } => {
                quote! {
                    // #[getter]
                    // fn #ident(&self) -> &#ty {
                    //     &self.#ident
                    // }
                }
            },
            MemberType::Flag { fl } => {
                let get_ident = format_ident!("get_{}", ident);
                let set_ident = format_ident!("set_{}", ident);

                quote!{
                    #[getter]
                    fn #get_ident(&self) -> bool {
                        (self.flag & #fl) == #fl
                    }

                    #[setter]
                    fn #set_ident(&mut self, value: bool) -> ::pyo3::PyResult<()> {
                        if value {
                            self.flag |= #fl;
                        } else {
                            self.flag &= !#fl;
                        }
                        Ok(())
                    }
                }
            },
            _ => quote!(),
        }
    });
    
    quote!( #(#gs)* )
}
