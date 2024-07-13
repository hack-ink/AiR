#[cfg(target_os = "macos")] mod macos;
#[cfg(all(unix, not(target_os = "macos")))] mod unix;
#[cfg(target_os = "windows")] mod windows;

#[derive(Debug)]
pub struct Os {
	#[cfg(target_os = "macos")]
	nsra: objc2::rc::Retained<objc2_app_kit::NSRunningApplication>,
	#[cfg(target_os = "windows")]
	hwnd: *mut winapi::shared::windef::HWND__,
}
impl Os {
	pub fn new() -> Self {
		Self {
			#[cfg(target_os = "macos")]
			nsra: Self::get_nsra(),
			#[cfg(target_os = "windows")]
			hwnd: Self::get_hwnd(),
		}
	}
}
