// serde
use serde::Deserialize;
// air
use super::fundamental::http::HttpClient;

#[derive(Debug, Deserialize)]
pub(crate) struct Quote {
	pub(crate) author: String,
	pub(crate) content: String,
}

#[derive(Debug, Default)]
pub(crate) struct Quoter(HttpClient);
impl Quoter {
	pub(crate) fn get(&self) -> Option<String> {
		self.0.get("https://api.quotable.io/random").ok().and_then(|r| {
			r.into_json::<Quote>().ok().map(|q| format!("{}\n\n{}", q.content, q.author))
		})
	}
}
