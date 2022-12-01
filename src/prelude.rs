pub use anyhow::{Error, Result,anyhow};
pub use std::io::prelude::*;
pub use crate::{Part, Args, get_args};
pub use crate::path_or_std::{InputFile, OutputFile};

pub fn default<T: Default>() -> T {
    Default::default()
}