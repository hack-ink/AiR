// TODO: Some errors are not used since we use `.unwrap()`.

#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error(transparent)]
	Io(#[from] std::io::Error),

	#[error(transparent)]
	AppDirs2(#[from] app_dirs2::AppDirsError),
	#[error(transparent)]
	Arboard(#[from] arboard::Error),
	#[error(transparent)]
	Eframe(#[from] eframe::Error),
	#[error(transparent)]
	OpenAi(#[from] async_openai::error::OpenAIError),
	#[error(transparent)]
	Reqwew(#[from] reqwew::error::Error),
	#[error(transparent)]
	Toml(#[from] toml::de::Error),

	#[error(transparent)]
	Enigo(#[from] EnigoError),
	#[error(transparent)]
	GlobalHotKey(#[from] GlobalHotKeyError),
	#[error("unsupported key: {0}")]
	UnsupportedKey(String)
}

#[derive(Debug, thiserror::Error)]
pub enum EnigoError {
	#[error(transparent)]
	Input(#[from] enigo::InputError),
	#[error(transparent)]
	NewCon(#[from] enigo::NewConError),
}

#[derive(Debug, thiserror::Error)]
pub enum GlobalHotKeyError {
	#[error(transparent)]
	Main(#[from] global_hotkey::Error),
	#[error(transparent)]
	Parse(#[from] global_hotkey::hotkey::HotKeyParseError),
}
