// std
use std::borrow::Cow;
// crates.io
use async_openai::{
	config::OpenAIConfig,
	types::{
		ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
		ChatCompletionResponseStream, ChatCompletionStreamOptions, CreateChatCompletionRequestArgs,
	},
	Client,
};
use serde::{Deserialize, Serialize};
// self
use super::setting::Ai;
use crate::{prelude::*, widget::ComboBoxItem};

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
	Custom(String),
	Gpt4o,
	Gpt4oMini,
	Gpt4Turbo,
	Gpt35Turbo,
}
impl Model {
	// pub const MODEL_URI: &'static str = "https://platform.openai.com/docs/models";
	pub const PRICE_URI: &'static str = "https://openai.com/pricing";

	pub fn as_str(&self) -> &str {
		match self {
			Self::Custom(s) => s,
			Self::Gpt4o => "gpt-4o",
			Self::Gpt4oMini => "gpt-4o-mini",
			Self::Gpt4Turbo => "gpt-4-turbo",
			Self::Gpt35Turbo => "gpt-3.5-turbo",
		}
	}

	pub fn prices(&self) -> (f32, f32) {
		match self {
			Self::Custom(_) => (0., 0.),
			Self::Gpt4o => (0.000005, 0.000015),
			Self::Gpt4oMini => (0.00000015, 0.0000006),
			Self::Gpt4Turbo => (0.00001, 0.00003),
			Self::Gpt35Turbo => (0.0000005, 0.0000015),
		}
	}
}
impl Default for Model {
	fn default() -> Self {
		Self::Gpt4oMini
	}
}
impl ComboBoxItem for Model {
	type Array = [Self; Self::COUNT];

	const COUNT: usize = 5;

	fn all() -> Self::Array {
		[Self::Custom("".into()), Self::Gpt4o, Self::Gpt4oMini, Self::Gpt4Turbo, Self::Gpt35Turbo]
	}

	fn selectable_str(&self) -> Cow<str> {
		Cow::Borrowed(match self {
			Self::Custom(_) => "Custom",
			Self::Gpt4o => "GPT-4o",
			Self::Gpt4oMini => "GPT-4o Mini",
			Self::Gpt4Turbo => "GPT-4 Turbo",
			Self::Gpt35Turbo => "GPT-3.5 Turbo",
		})
	}
}
