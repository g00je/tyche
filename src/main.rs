use std::{
    fs::{create_dir, File},
    io::Write,
};

mod macros;

fn main() {
    let _ = create_dir("pkg/plutus");
    File::create("pkg/plutus/py.typed").unwrap();

    let mut fd = File::create("pkg/plutus/__init__.py").unwrap();
    write!(fd, "\nfrom plutus_internal import *\n\n").unwrap();

    write_stuff().unwrap();
}

fn write_stuff() -> std::io::Result<()> {
    let mut pyi_fd = File::create("pkg/plutus/__init__.pyi")?;
    write!(pyi_fd, "\nimport plutus\n\n")?;

    let mut pydantic_fd = File::create("pkg/plutus/models.py")?;
    write!(pydantic_fd, "\nimport pydantic\n\n")?;

    let mut cs_fd = File::create("pkg/models.h")?;
    write!(cs_fd, "
#ifndef __PLUTUS_MODELS_H__
#define __PLUTUS_MODELS_H__\n

#include <stdbool.h>
#include <stdint.h>
#include <stddef.h>

// #ifdef DO_PAD_CHECK\n
")?;

    macros::act_on_models! {($X:ident) => {
        write!(pyi_fd, "{}\n\n", plutus_internal::$X::PYI)?;
        write!(pydantic_fd, "{}\n\n", plutus_internal::$X::get_pydantic())?;
        write!(cs_fd, "{}\n\n", plutus_internal::$X::CS)?;
    }}

    write!(cs_fd, "\n#endif // __PLUTUS_MODELS_H__\n")?;

    Ok(())
}

