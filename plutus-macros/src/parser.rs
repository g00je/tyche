use core::panic;

use proc_macro2::{Ident, TokenStream, TokenTree};
use quote::{format_ident, TokenStreamExt};

use syn::{
    punctuated::Punctuated, token::Brace, Attribute, Expr, Field, Fields,
    FieldsNamed, GenericArgument, ItemStruct, Lit, Meta, PathArguments, Type,
};

#[derive(Debug)]
pub enum MemberType {
    String {
        len: usize,
        validator: Option<TokenStream>,
    },
    Number {
        ty: Ident,
        min: Option<TokenStream>,
        max: Option<TokenStream>,
        is_float: bool,
    },
    BigInt {
        len: usize,
    },
    Bytes {
        len: usize,
    },
    Model {
        ty: Ident,
        cty: Ident,
        optional: bool,
    },
    Flag {
        fl: Ident,
    },
    Ipv4,
}

#[derive(Debug)]
pub struct Member {
    pub ident: Ident,
    pub ty: MemberType,
    pub private: bool,
    pub arr: Option<Vec<usize>>,
}

#[derive(Debug)]
pub struct Model {
    pub ident: Ident,
    pub c_ident: Ident,
    pub members: Vec<Member>,
    pub has_models: bool,
    pub hexable: bool,
}

pub(crate) fn parse(args: TokenStream, item: ItemStruct) -> Model {
    let members = parse_fields(item.fields);
    let model = Model {
        c_ident: format_ident!("C{}", item.ident),
        ident: item.ident,
        has_models: members.iter().any(|m| match &m.ty {
            MemberType::Model { .. } => true,
            _ => false,
        }),
        members,
        hexable: args.into_iter().any(|x| {
            if let TokenTree::Ident(i) = x {
                i.to_string() == "hex"
            } else {
                false
            }
        }),
    };

    model
}

fn parse_fields(fields: Fields) -> Vec<Member> {
    let fields = match fields {
        Fields::Named(fields) => fields,
        Fields::Unit => FieldsNamed {
            brace_token: Brace::default(),
            named: Punctuated::new(),
        },
        _ => panic!("invalid struct fields"),
    };

    let mut members: Vec<Member> =
        fields.named.iter().map(parse_member).collect();

    if members.iter().any(|m| m.ident.to_string() == "flag") {
        members.push(Member {
            ident: format_ident!("alive"),
            ty: MemberType::Flag { fl: format_ident!("FLAG_ALIVE") },
            private: false,
            arr: None,
        });

        members.push(Member {
            ident: format_ident!("edited"),
            ty: MemberType::Flag { fl: format_ident!("FLAG_EDITED") },
            private: false,
            arr: None,
        });

        members.push(Member {
            ident: format_ident!("private"),
            ty: MemberType::Flag { fl: format_ident!("FLAG_PRIVATE") },
            private: false,
            arr: None,
        });
    }

    members
}

fn parse_member(f: &Field) -> Member {
    let ident = match &f.ident {
        Some(i) => i,
        None => panic!("field ident not found"),
    };

    let (ty, arr) = parse_type(&f.ty, &parse_attrs(&f.attrs));

    Member {
        ident: ident.clone(),
        ty,
        private: ident.to_string().chars().next().unwrap() == '_',
        arr,
    }
}

#[derive(Debug)]
enum Attr {
    Str { validator: Option<TokenStream> },
    Int { min: Option<TokenStream>, max: Option<TokenStream> },
    BigInt,
    Flg,
    Ip4,
    Non,
}

