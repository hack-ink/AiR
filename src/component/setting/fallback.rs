// crates.io
use language::Language;
use serde::{Deserialize, Deserializer};

pub fn translate_a<'de, D>(d: D) -> Result<Language, D::Error>
where
	D: Deserializer<'de>,
{
	Ok(Language::deserialize(d).unwrap_or(Language::ZhCn))
}

pub fn translate_b<'de, D>(d: D) -> Result<Language, D::Error>
where
	D: Deserializer<'de>,
{
	Ok(Language::deserialize(d).unwrap_or(Language::EnGb))
}
