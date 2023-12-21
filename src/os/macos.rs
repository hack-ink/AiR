// crates.io
use accessibility::{AXAttribute, AXUIElement};
use accessibility_sys::{kAXFocusedUIElementAttribute, kAXSelectedTextAttribute};
use core_foundation::{base::CFType, string::CFString};
use icrate::AppKit::NSApplication;

pub(crate) fn hide_application() {
	unsafe {
		NSApplication::sharedApplication().hide(None);
	}
}

pub(crate) fn activate_application() {
	unsafe {
		NSApplication::sharedApplication().activateIgnoringOtherApps(true);
	}
}

pub(crate) fn get_selected_text() -> Option<String> {
	fn attribute(attr: &'static str) -> AXAttribute<CFType> {
		AXAttribute::new(&CFString::from_static_string(attr))
	}

	let ui = AXUIElement::system_wide()
		.attribute(&attribute(kAXFocusedUIElementAttribute))
		.ok()?
		.downcast_into::<AXUIElement>()?;
	let text =
		ui.attribute(&attribute(kAXSelectedTextAttribute)).ok()?.downcast_into::<CFString>()?;

	Some(text.to_string())
}
