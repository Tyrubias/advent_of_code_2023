#![forbid(clippy::expect_used)]
#![forbid(clippy::unwrap_used)]
#![forbid(clippy::panic)]
#![forbid(unsafe_code)]

use thiserror::Error;

use std::{
    io,
    path::{Path, PathBuf},
    process::Command,
    str,
};

#[derive(Error, Debug)]
pub enum UtilityError {
    #[error("can't call Cargo")]
    Cargo(#[from] io::Error),
    #[error("workspace path isn't UTF-8")]
    InvalidUnicode(#[from] str::Utf8Error),
    #[error("no parent directory of workspace")]
    NoParentDirectory,
}

pub fn workspace_root() -> Result<PathBuf, UtilityError> {
    let output = Command::new(env!("CARGO"))
        .arg("locate-project")
        .arg("--workspace")
        .arg("--message-format=plain")
        .output()?;

    let cargo_path = Path::new(str::from_utf8(&output.stdout)?.trim());

    Ok(cargo_path
        .parent()
        .ok_or(UtilityError::NoParentDirectory)?
        .to_path_buf())
}
