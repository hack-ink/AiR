// std
use std::{
	sync::{Arc, Mutex},
	thread,
};
// serde
use serde::Deserialize;
// self
use super::fundamental::http::HttpClient;

#[derive(Clone, Debug)]
pub struct Quoter {
	pub quote: Arc<Mutex<Option<String>>>,
	http: HttpClient,
}
impl Quoter {
	pub fn refresh(&mut self) {
		let Quoter { quote, http } = self.to_owned();

		thread::spawn(move || {
			if let Ok(mut q) = quote.try_lock() {
				tracing::info!("fetching quote");

				if let Ok(r) = http.get("https://api.quotable.io/random") {
					if let Ok(Quote { author, content }) = r.json::<Quote>() {
						*q = Some(format!("{content}\n\n{author}"));
					}
				}
			}
		});
	}
}
impl Default for Quoter {
	fn default() -> Self {
		let mut q = Self { quote: Arc::new(Mutex::new(None)), http: HttpClient::default() };

		q.refresh();

		q
	}
}

#[derive(Debug, Deserialize)]
struct Quote {
	author: String,
	content: String,
}
