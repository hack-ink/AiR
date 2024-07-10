#[cfg(target_os = "macos")] mod macos;
#[cfg(all(unix, not(target_os = "macos")))] mod unix;
#[cfg(target_os = "windows")] mod windows;

#[derive(Debug)]
pub struct Os;
