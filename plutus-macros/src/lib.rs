use std::io::Write;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

mod parser;
mod mtc;
mod ctm;
mod getset;
mod default;
mod c_model;
mod dict;
use parser::{MemberType, Model};

#[proc_macro_attribute]
pub fn model(_args: TokenStream, code: TokenStream) -> TokenStream {
    let item = parse_macro_input!(code as ItemStruct);

    let Model { ident, c_ident, members, has_models } = parser::parse(item);
    let verr = quote!(::pyo3::exceptions::PyValueError::new_err);

    // let string_from_utf8 = quote! {
    //     ::std::string::String::from_utf8(value.clone()).unwrap_or_else(|e| {
    //         ::std::string::String::from_utf8(value[..e.utf8_error().valid_up_to()].into())
    //         .unwrap_or(::std::string::String::new())
    //     })
    // };

    let fields = members.iter().map(|m| {
        if m.private { return None }

        fn arr(ty: &MemberType) -> TokenStream2 {
            match ty {
                MemberType::Array { ty, len } => {
                    let ty = arr(&ty);
                    quote!( [#ty; #len] )
                }
                MemberType::Number { ty, .. } => quote! { #ty },
                MemberType::Bytes { len } => quote!( [u8; #len] ),
                MemberType::String { len, .. } => quote!( [u8; #len] ),
                MemberType::Model { ty, .. } => quote!( ::pyo3::Py<#ty> ),
                MemberType::Flag { .. } => quote!(),
            }
        }

        let ident = &m.ident;
        Some(match &m.ty {
            MemberType::Number { ty, .. } => quote! {
                #[pyo3(get, set)]
                #ident:#ty,
            },
            MemberType::Array { ty, len } => {
                let ty = arr(ty);
                quote! {
                    #[pyo3(get, set)]
                    #ident: [#ty; #len],
                }
            }
            MemberType::Bytes { len } => quote! { #ident: [u8; #len], },
            MemberType::String { .. } => quote! {
                #[pyo3(get)]
                #ident: String,
            },
            MemberType::Model { ty, .. } => quote! {
                #[pyo3(get, set)]
                #ident: ::pyo3::Py<#ty>,
            },
            MemberType::Flag { .. } => quote!(),
        })
    });

    let dict_method = dict::dict_method(&members);
    let default_tokens = default::default(has_models, &members);
    let getsets = getset::getset(&members);
    let c_struct = c_model::c_model(&c_ident, &members);
    let mtc_tokens = mtc::mtc(has_models, &members);
    let ctm_tokens = ctm::ctm(has_models, &members);

    let output = quote! {
        #c_struct

        impl ::core::convert::TryFrom<#ident> for #c_ident {
            type Error = ::pyo3::PyErr;

            fn try_from(value: #ident) -> Result<Self, Self::Error> {
                #mtc_tokens
            }
        }

        #[::pyo3::pyclass]
        #[derive(Clone, Debug)]
        struct #ident {
            #(#fields)*
        }

        impl ::core::convert::TryFrom<#c_ident> for #ident {
            type Error = ::pyo3::PyErr;

            fn try_from(value: #c_ident) -> Result<Self, Self::Error> {
                #ctm_tokens
            }
        }

        impl ::core::convert::TryFrom<&[u8]> for #ident {
            type Error = ::pyo3::PyErr;

            fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
                let value: Result<#c_ident, _> = value.try_into();
                match value {
                    Err(_) => Err(#verr("invalid value to convert")),
                    Ok(value) => Ok(value.try_into()?)
                }
            }
        }

        impl #ident {
            fn default() -> ::pyo3::PyResult<Self> {
                #default_tokens
            }
        }

        #[::pyo3::pymethods]
        impl #ident {
            #[classattr]
            const SIZE: u64 = <#c_ident>::SIZE as u64;

            #[new]
            fn py_new(value: Option<&::pyo3::PyAny>) -> ::pyo3::PyResult<Self> {
                match value {
                    Some(value) => {
                        if let Ok(m) = value.extract::<#ident>() {
                            return Ok(m);
                        }

                        let result: Result<Vec<u8>, _> = value.extract::<Vec<u8>>().or_else(|_| {
                            match value.extract::<String>() {
                                Err(e) => Err(e),
                                Ok(v) => {
                                    if v.len() != <#c_ident>::SIZE * 2 {
                                        return Err(#verr("invalid length"));
                                    }

                                    let v: Result<
                                            Vec<u8>, ::core::num::ParseIntError
                                        > = (0..v.len()).step_by(2)
                                            .map(|i| u8::from_str_radix(&v[i..i + 2], 16))
                                            .collect();

                                    match v {
                                        Err(_) => Err(#verr("invalid hex")),
                                        Ok(v) => Ok(v)
                                    }
                                }
                            }
                        });


                        if let Ok(data) = result {
                            let data = data.as_slice();

                            if data.len() != <#c_ident>::SIZE {
                                return Err(#verr("invalid input length"));
                            }

                            let m: #ident = data.try_into()?;
                            return Ok(m);
                        }

                        Ok(Self::default()?)
                    }
                    None => Ok(Self::default()?),
                }
            }

            fn __repr__(&self) -> String {
                format!("{:#?}", self)
            }

            fn __bytes__(&self) -> ::pyo3::PyResult<::std::borrow::Cow<[u8]>> {
                let data: &[u8] = <#c_ident>::try_from(self.clone())?.into();
                Ok(data.to_owned().into())
            }

            fn __eq__(&self, other: &Self) -> ::pyo3::PyResult<bool> {
                let a: &[u8] = <#c_ident>::try_from(self.clone())?.into();
                let b: &[u8] = <#c_ident>::try_from(other.clone())?.into();
                Ok(a == b)
            }

            fn hex(&self) -> ::pyo3::PyResult<String> {
                let data: &[u8] = <#c_ident>::try_from(self.clone())?.into();
                Ok(data.iter().map(|x| format!("{x:02x}")).collect())
            }

            #[classmethod]
            fn batch(cls: &::pyo3::types::PyType, value: &::pyo3::PyAny) -> ::pyo3::PyResult<Vec<Self>> {
                let result: Result<Vec<u8>, _> = value.extract::<Vec<u8>>().or_else(|_| {
                    match value.extract::<String>() {
                        Err(e) => Err(e),
                        Ok(v) => {
                            if v.len() != <#c_ident>::SIZE * 2 {
                                return Err(#verr("invalid hex length"));
                            }

                            let v: Result<
                                    Vec<u8>, ::core::num::ParseIntError
                                > = (0..v.len()).step_by(2)
                                    .map(|i| u8::from_str_radix(&v[i..i + 2], 16))
                                    .collect();

                            match v {
                                Err(_) => Err(#verr("invalid hex")),
                                Ok(v) => Ok(v)
                            }
                        }
                    }
                });

                if let Ok(data) = result {
                    let data = data.as_slice();

                    if data.len() % <#c_ident>::SIZE != 0 {
                        return Err(#verr(
                            "invalid input length"
                        ));
                    }

                    let total = data.len() / <#c_ident>::SIZE;
                    let mut result: Vec<#ident> = Vec::with_capacity(total);
                    for chunk in data.chunks(<#c_ident>::SIZE) {
                        result.push(chunk.try_into()?);
                    }

                    return Ok(result)
                }

                Err(#verr("invalid data"))
            }

            #dict_method

            #getsets
        }
    };

    // println!("\n\n{output}\n\n");
    let mut p = ::std::process::Command::new("rustfmt")
        .stdin(::std::process::Stdio::piped())
        // .stdout(::std::process::Stdio::piped())
        // .stderr(::std::process::Stdio::piped())
        .spawn()
        .unwrap();
    let mut stdin = p.stdin.take().unwrap();
    stdin.write_all(output.to_string().as_bytes()).unwrap();

    // let mut buf = String::new();
    // stdout.read_to_string(&mut buf).unwrap();

    // let mut fd = ::std::fs::File::options()
    //     .create(true)
    //     .append(true)
    //     .open("src/xx.rs")
    //     .unwrap();
    // fd.seek(::std::io::SeekFrom::End(0)).unwrap();
    // fd.write(b"// ------------------\n\n").unwrap();
    // fd.write_all(output.to_string().as_bytes()).unwrap();

    output.into()
}
