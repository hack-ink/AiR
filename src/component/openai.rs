// crates.io
use async_openai::{
	config::OpenAIConfig,
	types::{
		ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
		ChatCompletionResponseStream, CreateChatCompletionRequestArgs,
	},
	Client,
};
use eframe::egui::WidgetText;
use serde::{Deserialize, Serialize};
// self
use super::setting::Ai;
use crate::prelude::*;

#[derive(Debug)]
pub struct OpenAi {
	pub client: Client<OpenAIConfig>,
	pub setting: Ai,
}
impl OpenAi {
	pub fn new(setting: Ai) -> Self {
		let client = Client::with_config(
			OpenAIConfig::new().with_api_base(&setting.api_base).with_api_key(&setting.api_key),
		);

		Self { client, setting }
	}

	pub fn reload(&mut self, setting: Ai) {
		self.client = Client::with_config(
			OpenAIConfig::new().with_api_base(&setting.api_base).with_api_key(&setting.api_key),
		);
		self.setting = setting;
	}

	pub async fn chat(&self, prompt: &str, content: &str) -> Result<ChatCompletionResponseStream> {
		let msg = [
			ChatCompletionRequestSystemMessageArgs::default().content(prompt).build()?.into(),
			ChatCompletionRequestUserMessageArgs::default().content(content).build()?.into(),
		];
		let req = CreateChatCompletionRequestArgs::default()
			.model(self.setting.model.as_str())
			.temperature(self.setting.temperature)
			.max_tokens(4_096_u16)
			.messages(&msg)
			.build()?;
		let stream = self.client.chat().create_stream(req).await?;

		Ok(stream)
	}
}

// https://platform.openai.com/docs/models.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Model {
	Gpt4o,
	Gpt35Turbo,
}
impl Model {
	pub fn as_str(&self) -> &'static str {
		match self {
			Self::Gpt4o => "gpt-4o",
			Self::Gpt35Turbo => "gpt-3.5-turbo",
		}
	}

	// https://openai.com/pricing
	pub fn prices(&self) -> (f32, f32) {
		match self {
			Self::Gpt4o => (0.000005, 0.000015),
			Self::Gpt35Turbo => (0.0000005, 0.0000015),
		}
	}

	pub fn all() -> [Self; 2] {
		[Self::Gpt4o, Self::Gpt35Turbo]
	}
}
impl Default for Model {
	fn default() -> Self {
		Self::Gpt4o
	}
}
#[allow(clippy::from_over_into)]
impl Into<WidgetText> for &Model {
	fn into(self) -> WidgetText {
		self.as_str().into()
	}
}
