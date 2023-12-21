// std
use std::{
	sync::{Arc, Mutex},
	thread,
};
// serde
use serde::Deserialize;
// air
use super::fundamental::http::HttpClient;

#[derive(Clone, Debug, Default)]
pub(crate) struct Quoter {
	pub(crate) quote: Arc<Mutex<String>>,
	http: HttpClient,
}
impl Quoter {
	pub fn new() -> Self {
		let mut q = Self::default();

		q.try_update();

		q
	}

	pub(crate) fn try_update(&mut self) {
		let Quoter { quote, http } = self.to_owned();

		thread::spawn(move || {
			if let Ok(mut q) = quote.lock() {
				if let Ok(r) = http.get("https://api.quotable.io/random") {
					if let Ok(Quote { author, content }) = r.into_json::<Quote>() {
						*q = format!("{content}\n\n{author}");
					}
				}
			}
		});

		tracing::info!("Quoter is updating");
	}
}

#[derive(Debug, Deserialize)]
struct Quote {
	author: String,
	content: String,
}
