use crate::parser::{Member, MemberType};
use proc_macro2::{Ident, TokenStream};
use quote::quote;

pub fn c_model(c_ident: &Ident, members: &Vec<Member>) -> TokenStream {
    let c_fields = members.iter().map(|m| {
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
                MemberType::Ipv4  => quote!([u8; 4]),
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
            MemberType::Ipv4  => quote!(#ident: [u8; 4],),
        }
    });

    quote!{
        #[repr(C)]
        #[derive(Debug)]
        struct #c_ident {
            #(#c_fields)*
        }

        impl #c_ident {
            const SIZE: usize = ::core::mem::size_of::<#c_ident>();
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
    }
}

