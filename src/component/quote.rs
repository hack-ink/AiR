// serde
use serde::Deserialize;
// self
use super::net::{Http, Response, HTTP_CLIENT};
use crate::prelude::*;

#[derive(Debug)]
pub struct Quoter;
impl Quoter {
	//  const API: &'static str = "https://api.quotable.io/random";
	const API: &'static str = "https://zenquotes.io/api/random";
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

		let b = HTTP_CLIENT.get_with_reties(Self::API, 3, 500).await?;
		// let q = b.json::<Quote>()?;
		let q = b.json::<[Quote; 1]>()?;

		// Ok(format!("{}\n\n{}", q.content, q.author))
		Ok(format!("{}\n\n{}", q[0].q, q[0].a))
	}
}

#[derive(Debug, Deserialize)]
// struct Quote {
// 	author: String,
// 	content: String,
// }
struct Quote {
	q: String,
	a: String,
}
