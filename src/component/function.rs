// std
use std::borrow::Cow;
// crates.io
use serde::{Deserialize, Serialize};
// self
use super::setting::Chat;
use crate::widget::ComboBoxItem;

#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Function {
	Rewrite,
	RewriteDirectly,
	Translate,
	TranslateDirectly,
	// TODO: refactor code.
}
impl Function {
	pub fn basic(&self) -> Self {
		match self {
			Self::Rewrite | Self::RewriteDirectly => Self::Rewrite,
			Self::Translate | Self::TranslateDirectly => Self::Translate,
		}
	}

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
impl Default for Function {
	fn default() -> Self {
		Self::Rewrite
	}
}
impl ComboBoxItem for Function {
	type Array = [Self; Self::COUNT];

	const COUNT: usize = 2;

	fn all() -> Self::Array {
		[Self::Rewrite, Self::Translate]
	}

	fn display(&self) -> Cow<str> {
		Cow::Borrowed(match self {
			Self::Rewrite | Self::RewriteDirectly => "Rewrite",
			Self::Translate | Self::TranslateDirectly => "Translate",
		})
	}
}