fn parse_attrs(attrs: &Vec<Attribute>) -> Attr {
    if attrs.len() == 0 {
        return Attr::Non;
    }

    let attr = &attrs[0];

    match &attr.meta {
        Meta::Path(m) => match m.segments[0].ident.to_string().as_str() {
            "str" => Attr::Str { validator: None },
            "int" => Attr::Int { min: None, max: None },
            "bigint" => Attr::BigInt,
            "flag" => Attr::Flg,
            "ipv4" => Attr::Ip4,
            _ => Attr::Non,
        },
        Meta::List(m) => {
            let mut values: Vec<(Ident, TokenStream)> = Vec::new();
            let mut iter = m.tokens.clone().into_iter();
            // println!("{m:#?}");
            while let Some(t) = iter.next() {
                let ident = match t {
                    TokenTree::Ident(ident) => ident,
                    a => panic!("key attrs mut be ident: {a}"),
                };

                match iter.next().expect("invalid attrs") {
                    TokenTree::Punct(p) => {
                        if p.as_char() != '=' {
                            panic!("invalid attrs")
                        }
                    }
                    _ => panic!("invalid attrs"),
                }

                let mut s = TokenStream::new();
                while let Some(t) = iter.next() {
                    match t {
                        TokenTree::Punct(p) => {
                            if p.as_char() == ',' {
                                break;
                            }
                            s.append(p)
                        }
                        t => s.append(t),
                    }
                }
                // println!("{ident}: {s}");

                // let mut negetive = false;
                //
                // match iter.next().expect("attr value not found") {
                //     TokenTree::Punct(p) => {
                //         if p.as_char() != '-' {
                //             negetive = true
                //             panic!("invalid punct before value: {p}")
                //         }
                //     }
                // }

                values.push((ident, s));
            }

            match m.path.segments[0].ident.to_string().as_str() {
                "str" => Attr::Str {
                    validator: values.iter().find_map(|(i, t)| {
                        if i.to_string() == "validator" {
                            Some(t.clone())
                        } else {
                            None
                        }
                    }),
                },
                "int" => {
                    let mut min = None;
                    let mut max = None;

                    for (i, t) in values {
                        let i = i.to_string();
                        match i.as_str() {
                            "min" => min = Some(t),
                            "max" => max = Some(t),
                            _ => (),
                        }
                    }

                    Attr::Int { min, max }
                }
                "flag" => Attr::Flg,
                "ipv4" => Attr::Ip4,
                _ => Attr::Non,
            }
        }
        Meta::NameValue(m) => {
            println!("unknown: {:?}", m);
            Attr::Non
        }
    }
}

fn parse_type(ty: &Type, attr: &Attr) -> (MemberType, Option<Vec<usize>>) {
    match &ty {
        Type::Path(ty) => {
            let seg = &ty.path.segments[0];
            let ident = &seg.ident;

            if ident.to_string() == "Option" {
                if let PathArguments::AngleBracketed(ab) = &seg.arguments {
                    if let GenericArgument::Type(ty) = &ab.args[0] {
                        let (mt, arr) = parse_type(ty, attr);
                        if let MemberType::Model { ty, cty, .. } = mt {
                            return (
                                MemberType::Model { ty, cty, optional: true },
                                arr,
                            );
                        }

                        return (mt, arr);
                    }
                    panic!("invalid generic arg");
                }
                panic!("invalid args for Option");
            }

            if let Attr::Flg = attr {
                return (MemberType::Flag { fl: ident.clone() }, None);
            }
            let first_char = ident.to_string().chars().next().unwrap();
            if first_char.is_uppercase() {
                (
                    MemberType::Model {
                        ty: ident.clone(),
                        cty: format_ident!("C{}", ident),
                        optional: false,
                    },
                    None,
                )
            } else {
                let is_float = first_char == 'f';
                if let Attr::Int { min, max } = attr {
                    (
                        MemberType::Number {
                            ty: ident.clone(),
                            min: min.clone(),
                            max: max.clone(),
                            is_float,
                        },
                        None,
                    )
                } else {
                    (
                        MemberType::Number {
                            ty: ident.clone(),
                            min: None,
                            max: None,
                            is_float,
                        },
                        None,
                    )
                }
            }
        }
        Type::Array(arr) => {
            // println!("{:#?}", arr);

            let len: usize = match &arr.len {
                Expr::Lit(lit) => match &lit.lit {
                    Lit::Int(v) => v.base10_parse::<usize>().unwrap(),
                    _ => panic!("array length must be literal"),
                },
                _ => panic!("array length must be literal"),
            };

            let (ty, arr) = parse_type(&arr.elem, attr);
            if let MemberType::Number { ty, .. } = &ty {
                if ty.to_string() == "u8" {
                    return match attr {
                        Attr::BigInt => (MemberType::BigInt { len }, None),
                        Attr::Str { validator } => (
                            MemberType::String {
                                len,
                                validator: validator.clone(),
                            },
                            None,
                        ),
                        Attr::Ip4 => {
                            if len != 4 {
                                panic!("ipv4 length must be 4")
                            }
                            (MemberType::Ipv4, None)
                        }
                        Attr::Non => (MemberType::Bytes { len }, None),
                        _ => todo!("idk about this"),
                    };
                }
            }

            let mut lenv = vec![len];
            if let Some(l) = arr {
                lenv.extend(l)
            }

            (ty, Some(lenv))
        }
        _ => panic!("invalid type {:?}", ty),
    }
}
