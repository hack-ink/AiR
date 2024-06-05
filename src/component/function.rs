#[derive(Debug)]
pub enum Function {
	Polish,
	// Translate,
}
impl Function {
	pub fn prompt(&self) -> &'static str {
		match self {
			Self::Polish =>
				"\
				As an English professor, assist me in refining this text. \
				Amend any grammatical errors and enhance the language to sound more like a native speaker.\
				Provide the text alone, without any additional commentary.\
				",
			// Self::Translate => "\
			// 	Translate the following text into English.\
			// 	",
		}
	}
}
