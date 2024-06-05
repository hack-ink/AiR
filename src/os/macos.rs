// crates.io
use accessibility::{AXAttribute, AXUIElement, AXUIElementAttributes};
use accessibility_sys::{
	kAXFocusedUIElementAttribute, kAXFocusedWindowAttribute, kAXSelectedTextAttribute,
};
use core_foundation::{base::CFType, string::CFString};
use objc2_app_kit::{NSRunningApplication, NSWorkspace};
// self
use super::*;

impl Accessibility for Os {
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

	fn selected_text() -> Option<String> {
		let pid = unsafe {
			let workspace = NSWorkspace::sharedWorkspace();
			let app = workspace.frontmostApplication().unwrap();
			dbg!(&app);

			app.processIdentifier()
		};

		fn attr(attr: &'static str) -> AXAttribute<CFType> {
			AXAttribute::new(&CFString::from_static_string(attr))
		}

		fn try_get_focus_element(ax_ui_element: &AXUIElement) -> Option<AXUIElement> {
			if let Ok(e) = ax_ui_element.attribute(&attr(kAXFocusedUIElementAttribute)) {
				return e.downcast_into();
			}

			ax_ui_element
				.children()
				.ok()
				.and_then(|es| es.iter().find_map(|e| try_get_focus_element(&e)))
		}

		let root = AXUIElement::application(pid);
		dbg!(&root);

		let window = root
			.attribute(&attr(kAXFocusedWindowAttribute))
			.unwrap()
			.downcast_into::<AXUIElement>()
			.unwrap();
		dbg!(&window);

		let element = try_get_focus_element(&window);
		dbg!(element);

		// fn attribute(attr: &'static str) -> AXAttribute<CFType> {
		// 	AXAttribute::new(&CFString::from_static_string(attr))
		// }

		// match AXUIElement::system_wide().attribute(&attribute(kAXFocusedUIElementAttribute)) {
		// 	Ok(ui) => {
		// 		match ui
		// 			.downcast_into::<AXUIElement>()?
		// 			.attribute(&attribute(kAXSelectedTextAttribute))
		// 		{
		// 			Ok(text) => return Some(text.downcast_into::<CFString>()?.to_string()),
		// 			Err(e) => tracing::error!("get `kAXSelectedTextAttribute` returns `{e}`"),
		// 		}
		// 	},
		// 	Err(e) => tracing::error!("get `kAXFocusedUIElementAttribute` returns `{e}`"),
		// }

		None
	}
}
