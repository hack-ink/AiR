// crates.io
// use accessibility::{AXAttribute, AXUIElement, AXUIElementAttributes};
// use accessibility_sys::{
// 	kAXFocusedUIElementAttribute, kAXFocusedWindowAttribute, kAXSelectedTextAttribute,
// };
// use core_foundation::{
// 	base::{CFType, ToVoid},
// 	string::CFString,
// };
use objc2_app_kit::{NSApplication, NSRunningApplication, NSWindowCollectionBehavior};
use objc2_foundation::MainThreadMarker;
// self
use super::*;

impl Accessibility for Os {
	// fn selected_text() -> Option<String> {
	// 	fn attr(attr: &'static str) -> AXAttribute<CFType> {
	// 		AXAttribute::new(&CFString::from_static_string(attr))
	// 	}
	//
	// 	fn try_get_focus_element(ax_ui_element: &AXUIElement) -> Option<AXUIElement> {
	// 		if let Ok(e) = ax_ui_element.attribute(&attr(kAXFocusedUIElementAttribute)) {
	// 			return e.downcast_into();
	// 		}
	//
	// 		ax_ui_element
	// 			.children()
	// 			.ok()
	// 			.and_then(|es| es.iter().find_map(|e| try_get_focus_element(&e)))
	// 	}
	//
	// 	let pid = unsafe {
	// 		let workspace = NSWorkspace::sharedWorkspace();
	// 		let app = workspace.frontmostApplication()?;
	//
	// 		app.processIdentifier()
	// 	};
	// 	let root = AXUIElement::application(pid);
	// 	let window = root
	// 		.attribute(&attr(kAXFocusedWindowAttribute))
	// 		.unwrap()
	// 		.downcast_into::<AXUIElement>()?;
	//
	// 	tracing::debug!("window: {window:?}");
	//
	// 	let element = try_get_focus_element(&window)?;
	//
	// 	tracing::debug!("element: {window:?}");
	//
	// 	element
	// 		.attribute(&attr(kAXSelectedTextAttribute))
	// 		.ok()
	// 		.and_then(|t| t.downcast_into::<CFString>())
	// 		.map(|t| t.to_string())
	// }
}

impl AppKit for Os {
	fn hide() {
		unsafe {
			NSRunningApplication::currentApplication().hide();
		}
	}

	fn unhide() {
		unsafe {
			NSRunningApplication::currentApplication().unhide();
		}
	}

	fn set_move_to_active_space() {
		unsafe {
			// let app: *mut AnyObject =
			// 	objc2::msg_send![objc2::class!(NSApplication), sharedApplication];
			// let window: *mut AnyObject = objc2::msg_send![app, mainWindow];
			// let _: () = objc2::msg_send![window, setCollectionBehavior: 1_u64<<1];

			NSApplication::sharedApplication(MainThreadMarker::new_unchecked())
				.mainWindow()
				.expect("no main window")
				.setCollectionBehavior(NSWindowCollectionBehavior::MoveToActiveSpace);
		}
	}
}
