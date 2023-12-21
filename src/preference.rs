// crates.io
// use app_dirs2::{AppDataType, AppInfo};
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};

pub(super) static PREFERENCE: OnceCell<Preference> = OnceCell::new();

// const AIR: AppInfo = AppInfo { name: "AIR", author: "Hack Ink" };

#[derive(Debug, Serialize, Deserialize)]
pub(super) struct Preference {
	hide_on_lost_focus: bool,
}
impl Default for Preference {
	fn default() -> Self {
		Self { hide_on_lost_focus: true }
	}
}
