#[cfg(feature = "tokenizer")]
pub fn price_rounded(value: f32) -> f32 {
	(value * 1_000_000.).round() / 1_000_000.
}
