use super::*;

impl Analysis<'_> {
	/// Annotate x64 unwind coverage starts with their runtime function record RVAs.
	pub fn scan_exceptions(&mut self) {
		if let Wrap::T64(file) = self.pe {
			match file.exception_x64() {
				Ok(exceptions) => {
					for function in exceptions.image() {
						let rva = function.BeginAddress;
						let Ok(runtime_function) = self.pe.headers().file_offset_to_rva(self.pe.offset_of(function)) else {
							continue
						};
						let comment = format!("RUNTIME_FUNCTION at {runtime_function:#x}");
						self.comments.insert(rva, factmap::CommentFact { rva, comment });
					}
				},
				Err(pelite::Error::Null) => {},
				Err(error) => eprintln!("analysis: x64 exceptions: {error}"),
			}
		}
	}
}
