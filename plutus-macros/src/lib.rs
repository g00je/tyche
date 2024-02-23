use std::io::{Read, Seek, Write};

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{parse_macro_input, ItemStruct};

mod parser;
use parser::{MemberType, Model};

#[proc_macro_attribute]
pub fn model(_args: TokenStream, code: TokenStream) -> TokenStream {
    let item = parse_macro_input!(code as ItemStruct);

    let Model { ident, c_ident, members } = parser::parse(item);

    // let fields = match fields {
    //     syn::Fields::Named(fields) => fields,
    //     syn::Fields::Unit => FieldsNamed {
    //         brace_token: Brace::default(),
    //         named: Punctuated::new(),
    //     },
    //     _ => panic!("invalid struct fields"),
    // };
    //
    // let mut c_fields = fields.named.clone();

    // let string_from_utf8 = quote! {
    //     ::std::string::String::from_utf8(value.clone()).unwrap_or_else(|e| {
    //         ::std::string::String::from_utf8(value[..e.utf8_error().valid_up_to()].into())
    //         .unwrap_or(::std::string::String::new())
    //     })
    // };

    let c_fields = members.iter().map(|m| {
        fn arr(ty: &MemberType) -> TokenStream2 {
            match ty {
                MemberType::Array { ty, len } => {
                    let ty = arr(&ty);
                    quote!( [#ty; #len] )
                }
                MemberType::Number { ty, .. } => quote! { #ty },
                MemberType::Bytes { len } => quote!( [u8; #len] ),
                MemberType::String { len, .. } => quote!( [u8; #len] ),
                MemberType::Model { cty, .. } => quote!( #cty ),
            }
        }

        let ident = &m.ident;

        match &m.ty {
            MemberType::Number { ty, .. } => quote!(#ident : #ty,),
            MemberType::Array { ty, len } => {
                let ty = arr(ty);
                quote! { #ident: [#ty; #len], }
            }
            MemberType::Bytes { len } => quote!(#ident: [u8; #len], ),
            MemberType::String { len, .. } => quote!(#ident: [u8; #len], ),
            MemberType::Model { cty, .. } => quote!(#ident: #cty, ),
        }
    });

    let fields = members.iter().map(|m| {
        fn arr(ty: &MemberType) -> TokenStream2 {
            match ty {
                MemberType::Array { ty, len } => {
                    let ty = arr(&ty);
                    quote!( [#ty; #len] )
                }
                MemberType::Number { ty, .. } => quote! { #ty },
                MemberType::Bytes { len } => quote!( [u8; #len] ),
                MemberType::String { len, .. } => quote!( [u8; #len] ),
                MemberType::Model { ty, .. } => quote!( #ty ),
            }
        }

        let ident = &m.ident;
        match &m.ty {
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
                #ident: #ty,
            },
        }
    });

    let default_fields = members.iter().map(|m| {
        fn arr(ty: &MemberType) -> TokenStream2 {
            match ty {
                MemberType::Array { ty, len } => {
                    let ty = arr(&ty);
                    quote!( [#ty; #len] )
                }
                MemberType::Number { ty, .. } => quote! { <#ty>::default() },
                MemberType::Bytes { len } => quote!( [0; #len] ),
                MemberType::String { len, .. } => quote!( [0; #len] ),
                MemberType::Model { ty, .. } => quote!( <#ty>::default() ),
            }
        }

        let ident = &m.ident;
        match &m.ty {
            MemberType::Array { ty, len } => {
                let ty = arr(ty);
                quote! { #ident: [#ty; #len], }
            }
            MemberType::Bytes { len } => quote!(#ident: [0; #len], ),
            MemberType::String { .. } => quote!(#ident: String::default(), ),
            MemberType::Model { ty, .. } => quote!(#ident: <#ty>::default(), ),
            MemberType::Number { ty, .. } => quote!(#ident: <#ty>::default(), ),
        }
    });

    let mtc_fields = members.iter().map(|m| {
        fn arr(ty: &MemberType) -> Option<TokenStream2> {
            match ty {
                MemberType::Array { ty, .. } => {
                    match arr(&ty) {
                        Some(ty) => Some(quote!( #ty )),
                        None => None
                    }
                }
                MemberType::Model { ty, .. } => Some(quote!( .iter() )),
                _ => None
            }
        }
        let ident = &m.ident;
        match &m.ty {
            MemberType::Array { ty, .. } => {
                match arr(ty) {
                    Some(ty) => quote! { #ident: value #ty, },
                    None => quote! { #ident: value.#ident, }
                }
            }
            MemberType::Bytes { .. } => quote!(#ident: value.#ident, ),
            MemberType::String { len, .. } => quote! {
                // #ident: string_to_array(value.#ident, #len),
                #ident: {
                    let mut data = value.#ident.as_bytes().to_vec();
                    data.resize(#len, 0);
                    data.as_slice().try_into().unwrap()
                },
            },
            MemberType::Model { .. } => {
                quote!(#ident: value.#ident.into(), )
            }
            MemberType::Number { .. } => quote!(#ident: value.#ident, ),
        }
    });

    let ctm_fields = members.iter().map(|m| {
        let ident = &m.ident;
        fn arr(ty: &MemberType) -> Option<TokenStream2> {
            match ty {
                MemberType::Array { ty, len } => {
                    let ty = arr(&ty);
                    Some(quote!())
                },
                MemberType::Model { ty, .. } => {
                    Some(quote! {
                        // #ident: value.#ident.iter().map(|f| f.into()).collect::<Vec< #ty >>(),
                    })
                },
                _ => None
            }
        }
        match &m.ty {
            MemberType::Array { ty, .. } => {
                // let ty = arr(ty);
                quote! { #ident: value.#ident, }
                // quote! { #ident: #ty, }
            },
            MemberType::Bytes { .. } => quote!(#ident: value.#ident, ),
            MemberType::String { .. } => quote! {
                // #ident: string_to_array(value.#ident, #len),
                #ident: ::std::string::String::from_utf8(value.#ident.to_vec())
                    .unwrap_or_else(|e| {
                        ::std::string::String::from_utf8(
                            value.#ident[..e.utf8_error().valid_up_to()].into()
                        ).unwrap_or(::std::string::String::new())
                    }),
            },
            MemberType::Model { .. } => {
                quote!(#ident : value.#ident.into(),)
            }
            MemberType::Number { .. } => quote!(#ident: value.#ident, ),
        }
    });

    let get_sets = members.iter().map(|m| {
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
                let mut validation: Option<TokenStream2> = None;
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
            }
            _ => quote!(),
        }
    });

    let output = quote! {
        #[repr(C)]
        #[derive(Debug)]
        struct #c_ident {
            #(#c_fields)*
        }

        impl #c_ident {
            const SIZE: usize = ::core::mem::size_of::<#c_ident>();
        }

        impl ::std::convert::From<#c_ident> for &[u8] {
            fn from(value: #c_ident) -> Self {
                unsafe {
                    ::core::slice::from_raw_parts(
                        &value as *const #c_ident as *const u8,
                        <#c_ident>::SIZE
                    )
                }
            }
        }

        impl ::std::convert::TryFrom<&[u8]> for #c_ident {
            type Error = &'static str;

            fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
                unsafe {
                    let value: Result<[u8; <#c_ident>::SIZE], _> = value.try_into();
                    match value {
                        Err(_) => Err("invalid input length"),
                        Ok(v) => Ok(::core::mem::transmute_copy(&v))
                    }
                }
            }
        }

        impl ::core::convert::From<#ident> for #c_ident {
            fn from(value: #ident) -> Self {
                Self {
                    #(#mtc_fields)*
                }
            }
        }

        #[pyclass]
        #[derive(Clone, Debug)]
        struct #ident {
            #(#fields)*
        }

        impl ::core::convert::From<#c_ident> for #ident {
            fn from(value: #c_ident) -> Self {
                Self {
                    #(#ctm_fields)*
                }
            }
        }

        impl ::core::convert::TryFrom<&[u8]> for #ident {
            type Error = &'static str;

            fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
                let value: Result<#c_ident, _> = value.try_into();
                match value {
                    Err(e) => Err("invalid value to convert"),
                    Ok(value) => Ok(value.into())
                }
            }
        }

        impl ::core::convert::TryFrom<&str> for #ident {
            type Error = &'static str;

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                if value.len() != <#c_ident>::SIZE * 2 {
                    return Err("invalid value length.");
                }

                let value: Result<Vec<u8>, ::core::num::ParseIntError> = (0..value.len())
                    .step_by(2)
                    .map(|i| u8::from_str_radix(&value[i..i + 2], 16))
                    .collect();

                let value = match value {
                    Err(_) => return Err("invalid hex"),
                    Ok(v) => v
                };

                let value: Result<#c_ident, _> = value.as_slice().try_into();
                let value = match value {
                    Err(_) => return Err("invalid data"),
                    Ok(v) => v
                };

                Ok(value.into())
            }
        }

        impl Default for #ident {
            fn default() -> Self {
                Self {
                    #(#default_fields)*
                }
            }
        }

        #[pymethods]
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

                        if let Ok(data) = value.extract::<&[u8]>() {
                            let m: Result<#ident, _> = data.try_into();
                            return match m {
                                Ok(m) => Ok(m),
                                Err(e) => Err(::pyo3::exceptions::PyValueError::new_err(e))
                            };
                        }

                        if let Ok(data) = value.extract::<String>() {
                            let m: Result<#ident, _> = data.as_str().try_into();
                            return match m {
                                Ok(m) => Ok(m),
                                Err(e) => Err(::pyo3::exceptions::PyValueError::new_err(e))
                            };
                        }

                        Ok(Self::default())
                    }
                    None => Ok(Self::default()),
                }
            }

            fn __repr__(&self) -> String {
                format!("{:#?}", self)
            }

            fn __bytes__(&self) -> ::std::borrow::Cow<[u8]> {
                let data: &[u8] = <#c_ident>::from(self.clone()).into();
                data.to_owned().into()
            }

            fn __eq__(&self, other: &Self) -> bool {
                let a: &[u8] = <#c_ident>::from(self.clone()).into();
                let b: &[u8] = <#c_ident>::from(other.clone()).into();
                a == b
            }

            #(#get_sets)*
        }
    };

    // println!("\n\n{output}\n\n");
    let mut p = ::std::process::Command::new("rustfmt")
        .stdin(::std::process::Stdio::piped())
        // .stdout(::std::process::Stdio::piped())
        .spawn()
        .unwrap();
    let mut stdin = p.stdin.take().unwrap();
    stdin.write_all(output.to_string().as_bytes()).unwrap();

    // let mut stdout = p.stdout.take().unwrap();
    // stdout.read_to_string(output.to_string().as_bytes()).unwrap();

    // let mut fd = ::std::fs::File::options()
    //     .create(true)
    //     .append(true)
    //     .open("src/out0.rs")
    //     .unwrap();
    // fd.seek(::std::io::SeekFrom::End(0)).unwrap();
    // fd.write(b"\n\n\n// ----------------------------\n\n\n").unwrap();
    // let mut buf: Vec<u8> = Vec::new();
    // stdout.read(&mut buf).unwrap();
    // fd.write_all(buf.as_slice()).unwrap();

    output.into()
}
