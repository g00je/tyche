use proc_macro2::{Ident, TokenTree};
use quote::format_ident;

use syn::{
    punctuated::Punctuated, token::Brace, Attribute, Expr, Field, Fields,
    FieldsNamed, ItemStruct, Lit, Meta,
};

use syn::Type;

#[derive(Debug)]
pub enum MemberType {
    String { len: usize, validator: Option<Ident> },
    Number { ty: Ident, min: Option<usize>, max: Option<usize> },
    Bytes { len: usize },
    Model { ty: Ident, cty: Ident },
    Array { ty: Box<MemberType>, len: usize },
}

#[derive(Debug)]
pub struct Member {
    pub ident: Ident,
    pub ty: MemberType,
}

#[derive(Debug)]
pub struct Model {
    pub ident: Ident,
    pub c_ident: Ident,
    pub members: Vec<Member>,
}

pub(crate) fn parse(item: ItemStruct) -> Model {
    let model = Model {
        c_ident: format_ident!("C{}", item.ident),
        ident: item.ident,
        members: parse_fields(item.fields),
    };

    // println!("{model:#?}");

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

    fields.named.iter().map(parse_member).collect()
}

fn parse_member(f: &Field) -> Member {
    let ident = match &f.ident {
        Some(i) => i,
        None => panic!("field ident not found"),
    };

    Member {
        ident: ident.clone(),
        ty: parse_type(&f.ty, &parse_attrs(&f.attrs)),
    }
}

#[derive(Debug)]
enum Attr {
    Str { validator: Option<Ident> },
    Int { min: Option<usize>, max: Option<usize> },
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
                "int" => Attr::Int { min: None, max: None },
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
            if ident.to_string().chars().next().unwrap().is_uppercase() {
                MemberType::Model {
                    ty: ident.clone(),
                    cty: format_ident!("C{}", ident),
                }
            } else {
                MemberType::Number {
                    ty: ty.path.segments[0].ident.clone(),
                    min: None,
                    max: None,
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
