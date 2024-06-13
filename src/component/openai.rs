// std
use std::sync::{Arc, Mutex};
// crates.io
use async_openai::{
	config::OpenAIConfig,
	types::{
		// ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestMessage,
		ChatCompletionRequestSystemMessageArgs,
		ChatCompletionRequestUserMessageArgs,
		CreateChatCompletionRequestArgs,
	},
	Client,
};
use eframe::egui::WidgetText;
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use tokio::runtime::Runtime;
// self
use crate::{component::setting::Ai, prelude::*};

#[derive(Debug)]
pub struct OpenAi {
	pub client: Arc<Client<OpenAIConfig>>,
	// TODO: use Mutex.
	pub setting: Ai,
	pub output: Arc<Mutex<String>>,
}
impl OpenAi {
	pub fn new(setting: Ai) -> Self {
		Self {
			client: Arc::new(Client::with_config(
				OpenAIConfig::new()
					.with_api_base("https://aihubmix.com/v1")
					.with_api_key(&setting.api_key),
			)),
			setting,
			output: Arc::new(Mutex::new(Default::default())),
		}
	}

	pub fn chat(&self, runtime: &Runtime, prompt: &str, content: &str) -> Result<()> {
		let msg = [
			ChatCompletionRequestSystemMessageArgs::default().content(prompt).build()?.into(),
			ChatCompletionRequestUserMessageArgs::default().content(content).build()?.into(),
		];
		let req = CreateChatCompletionRequestArgs::default()
			.model(self.setting.model.as_str())
			.temperature(self.setting.temperature_rounded())
			.max_tokens(4_096_u16)
			.messages(&msg)
			.build()?;
		let c = self.client.clone();
		let o = self.output.clone();

		runtime.spawn(async move {
			match c.chat().create_stream(req).await {
				Ok(mut s) => {
					o.lock().expect("output must be available").clear();

					while let Some(r) = s.next().await {
						match r {
							Ok(resp) => {
								resp.choices.iter().for_each(|c| {
									if let Some(c) = &c.delta.content {
										o.lock().expect("output must be available").push_str(c);
									}
								});
							},
							Err(e) => println!("failed to receive chat response: {e}"),
						}
					}
				},
				Err(e) => {
					tracing::error!("failed to create chat stream: {e}");
				},
			}
		});

		Ok(())
	}
}

// https://platform.openai.com/docs/models
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
