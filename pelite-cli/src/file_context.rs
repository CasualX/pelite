use super::*;

#[derive(Debug)]
struct FileError {
	operation: &'static str,
	path: PathBuf,
	source: io::Error,
}

impl fmt::Display for FileError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "failed to {} {}: {}", self.operation, self.path.display(), self.source)
	}
}

impl error::Error for FileError {
	fn source(&self) -> Option<&(dyn error::Error + 'static)> {
		Some(&self.source)
	}
}

pub trait FileContext<T> {
	fn file_context(self, operation: &'static str, path: impl AsRef<Path>) -> Result<T>;
}

impl<T> FileContext<T> for io::Result<T> {
	fn file_context(self, operation: &'static str, path: impl AsRef<Path>) -> Result<T> {
		self.map_err(|source| Box::new(FileError {
			operation, path: path.as_ref().to_owned(), source,
		}) as Box<dyn error::Error>)
	}
}
