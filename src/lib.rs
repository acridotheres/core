#![allow(clippy::too_many_arguments)]

pub mod archive;
pub mod file;
pub mod formats;

use std::{
    fs::create_dir_all,
    io::{Error, ErrorKind, Result},
    path::Path,
};

pub fn get_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

pub(crate) fn prepare_output_dir(output: &Path) -> Result<()> {
    if !output.exists() {
        match create_dir_all(output) {
            Ok(_) => {}
            Err(_) => {
                return Err(Error::new(
                    ErrorKind::Other,
                    "Failed to create output directory",
                ))
            }
        }
    }

    Ok(())
}
