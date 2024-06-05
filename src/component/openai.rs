// std
use std::{
	sync::{Arc, Mutex},
	thread,
};
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
use futures::StreamExt;
// self
use crate::component::setting::Ai;
use tokio::runtime::Runtime;

pub struct OpenAi {
	// TODO: custom API endpoints.
	pub client: Client<OpenAIConfig>,
	pub setting: Ai,
}
impl OpenAi {
	pub fn new(setting: Ai) -> Self {
		Self {
			client: Client::with_config(OpenAIConfig::new().with_api_key(&setting.api_key)),
			setting,
		}
	}

	pub fn chat<P, C>(
		&self,
		prompt: &str,
		content: &str,
		output: Arc<Mutex<String>>,
		token_count: Arc<Mutex<(usize, usize)>>,
	) {
		let model = self.setting.model.clone();
		let msg = [
			ChatCompletionRequestSystemMessageArgs::default()
				.content(prompt)
				.build()
				.unwrap()
				.into(),
			ChatCompletionRequestUserMessageArgs::default()
				.content(content)
				.build()
				.unwrap()
				.into(),
		];
		let req = CreateChatCompletionRequestArgs::default()
			.model(&model)
			.temperature(self.setting.temperature_rounded())
			.max_tokens(4_096_u16)
			.messages(&msg)
			.build()
			.unwrap();
		let c = self.client.clone();

		thread::spawn(move || {
			// TODO: use ctx.
			Runtime::new().unwrap().block_on(async {
				let mut s = c.chat().create_stream(req).await.unwrap();

				// token_count.lock().unwrap().0 = tiktoken_rs::num_tokens_from_messages(
				// 	&model,
				// 	&[(&msg[0]).into(), (&msg[1]).into()],
				// )
				// .unwrap();

				while let Some(r) = s.next().await {
					match r {
						Ok(resp) => {
							resp.choices.iter().for_each(|c| {
								if let Some(c) = &c.delta.content {
									let o = {
										let mut o = output.lock().unwrap();

										o.push_str(c);

										o.to_owned()
									};

									// token_count.lock().unwrap().1 =
									// 	tiktoken_rs::num_tokens_from_messages(
									// 		&model,
									// 		&[(&ChatCompletionRequestMessage::from(
									// 			ChatCompletionRequestAssistantMessageArgs::default(
									// 			)
									// 			.content(o)
									// 			.build()
									// 			.unwrap(),
									// 		))
									// 			.into()],
									// 	)
									// 	.unwrap();
								}
							});
						},
						Err(e) => println!("error: {e}"),
					}
				}
			});
		});
	}
}

// TODO: use enum.
#[derive(Debug)]
pub struct Model;
impl Model {
	const GPT_3_5_TURBO_0301: &'static str = "gpt-3.5-turbo-0301";
	const GPT_3_5_TURBO_0613: &'static str = "gpt-3.5-turbo-0613";
	const GPT_4: &'static str = "gpt-4";
	const GPT_4_0613: &'static str = "gpt-4-0613";
	const GPT_4_1106_PREVIEW: &'static str = "gpt-4-1106-preview";
	const GPT_4_32K: &'static str = "gpt-4-32k";
	const GPT_4_32K_0613: &'static str = "gpt-4-32k-0613";

	pub fn default() -> &'static str {
		Self::GPT_4_1106_PREVIEW
	}

	pub fn all() -> [&'static str; 7] {
		[
			Self::GPT_3_5_TURBO_0301,
			Self::GPT_3_5_TURBO_0613,
			Self::GPT_4,
			Self::GPT_4_0613,
			Self::GPT_4_1106_PREVIEW,
			Self::GPT_4_32K,
			Self::GPT_4_32K_0613,
		]
	}

	// https://openai.com/pricing
	pub fn price_of(model: &str) -> (f32, f32) {
		match model {
			Self::GPT_3_5_TURBO_0301 | Self::GPT_3_5_TURBO_0613 => (0.000001, 0.000002),
			Self::GPT_4 | Self::GPT_4_0613 | Self::GPT_4_32K_0613 => (0.00003, 0.00006),
			Self::GPT_4_1106_PREVIEW => (0.00001, 0.00003),
			Self::GPT_4_32K => (0.00006, 0.00012),
			_ => unreachable!(),
		}
	}
}
