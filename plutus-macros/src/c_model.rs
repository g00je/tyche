use crate::parser::{MemberType, Model};
use proc_macro2::TokenStream;
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

    let default_tokens = default(model);

    let mut s = TokenStream::new();
    quote_into! {s +=
        #[repr(C)]
        #[derive(Debug)]
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

        #default_tokens

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

fn default(model: &Model) -> TokenStream {
    let c_ident = &model.c_ident;
    let default_fields = model.members.iter().map(|m| {
        fn arr(ty: &MemberType) -> TokenStream {
            match ty {
                MemberType::Array { ty, len } => {
                    let ty = arr(&ty);
                    let mut s = TokenStream::new();
                    quote_into!(s += [
                        #{(0..*len).for_each(|_| quote_into!(s += #ty,))}
                    ]);
                    s
                }
                MemberType::Number { is_float, .. } => {
                    if *is_float {
                        quote!(0.0)
                    } else {
                        quote!(0)
                    }
                }
                MemberType::Bytes { len } => quote!( [0; #len] ),
                MemberType::Ipv4 => quote!([0, 0, 0, 0]),
                MemberType::String { len, .. } => quote!( [0; #len] ),
                MemberType::Model { cty, .. } => {
                    quote!( <#cty>::default() )
                }
                MemberType::Flag { .. } => quote!(),
            }
        }

        let ident = &m.ident;
        match &m.ty {
            MemberType::Array { ty, len } => {
                let ty = arr(ty);
                let mut s = TokenStream::new();
                quote_into!(s += #ident: [
                    #{(0..*len).for_each(|_| quote_into!(s += #ty,))}
                ],);
                s
            }
            MemberType::Bytes { len } => quote!(#ident: [0; #len], ),
            MemberType::Ipv4 => quote!(#ident: [0, 0, 0, 0],),
            MemberType::String { len, .. } => quote!(#ident: [0; #len], ),
            MemberType::Model { cty, .. } => {
                quote!(#ident: <#cty>::default(),)
            }
            MemberType::Number { is_float, .. } => {
                if *is_float {
                    quote!(#ident: 0.0,)
                } else {
                    quote!(#ident: 0,)
                }
            }
            MemberType::Flag { .. } => quote!(),
        }
    });

    let mut s = TokenStream::new();
    quote_into! { s += 
        impl Default for #c_ident {
            fn default() -> Self {
                Self {
                    #{default_fields.for_each(|i| quote_into!(s += #i))}
                }
            }
        }
    };

    s
}
