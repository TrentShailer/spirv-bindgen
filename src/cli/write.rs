use std::{
    io::{self, Write},
    process::Child,
};

use proc_macro2::TokenStream;

pub fn write_formatted<W: Into<std::process::Stdio>>(
    tokens: TokenStream,
    out: W,
) -> io::Result<Child> {
    let mut child = std::process::Command::new("rustfmt")
        .arg("--edition")
        .arg("2024")
        .stdout(out)
        .stdin(std::process::Stdio::piped())
        .spawn()?;

    let mut stdin = child
        .stdin
        .take()
        .expect("Failed to take stdin on rustfmt process");

    stdin.write_all(tokens.to_string().as_bytes())?;

    Ok(child)
}
