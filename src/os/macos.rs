// crates.io
use accessibility::{AXAttribute, AXUIElement};
use accessibility_sys::{kAXFocusedUIElementAttribute, kAXSelectedTextAttribute};
use core_foundation::{base::CFType, string::CFString};
use icrate::AppKit::NSApplication;
// air
use super::*;

impl Accessibility for Os {
	fn hide_application() {
		unsafe {
			NSApplication::sharedApplication().hide(None);
		}
	}

	fn activate_application() {
		unsafe {
			NSApplication::sharedApplication().activateIgnoringOtherApps(true);
		}
	}

	fn get_selected_text() -> Option<String> {
		fn attribute(attr: &'static str) -> AXAttribute<CFType> {
			AXAttribute::new(&CFString::from_static_string(attr))
		}

		match AXUIElement::system_wide().attribute(&attribute(kAXFocusedUIElementAttribute)) {
			Ok(ui) => {
				match ui
					.downcast_into::<AXUIElement>()?
					.attribute(&attribute(kAXSelectedTextAttribute))
				{
					Ok(text) => return Some(text.downcast_into::<CFString>()?.to_string()),
					Err(e) => tracing::error!("get `kAXSelectedTextAttribute` returns `{e}`"),
				}
			},
			Err(e) => tracing::error!("get `kAXFocusedUIElementAttribute` returns `{e}`"),
		}

		None
	}
}
