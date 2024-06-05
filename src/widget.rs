#[derive(Debug)]
pub struct ApiKey {
	pub label: String,
	pub visibility: bool,
}
impl ApiKey {
	pub fn clicked(&mut self) {
		self.label = match self.label.as_str() {
			"show" => "hide".into(),
			"hide" => "show".into(),
			_ => unreachable!(),
		};
		self.visibility = !self.visibility;
	}
}
impl Default for ApiKey {
	fn default() -> Self {
		Self { label: "show".into(), visibility: true }
	}
}
