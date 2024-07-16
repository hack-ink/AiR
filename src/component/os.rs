#[cfg(target_os = "macos")] mod macos;
#[cfg(all(unix, not(target_os = "macos")))] mod unix;
#[cfg(target_os = "windows")] mod windows;

#[derive(Debug)]
pub struct Os {
	#[cfg(target_os = "macos")]
	app: objc2::rc::Retained<objc2_app_kit::NSRunningApplication>,
	#[cfg(target_os = "macos")]
	window: Option<objc2::rc::Retained<objc2_app_kit::NSWindow>>,
	#[cfg(target_os = "windows")]
	window: Option<*mut winapi::shared::windef::HWND__>,
}
impl Os {
	pub fn new() -> Self {
		Self {
			#[cfg(target_os = "macos")]
			app: Self::get_app(),
			#[cfg(target_os = "macos")]
			window: None,
			#[cfg(target_os = "windows")]
			window: None,
		}
	}
}
