use std::ffi::{OsStr, OsString};
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{Read, Stdin, stdin, Stdout, stdout, Write};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum PathOrStd {
	Path(PathBuf),
	Std,
}

pub trait InputFileTrait: Read+std::fmt::Debug {}
impl InputFileTrait for Stdin {}
impl InputFileTrait for File {}
pub type InputFile = Box<dyn InputFileTrait>;

pub trait OutputFileTrait: Write+std::fmt::Debug {}
impl OutputFileTrait for Stdout {}
impl OutputFileTrait for File {}
pub type OutputFile = Box<dyn OutputFileTrait>;

impl PathOrStd {
	pub fn create(self) -> std::io::Result<OutputFile> {
		Ok(match self {
			PathOrStd::Path(path) => Box::new(File::create(path)?),
			PathOrStd::Std => Box::new(stdout()),
		})
	}

	#[allow(dead_code)]
	pub fn open(self) -> std::io::Result<InputFile> {
		Ok(match self {
			PathOrStd::Path(path) => Box::new(File::open(path)?),
			PathOrStd::Std => Box::new(stdin()),
		})
	}

	// TODO: Allow other sets of options
}

impl Default for PathOrStd {
	fn default() -> Self {
		PathOrStd::Std
	}
}

impl Display for PathOrStd {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			// TODO: Is there a way to display it as stdout or stdin as appropriate
			PathOrStd::Std => write!(f, "-"),
			PathOrStd::Path(path) => write!(f, "{}", path.display()),
		}
	}
}

impl From<OsString> for PathOrStd {
	#[inline]
	fn from(s: OsString) -> Self {
		if s == "-" {
			PathOrStd::Std
		} else {
			PathOrStd::Path(PathBuf::from(s))
		}
	}
}

impl<T: ?Sized + AsRef<OsStr>> From<&T> for PathOrStd {
	#[inline]
	fn from(s: &T) -> PathOrStd {
		PathOrStd::from(s.as_ref().to_os_string())
	}
}
