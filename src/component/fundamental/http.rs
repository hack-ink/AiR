// std
use std::time::Duration;
// crates.io
use reqwest::{
	blocking::{Client, ClientBuilder, Response},
	Result,
};

#[derive(Clone, Debug)]
pub struct HttpClient(Client);
impl HttpClient {
	pub fn get(&self, url: &str) -> Result<Response> {
		self.0.get(url).send()
	}
}
impl Default for HttpClient {
	fn default() -> Self {
		Self(ClientBuilder::new().timeout(Duration::from_secs(5)).build().unwrap())
	}
}
