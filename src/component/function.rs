// std
use std::borrow::Cow;
// self
use super::setting::Chat;

#[derive(Debug)]
pub enum Function {
	Rewrite,
	RewriteDirectly,
	Translate,
	TranslateDirectly,
}
impl Function {
	pub fn is_directly(&self) -> bool {
		matches!(self, Self::RewriteDirectly | Self::TranslateDirectly)
	}

	pub fn prompt<'a>(&'a self, setting: &'a Chat) -> Cow<str> {
		match self {
			Self::Rewrite | Self::RewriteDirectly => setting.rewrite.prompt(),
			Self::Translate | Self::TranslateDirectly => setting.translation.prompt(),
		}
	}
}
