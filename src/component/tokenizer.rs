// std
use std::sync::{Arc, RwLock};
// crates.io
use llm_utils::tokenizer::LlmTokenizer;

// TODO: get rid of the `Arc<RwLock<_>>` wrapper.
#[derive(Debug)]
pub struct Tokenizer(Arc<RwLock<LlmTokenizer>>);
impl Tokenizer {
	pub fn new(model_id: &str) -> Self {
		Self(Arc::new(RwLock::new(LlmTokenizer::new_tiktoken(model_id))))
	}

	pub fn count_token(&self, input: &str, output: &str) -> (u32, u32) {
		// TODO: handle the error.
		let t = self.0.read().unwrap();

		(t.count_tokens(input), t.count_tokens(output))
	}
}
