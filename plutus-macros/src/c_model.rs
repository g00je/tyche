use crate::parser::{MemberType, Model};
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use quote_into::quote_into;

pub fn c_model(model: &Model) -> TokenStream {
    let c_ident = &model.c_ident;
    let c_fields = model.members.iter().map(|m| {
        fn arr(ty: &MemberType) -> TokenStream {
            match ty {
                MemberType::Array { ty, len } => {
                    let ty = arr(&ty);
                    quote!( [#ty; #len] )
                }
                MemberType::Number { ty, .. } => quote! { #ty },
                MemberType::Bytes { len } => quote!( [u8; #len] ),
                MemberType::String { len, .. } => quote!( [u8; #len] ),
                MemberType::Model { cty, .. } => quote!( #cty ),
                MemberType::Flag { .. } => quote!(),
                MemberType::Ipv4 => quote!([u8; 4]),
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
            MemberType::Flag { .. } => quote!(),
            MemberType::Ipv4 => quote!(#ident: [u8; 4],),
        }
    });

    let mut s = TokenStream::new();
    quote_into! {s +=
        #[repr(C)]
        #[derive(Debug, Default)]
        struct #c_ident {
            #{for f in c_fields {
                quote_into!(s += #f)
            }}
        }

        impl #c_ident {
            const SIZE: usize = ::core::mem::size_of::<#c_ident>();

            #{if model.hexable {
            quote_into!{s +=
            fn is_none(&self) -> bool {
                let data: Vec<u8> = self.into();
                data.iter().all(|x| *x == 0)
            }}}}
        }

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
