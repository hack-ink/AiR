// std
use std::borrow::Cow;
// crates.io
use eframe::egui::WidgetText;
use serde::{Deserialize, Serialize};
// self
use super::setting::Chat;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
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

	pub fn basic_all() -> [Self; 2] {
		[Self::Rewrite, Self::Translate]
	}

	pub fn basic_as_str(&self) -> &'static str {
		match self {
			Self::Rewrite | Self::RewriteDirectly => "Rewrite",
			Self::Translate | Self::TranslateDirectly => "Translate",
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
#[allow(clippy::from_over_into)]
impl Into<WidgetText> for &Function {
	fn into(self) -> WidgetText {
		self.basic_as_str().into()
	}
}
