use crate::parser::{Model, MemberType};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use quote_into::quote_into;

pub fn getset(model: &Model) -> TokenStream {
    let gs = model.members.iter().map(|m| {
        if m.private{return None}
        let ident = &m.ident;

        Some(match &m.ty {
            MemberType::Number { min, max, ty, .. } => {
                let mut s = TokenStream::new();

                quote_into! {s +=
                    #[setter]
                    fn #ident(&mut self, value: #ty) -> ::pyo3::PyResult<()> {
                        #{if let Some(min) = min {
                            let err = format!("minimum value is {}", min);
                            quote_into!(s += 
                                if value < #min as #ty {
                        return Err(::pyo3::exceptions::PyValueError::new_err(
                            #err
                        ));
                                }
                            )
                        }}

                        #{if let Some(max) = max {
                            let err = format!("maximum value is {}", max);
                            quote_into!(s += 
                                if value > #max as #ty {
                        return Err(::pyo3::exceptions::PyValueError::new_err(
                            #err
                        ));
                                }
                            )
                        }}

                        self.#ident = value;
                        Ok(())
                    }
                };

                s
            },
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
            MemberType::Ipv4 => {
                let get_ident = format_ident!("get_{}", ident);
                let set_ident = format_ident!("set_{}", ident);

                quote!{
                    #[getter]
                    fn #get_ident(&self) -> String {
                        self.#ident.iter().enumerate().map(|(i, x)| {
                            let x = x.to_string();
                            if i < self.#ident.len()-1 {x + "."} else {x}
                        }).collect::<String>()
                    }

                    #[setter]
                    fn #set_ident(&mut self, value: String) -> ::pyo3::PyResult<()> {
                        let err = Err(::pyo3::exceptions::PyValueError::new_err(
                            "invalid input for ipv4"
                        ));
                        if value.len() > 16 {
                            return err;
                        }

                        let result = value.split('.').map(|c| {
                            c.parse::<u8>()
                        }).collect::<Result<Vec<_>,_>>();

                        self.#ident = match result {
                            Err(_) => return err,
                            Ok(v) => {
                                if v.len() != 4 {
                                    return err;
                                }

                                match v.try_into() {
                                    Ok(v) => v,
                                    Err(_) => return err
                                }
                            }
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
        })
    });

    quote!( #(#gs)* )
}
