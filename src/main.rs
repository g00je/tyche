use std::{
    fs::{create_dir, File},
    io::Write,
};

fn main() {
    let _ = create_dir("pkg/plutus");
    File::create("pkg/plutus/py.typed").unwrap();

    let mut fd = File::create("pkg/plutus/__init__.py").unwrap();
    write!(fd, "\nfrom plutus_internal import *\n\n").unwrap();

    write_pyi().unwrap();
    write_pydantic().unwrap();
}

fn write_pyi() -> std::io::Result<()> {
    let mut fd = File::create("pkg/plutus/__init__.pyi")?;

    write!(fd, "\nimport plutus\n\n")?;
    write!(fd, "{}\n\n", plutus_internal::ResponseHead::PYI)?;
    write!(fd, "{}\n\n", plutus_internal::Gene::PYI)?;
    write!(fd, "{}\n\n", plutus_internal::Detail::PYI)?;
    write!(fd, "{}\n\n", plutus_internal::Record::PYI)?;
    write!(fd, "{}\n\n", plutus_internal::Agent::PYI)?;
    write!(fd, "{}\n\n", plutus_internal::Duration::PYI)?;
    write!(fd, "{}\n\n", plutus_internal::Eatery::PYI)?;
    write!(fd, "{}\n\n", plutus_internal::Dish::PYI)?;
    write!(fd, "{}\n\n", plutus_internal::Review::PYI)?;
    write!(fd, "{}\n\n", plutus_internal::ReviewData::PYI)?;
    write!(fd, "{}\n\n", plutus_internal::BlockHeader::PYI)?;
    write!(fd, "{}\n\n", plutus_internal::ReviewBlock::PYI)?;
    write!(fd, "{}\n\n", plutus_internal::MenuBlock::PYI)?;
    write!(fd, "{}\n\n", plutus_internal::SessionInfo::PYI)?;
    write!(fd, "{}\n\n", plutus_internal::Session::PYI)?;
    write!(fd, "{}\n\n", plutus_internal::User::PYI)?;
    write!(fd, "{}\n\n", plutus_internal::UserLoginArgs::PYI)?;

    Ok(())
}

fn write_pydantic() -> std::io::Result<()> {
    let mut fd = File::create("pkg/plutus/models.py")?;

    write!(fd, "\nimport pydantic\n\n")?;
    write!(fd, "{}\n\n", plutus_internal::ResponseHead::get_pydantic())?;
    write!(fd, "{}\n\n", plutus_internal::Gene::get_pydantic())?;
    write!(fd, "{}\n\n", plutus_internal::Detail::get_pydantic())?;
    write!(fd, "{}\n\n", plutus_internal::Record::get_pydantic())?;
    write!(fd, "{}\n\n", plutus_internal::Agent::get_pydantic())?;
    write!(fd, "{}\n\n", plutus_internal::Duration::get_pydantic())?;
    write!(fd, "{}\n\n", plutus_internal::Eatery::get_pydantic())?;
    write!(fd, "{}\n\n", plutus_internal::Dish::get_pydantic())?;
    write!(fd, "{}\n\n", plutus_internal::Review::get_pydantic())?;
    write!(fd, "{}\n\n", plutus_internal::ReviewData::get_pydantic())?;
    write!(fd, "{}\n\n", plutus_internal::BlockHeader::get_pydantic())?;
    write!(fd, "{}\n\n", plutus_internal::ReviewBlock::get_pydantic())?;
    write!(fd, "{}\n\n", plutus_internal::MenuBlock::get_pydantic())?;
    write!(fd, "{}\n\n", plutus_internal::SessionInfo::get_pydantic())?;
    write!(fd, "{}\n\n", plutus_internal::Session::get_pydantic())?;
    write!(fd, "{}\n\n", plutus_internal::User::get_pydantic())?;
    write!(fd, "{}\n\n", plutus_internal::UserLoginArgs::get_pydantic())?;

    Ok(())
}
