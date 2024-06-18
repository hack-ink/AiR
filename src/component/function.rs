// self
use super::setting::Translation;

#[derive(Debug)]
pub enum Function {
	Rewrite,
	RewriteDirectly,
	Translate,
	TranslateDirectly,
}
impl Function {
	pub fn prompt(&self, setting: &Translation) -> String {
		match self {
			Self::Rewrite | Self::RewriteDirectly =>
				"As an English professor, assist me in refining this text. \
				Amend any grammatical errors and enhance the language to sound more like a native speaker.\
				Provide the refined text only, without any other things."
					.into(),
			Self::Translate | Self::TranslateDirectly => format!(
				"As a language professor, assist me in translate this text from {} to {}. \
				Amend any grammatical errors and enhance the language to sound more like a native speaker.\
				Provide the translated text only, without any other things.",
				setting.source.as_str(),
				setting.target.as_str()
			),
		}
	}
}
