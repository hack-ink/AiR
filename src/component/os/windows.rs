// std
use std::{ffi::OsStr, os::windows::ffi::OsStrExt, ptr};
// crates.io
use winapi::{
	shared::windef::HWND__,
	um::winuser::{self, SW_HIDE, SW_SHOW},
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
			winuser::ShowWindowAsync(self.hwnd, SW_HIDE);
		}
	}

	pub fn unhide(&self) {
		unsafe {
			winuser::ShowWindowAsync(self.hwnd, SW_SHOW);
		}
	}

	pub fn set_move_to_active_space() {
		// Windows natively supports moving a window to another desktop.
	}
}
