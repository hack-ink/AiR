// std
use std::{ffi::OsStr, os::windows::ffi::OsStrExt, ptr};
// crates.io
use winapi::{
	shared::windef::HWND__,
	um::winuser::{self, SW_MINIMIZE, SW_RESTORE},
};
// self
use super::*;

impl Os {
	pub fn get_hwnd() -> *mut HWND__ {
		let window_title = OsStr::new("AiR").encode_wide().chain(Some(0)).collect::<Vec<_>>();

		unsafe { winuser::FindWindowW(ptr::null_mut(), window_title.as_ptr()) }
	}

	pub fn hide(&self) {
		unsafe {
			winuser::ShowWindowAsync(self.hwnd, SW_MINIMIZE);
		}
	}

	pub fn unhide(&self) {
		unsafe {
			winuser::ShowWindowAsync(self.hwnd, SW_RESTORE);
		}
	}

	pub fn set_move_to_active_space() {
		// Windows natively supports moving a window to another desktop.
	}
}
