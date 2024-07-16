// std
use std::{ffi::OsStr, os::windows::ffi::OsStrExt, ptr};
// crates.io
use winapi::um::winuser::{self, SW_MINIMIZE, SW_RESTORE};
// self
use super::*;

impl Os {
	pub fn obtain_window(&mut self) {
		let hwnd = winuser::FindWindowW(
			ptr::null_mut(),
			OsStr::new("AiR").encode_wide().chain(Some(0)).collect::<Vec<_>>().as_ptr(),
		);

		if hwnd.is_null() {
			panic!("window must be found");
		} else {
			self.window = Some(window);
		}
	}

	pub fn hide(&self) {
		unsafe {
			winuser::ShowWindowAsync(
				self.window.as_ref().expect("window must be found"),
				SW_MINIMIZE,
			);
		}
	}

	pub fn unhide(&self) {
		unsafe {
			winuser::ShowWindowAsync(
				self.window.as_ref().expect("window must be found"),
				SW_RESTORE,
			);
		}
	}

	pub fn stick_to_top(&self) {}
}
