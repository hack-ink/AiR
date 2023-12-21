// std
use std::time::Duration;
// crates.io
use ureq::{Agent, AgentBuilder, Error, Response};

#[derive(Debug)]
pub(crate) struct HttpClient(Agent);
impl HttpClient {
	// TODO?: Use `anyhow::Result`.
	pub(crate) fn get(&self, url: &str) -> Result<Response, Box<Error>> {
		Ok(self.0.get(url).call()?)
	}
}
impl Default for HttpClient {
	fn default() -> Self {
		Self(
			AgentBuilder::new()
				.timeout_read(Duration::from_secs(5))
				.timeout_write(Duration::from_secs(5))
				.build(),
		)
	}
}
