// crates.io
use objc2::rc::Retained;
use objc2_app_kit::{
	NSApplication, NSFloatingWindowLevel, NSNormalWindowLevel, NSRunningApplication, NSWindow,
	NSWindowCollectionBehavior,
};
use objc2_foundation::MainThreadMarker;
// self
use super::*;

impl Os {
	pub fn get_app() -> Retained<NSRunningApplication> {
		unsafe { NSRunningApplication::currentApplication() }
	}

	pub fn obtain_window(&mut self) {
		self.window = unsafe {
			Some(
				NSApplication::sharedApplication(MainThreadMarker::new_unchecked())
					.mainWindow()
					.expect("window must be found"),
			)
		};
	}

	pub fn set_move_to_active_space(&self) {
		unsafe {
			// // crates.io
			// use objc2::AnyObject;
			//
			// let app: *mut AnyObject =
			// 	objc2::msg_send![objc2::class!(NSApplication), sharedApplication];
			// let window: *mut AnyObject = objc2::msg_send![app, mainWindow];
			// let _: () = objc2::msg_send![window, setCollectionBehavior: 1_u64<<1];

			self.window().setCollectionBehavior(NSWindowCollectionBehavior::MoveToActiveSpace);
		}
	}

	pub fn hide(&self) {
		unsafe {
			self.app.hide();
		}
	}

	pub fn unhide(&self) {
		unsafe {
			self.app.unhide();
		}
	}

	pub fn stick_to_top(&self) {
		self.window().setLevel(NSFloatingWindowLevel);
	}

	pub fn unstick_to_top(&self) {
		self.window().setLevel(NSNormalWindowLevel);
	}

	fn window(&self) -> &Retained<NSWindow> {
		self.window.as_ref().expect("window must be found")
	}
}
