// serde
use serde::Deserialize;
// self
use super::net::{Http, Response, HTTP_CLIENT};
use crate::prelude::*;

#[derive(Debug)]
pub struct Quoter;
impl Quoter {
	pub const DEFAULT: &'static str = r#"  -----------
< Thinking... >
  -----------
         \   ^__^
          \  (oo)\_______
             (__)\       )\/\
                 ||----w |
                 ||     ||"#;

	pub async fn fetch(&self) -> Result<String> {
		tracing::info!("fetching quote");

		let b = HTTP_CLIENT.get_with_reties("https://api.quotable.io/random", 3, 500).await?;
		let q = b.json::<Quote>()?;

		Ok(format!("{}\n\n{}", q.content, q.author))
	}
}

#[derive(Debug, Deserialize)]
struct Quote {
	author: String,
	content: String,
}
