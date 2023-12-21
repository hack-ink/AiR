#[derive(Debug)]
pub(super) struct ApiKey {
	pub(super) label: String,
	pub(super) value: String,
	pub(super) password: bool,
}
impl Default for ApiKey {
	fn default() -> Self {
		Self { label: "s/h".to_string(), value: Default::default(), password: true }
	}
}
impl ApiKey {
	pub(super) fn clicked(&mut self) {
		// self.label = match self.label.as_str() {
		// 	"show" => "hide".to_string(),
		// 	"hide" => "show".to_string(),
		// 	_ => unreachable!(),
		// };
		self.password = !self.password;
	}
}
