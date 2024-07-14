// crates.io
use objc2::rc::Retained;
use objc2_app_kit::{NSApplication, NSRunningApplication, NSWindowCollectionBehavior};
use objc2_foundation::MainThreadMarker;
// self
use super::*;

impl Os {
	pub fn get_ca() -> Retained<NSRunningApplication> {
		unsafe { NSRunningApplication::currentApplication() }
	}

	pub fn hide(&self) {
		unsafe {
			self.ca.hide();
		}
	}

	pub fn unhide(&self) {
		unsafe {
			self.ca.unhide();
		}
	}

	pub fn set_move_to_active_space() {
		unsafe {
			// // crates.io
			// use objc2::AnyObject;
			//
			// let app: *mut AnyObject =
			// 	objc2::msg_send![objc2::class!(NSApplication), sharedApplication];
			// let window: *mut AnyObject = objc2::msg_send![app, mainWindow];
			// let _: () = objc2::msg_send![window, setCollectionBehavior: 1_u64<<1];

			NSApplication::sharedApplication(MainThreadMarker::new_unchecked())
				.mainWindow()
				.expect("main window must be found")
				.setCollectionBehavior(NSWindowCollectionBehavior::MoveToActiveSpace);
		}
	}
}
