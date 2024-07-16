// std
use std::{ffi::OsStr, os::windows::ffi::OsStrExt, ptr};
// crates.io
use winapi::{
	shared::windef::HWND__,
	um::winuser::{
		self, HWND_NOTOPMOST, HWND_TOPMOST, SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOSIZE, SWP_SHOWWINDOW,
		SW_MINIMIZE, SW_RESTORE,
	},
};
// self
use super::*;

impl Os {
	pub fn obtain_window(&mut self) {
		let hwnd = unsafe {
			winuser::FindWindowW(
				ptr::null_mut(),
				OsStr::new("AiR").encode_wide().chain(Some(0)).collect::<Vec<_>>().as_ptr(),
			)
		};

		if hwnd.is_null() {
			panic!("window must be found");
		} else {
			self.window = Some(hwnd);
		}
	}

	pub fn hide(&self) {
		unsafe {
			winuser::ShowWindowAsync(self.window(), SW_MINIMIZE);
		}
	}

	pub fn unhide(&self) {
		unsafe {
			winuser::ShowWindowAsync(self.window(), SW_RESTORE);
		}
	}

	pub fn stick_to_top(&self) {
		unsafe {
			winuser::SetWindowPos(
				self.window(),
				HWND_TOPMOST,
				0,
				0,
				0,
				0,
				SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE | SWP_SHOWWINDOW,
			);
		}
	}

	pub fn unstick_to_top(&self) {
		unsafe {
			winuser::SetWindowPos(
				self.window(),
				HWND_NOTOPMOST,
				0,
				0,
				0,
				0,
				SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE | SWP_SHOWWINDOW,
			);
		}
	}

	fn window(&self) -> *mut HWND__ {
		*self.window.as_ref().expect("window must be found")
	}
}
