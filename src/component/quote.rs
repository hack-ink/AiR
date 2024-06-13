// std
use std::{
	borrow::Cow,
	sync::{Arc, RwLock},
};
// serde
use serde::Deserialize;
use tokio::runtime::Runtime;
// self
use super::net::{Http, Response, HTTP_CLIENT};

#[derive(Debug)]
pub struct Quoter {
	pub quote: Arc<RwLock<String>>,
}
impl Quoter {
	const DEFAULT: &'static str = r#"  -----------
< Thinking... >
  -----------
         \   ^__^
          \  (oo)\_______
             (__)\       )\/\
                 ||----w |
                 ||     ||"#;

	pub fn refresh(&mut self, runtime: &Runtime) {
		let quote = self.quote.clone();

		runtime.spawn(async move {
			tracing::info!("fetching quote");

			if let Ok(r) =
				HTTP_CLIENT.get_with_reties("https://api.quotable.io/random", 3, 500).await
			{
				if let Ok(Quote { author, content }) = r.json::<Quote>() {
					if let Ok(mut q) = quote.write() {
						*q = format!("{content}\n\n{author}");
					}
				}
			}
		});
	}

	pub fn get(&self) -> Cow<str> {
		self.quote.read().map(|q| Cow::Owned(q.to_owned())).unwrap_or(Cow::Borrowed(Self::DEFAULT))
	}
}
impl Default for Quoter {
	fn default() -> Self {
		Self { quote: Arc::new(RwLock::new(Self::DEFAULT.into())) }
	}
}

#[derive(Debug, Deserialize)]
struct Quote {
	author: String,
	content: String,
}
