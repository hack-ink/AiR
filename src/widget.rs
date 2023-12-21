#[derive(Debug)]
pub(crate) struct ApiKey {
	pub(crate) label: String,
	pub(crate) value: String,
	pub(crate) password: bool,
}
impl Default for ApiKey {
	fn default() -> Self {
		Self { label: "s/h".to_string(), value: Default::default(), password: true }
	}
}
impl ApiKey {
	pub(crate) fn clicked(&mut self) {
		// self.label = match self.label.as_str() {
		// 	"show" => "hide".to_string(),
		// 	"hide" => "show".to_string(),
		// 	_ => unreachable!(),
		// };
		self.password = !self.password;
	}
}
