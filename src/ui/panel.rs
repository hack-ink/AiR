mod chat;
pub use chat::Chat;

mod setting;
pub use setting::Setting;

mod status;
pub use status::Status;

mod tabs;
pub use tabs::Tabs;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Panel {
	Chat,
	Setting,
}
impl Panel {
	pub fn name(&self) -> &str {
		match self {
			Self::Chat => "Chat",
			Self::Setting => "Setting",
		}
	}
}
impl Default for Panel {
	fn default() -> Self {
		Self::Chat
	}
}
