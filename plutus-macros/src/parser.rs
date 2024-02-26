use proc_macro2::{Ident, TokenStream, TokenTree};
use quote::format_ident;

use syn::{
    punctuated::Punctuated, token::Brace, Attribute, Expr, Field, Fields,
    FieldsNamed, ItemStruct, Lit, Meta,
};

use syn::{LitInt, Type};

#[derive(Debug)]
pub enum MemberType {
    String { len: usize, validator: Option<Ident> },
    Number { ty: Ident, min: Option<usize>, max: Option<usize> },
    Bytes { len: usize },
    Model { ty: Ident, cty: Ident },
    Array { ty: Box<MemberType>, len: usize },
    Flag { fl: Ident },
    Ipv4,
}

#[derive(Debug)]
pub struct Member {
    pub ident: Ident,
    pub ty: MemberType,
    pub private: bool,
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
        has_models: has_model(members.as_slice()),
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

fn has_model(members: &[Member]) -> bool {
    fn arr(ty: &MemberType) -> bool {
        match ty {
            MemberType::Array { ty, .. } => arr(ty),
            MemberType::Model { .. } => true,
            _ => false,
        }
    }

    members
        .iter()
        .find_map(|m| if arr(&m.ty) { Some(()) } else { None })
        .is_some()
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
        });

        members.push(Member {
            ident: format_ident!("edited"),
            ty: MemberType::Flag { fl: format_ident!("FLAG_EDITED") },
            private: false,
        });

        members.push(Member {
            ident: format_ident!("private"),
            ty: MemberType::Flag { fl: format_ident!("FLAG_PRIVATE") },
            private: false,
        });
    }

    members
}

fn parse_member(f: &Field) -> Member {
    let ident = match &f.ident {
        Some(i) => i,
        None => panic!("field ident not found"),
    };

    Member {
        ident: ident.clone(),
        ty: parse_type(&f.ty, &parse_attrs(&f.attrs)),
        private: ident.to_string().chars().next().unwrap() == '_',
    }
}

#[derive(Debug)]
enum Attr {
    Str { validator: Option<Ident> },
    Int { min: Option<usize>, max: Option<usize> },
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
            "flag" => Attr::Flg,
            "ipv4" => Attr::Ip4,
            _ => Attr::Non,
        },
        Meta::List(m) => {
            let mut values: Vec<(Ident, TokenTree)> = Vec::new();
            let mut iter = m.tokens.clone().into_iter();
            while let Some(t) = iter.next() {
                let ident = match t {
                    TokenTree::Ident(ident) => ident,
                    _ => panic!("invalid attrs"),
                };

                match iter.next().expect("invalid attrs") {
                    TokenTree::Punct(p) => {
                        if p.as_char() != '=' {
                            panic!("invalid attrs")
                        }
                    }
                    _ => panic!("invalid attrs"),
                }

                values.push((ident, iter.next().expect("invalid attrs")));
            }

            match m.path.segments[0].ident.to_string().as_str() {
                "str" => Attr::Str {
                    validator: values.iter().find_map(|v| {
                        if v.0.to_string() == "validator" {
                            match &v.1 {
                                TokenTree::Ident(i) => Some(i.clone()),
                                _ => panic!("invalid value for validator"),
                            }
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
                            "min" | "max" => {
                                let amount = Some(match &t {
                                    TokenTree::Literal(lit) => {
                                        LitInt::from(lit.clone())
                                            .base10_parse::<usize>()
                                            .unwrap()
                                    }
                                    _ => panic!("invalid {i}"),
                                });

                                if i == "min" {
                                    min = amount;
                                } else {
                                    max = amount;
                                }
                            }
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

fn parse_type(ty: &Type, attr: &Attr) -> MemberType {
    match &ty {
        Type::Path(ty) => {
            let ident = &ty.path.segments[0].ident;
            if let Attr::Flg = attr {
                return MemberType::Flag { fl: ident.clone() };
            }
            if ident.to_string().chars().next().unwrap().is_uppercase() {
                MemberType::Model {
                    ty: ident.clone(),
                    cty: format_ident!("C{}", ident),
                }
            } else {
                if let Attr::Int { min, max } = attr {
                    MemberType::Number { ty: ident.clone(), min: *min, max: *max }
                } else {
                    MemberType::Number {
                        ty: ident.clone(),
                        min: None,
                        max: None,
                    }
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

            let ty = parse_type(&arr.elem, attr);
            if let MemberType::Number { ty, .. } = &ty {
                if ty.to_string() == "u8" {
                    return match attr {
                        Attr::Str { validator } => MemberType::String {
                            len,
                            validator: validator.clone(),
                        },
                        Attr::Ip4 => {
                            if len != 4 {
                                panic!("ipv4 length must be 4")
                            }
                            MemberType::Ipv4
                        }
                        Attr::Non => MemberType::Bytes { len },
                        _ => todo!("idk about this"),
                    };
                }
            }

            MemberType::Array { ty: Box::new(ty), len }
        }
        _ => panic!("invalid type {:?}", ty),
    }
}
