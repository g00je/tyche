use crate::parser::{Model, MemberType};

pub fn pyi(model: &Model) -> String {
    let ident = &model.ident;
    let x = model.members.iter().map(|m| {
        if m.private { return None; }

        let array = |ty: &str| {
            let arr = match &m.arr {
                Some(a) => a,
                None => return ty.to_string(),
            };

            arr.iter().rev().fold(ty.to_string(), |a, _| format!("list[{a}]"))
        };

        let v = match &m.ty {
            MemberType::Number { is_float, .. } => {
                if *is_float { "float" } else { "int" } .to_string()
            }
            MemberType::String { .. } => "str".to_string(),
            MemberType::Bytes { .. } => "bytes".to_string(),
            MemberType::BigInt { .. } => "int".to_string(),
            MemberType::Model { ty, optional, .. } => {
                format!("{ty}{}", if *optional { " | None" } else {""})
            },
            MemberType::Ipv4 => "str".to_string(),
            MemberType::Flag { .. } => "bool".to_string(),
        };

        Some(format!("{}: {}", m.ident, array(v.as_str())))
    });

    let mut pyi_result = format!("class {ident}(plutus.{ident}):\n");
    x.for_each(|k| {
        if let Some(s) = k {
            pyi_result += format!("    {s}\n").as_str();
        }
    });

    pyi_result += "\n    SIZE: int\n\n";

    pyi_result += "    def __new__(value: str | bytes = None) -> bytes: ...\n";
    pyi_result += "    def __bytes__(self) -> bytes: ...\n";
    pyi_result += "    def hex(self) -> str: ...\n";
    pyi_result += "    @classmethod\n    ";
    pyi_result += format!("def batch(cls, value: str | bytes) -> list[{ident}]: ...\n").as_str();
    if model.hexable {
        pyi_result += "    def dict(self) -> str: ...\n";
    } else {
        pyi_result += "    def dict(self) -> dict: ...\n";
    }

    pyi_result
}
