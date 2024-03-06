use crate::parser::{MemberType, Model};

pub fn cs(model: &Model) -> String {
    let ident = &model.ident;

    let x = model.members.iter().map(|m| {
        let v = match &m.ty {
            MemberType::Number { is_float, ty, .. } => Some((
                if *is_float {
                    match ty.to_string().as_str() {
                        "f32" => "float",
                        "f64" => "double",
                        _ => panic!("invalid float"),
                    }
                    .to_string()
                } else {
                    let x = ty.to_string();
                    let mut x = x.chars();
                    let sign = match x.next().unwrap() {
                        'u' => "u",
                        _ => "",
                    };
                    let size = x.collect::<String>();
                    format!("{}int{}_t", sign, size)
                },
                String::new(),
            )),
            MemberType::String { len, .. } => {
                Some(("char".to_string(), format!("[{len}]")))
            }
            MemberType::Bytes { len } => {
                Some(("uint8_t".to_string(), format!("[{len}]")))
            }
            MemberType::Model { ty, .. } => {
                Some((ty.to_string(), String::new()))
            }
            MemberType::Ipv4 => {
                Some(("uint8_t".to_string(), "[4]".to_string()))
            }
            MemberType::Flag { .. } => None,
        };

        let x = match &m.arr {
            Some(a) => a.iter().fold(String::new(), |a, i| {
                format!("{a}[{i}]")
            }),
            None => String::new()
        };

        if let Some((a, b)) = v {
            Some(format!("{a} {}{x}{b}", m.ident))
        } else {
            None
        }
    });

    let mut result = format!("typedef struct {ident}");
    result += " {\n";
    x.for_each(|k| {
        if let Some(s) = k {
            result += format!("    {s};\n").as_str();
        }
    });

    result += "} ";
    result += &format!("{ident};");

    result
}
