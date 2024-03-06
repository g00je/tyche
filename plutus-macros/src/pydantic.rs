use crate::parser::{MemberType, Model};
use proc_macro2::{Ident, TokenStream};
use quote_into::quote_into;

pub fn pydantic(model: &Model) -> TokenStream {
    let ident = &model.ident;
    let ident_str = &model.ident.to_string();
    let c_ident = &model.c_ident;
    let mut s = TokenStream::new();

    if model.hexable {
        quote_into! {s += format!(
            "{}Model = pydantic.constr(min_length={}, max_length={})",
            #ident_str, <#c_ident>::SIZE * 2, <#c_ident>::SIZE * 2
        )};
        return s;
    }

    let x = model.members.iter().map(|m| {
        if m.private {
            return None;
        }

        fn arr(ty: &MemberType) -> String {
            match ty {
                MemberType::Array { ty, len } => format!(
                    "pydantic.conlist({}, min_length={}, max_length={})",
                    arr(ty),
                    len,
                    len
                ),
                MemberType::Number { is_float, min, max, ty } => {
                    if *is_float {
                        format!(
                            "pydantic.confloat({}{})",
                            if let Some(m) = min {
                                format!("gt={},", m)
                            } else {
                                "".to_string()
                            },
                            if let Some(m) = max {
                                format!("lt={}", m)
                            } else {
                                "".to_string()
                            }
                        )
                    } else {
                        let x = min_max(*min, *max, ty);
                        format!("pydantic.conint(gt={}, lt={})", x.0, x.1)
                    }
                }
                MemberType::String { len, .. } => {
                    format!("pydantic.constr(max_length={})", len)
                }
                MemberType::Bytes { len } => format!(
                    "pydantic.constr(min_length={}, max_length={})",
                    len * 2,
                    len * 2
                ),
                MemberType::Model { ty, optional, .. } => format!(
                    "{ty}Model{}",
                    if *optional { " | None" } else { "" }
                ),
                MemberType::Ipv4 => "str".to_string(),
                MemberType::Flag { .. } => "bool".to_string(),
            }
        }
        match &m.ty {
            MemberType::Array { ty, len } => Some(format!(
                "{}: pydantic.conlist({}, min_length={}, max_length={})",
                m.ident,
                arr(ty),
                len,
                len
            )),
            MemberType::Number { is_float, ty, min, max } => {
                Some(if *is_float {
                    format!(
                        "{}: pydantic.confloat({}{})",
                        m.ident,
                        if let Some(m) = min {
                            format!("gt={},", m)
                        } else {
                            "".to_string()
                        },
                        if let Some(m) = max {
                            format!("lt={}", m)
                        } else {
                            "".to_string()
                        }
                    )
                } else {
                    let x = min_max(*min, *max, ty);
                    format!(
                        "{}: pydantic.conint(gt={}, lt={})",
                        m.ident, x.0, x.1
                    )
                })
            }
            MemberType::String { len, .. } => Some(format!(
                "{}: pydantic.constr(max_length={})",
                m.ident, len
            )),
            MemberType::Bytes { len } => Some(format!(
                "{}: pydantic.constr(min_length={}, max_length={})",
                m.ident,
                len * 2,
                len * 2
            )),
            MemberType::Model { ty, optional, .. } => Some(format!(
                "{}: {ty}Model{}",
                m.ident,
                if *optional { " | None" } else { "" }
            )),
            MemberType::Ipv4 => Some(format!("{}: str", m.ident)),
            MemberType::Flag { .. } => Some(format!("{}: bool", m.ident)),
        }
    });

    let mut result = format!("class {ident}Model(pydantic.BaseModel):\n");
    x.for_each(|k| {
        if let Some(s) = k {
            result += format!("    {s}\n").as_str();
        }
    });

    quote_into!(s += #result .to_string());

    s
}

fn min_max(
    min: Option<usize>, max: Option<usize>, ty: &Ident,
) -> (String, String) {
    let tys = ty.to_string();
    let mut x = tys.chars();
    let ty = x.next().unwrap();
    let s = x.collect::<String>();

    let mn = if let Some(g) = min {
        g.to_string()
    } else {
        match ty {
            'u' => "-1".to_string(),
            // 'i' => (-(1i128 << s) - 1).to_string(),
            'i' => format!("-(1 << {s}) - 1"),
            'f' => panic!("no min max for floats"),
            _ => panic!("invalid number type"),
        }
    };

    let mx = if let Some(g) = max {
        g.to_string()
    } else {
        match ty {
            'u' => format!("1 << {s}"),
            'i' => format!("1 << ({s} - 1)"),
            'f' => panic!("no min max for floats"),
            _ => panic!("invalid number type"),
        }
    };

    (mn, mx)
}