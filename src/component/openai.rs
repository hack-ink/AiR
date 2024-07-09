// crates.io
use async_openai::{
	config::OpenAIConfig,
	types::{
		ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
		ChatCompletionResponseStream, ChatCompletionStreamOptions, CreateChatCompletionRequestArgs,
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
	pub model: Model,
	pub temperature: f32,
}
impl OpenAi {
	pub fn new(setting: Ai) -> Self {
		let Ai { api_base, api_key, model, temperature } = setting;
		let client =
			Client::with_config(OpenAIConfig::new().with_api_base(api_base).with_api_key(api_key));

		Self { client, model, temperature }
	}

	pub async fn chat(&self, prompt: &str, content: &str) -> Result<ChatCompletionResponseStream> {
		let msg = [
			ChatCompletionRequestSystemMessageArgs::default().content(prompt).build()?.into(),
			ChatCompletionRequestUserMessageArgs::default()
				.content(format!("<AiR>\n{content}\n</AiR>"))
				.build()?
				.into(),
		];
		let req = CreateChatCompletionRequestArgs::default()
			.model(self.model.as_str())
			.temperature(self.temperature)
			.max_tokens(4_096_u16)
			.messages(&msg)
			.stream(true)
			.stream_options(ChatCompletionStreamOptions { include_usage: true })
			.build()?;

		tracing::debug!("chatting with: {req:?}");

		let stream = self.client.chat().create_stream(req).await?;

		Ok(stream)
	}
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Model {
	Gpt4o,
	Gpt35Turbo,
}
impl Model {
	pub const MODEL_URI: &'static str = "https://platform.openai.com/docs/models";
	pub const PRICE_URI: &'static str = "https://openai.com/pricing";

	pub fn as_str(&self) -> &'static str {
		match self {
			Self::Gpt4o => "gpt-4o",
			Self::Gpt35Turbo => "gpt-3.5-turbo",
		}
	}

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
