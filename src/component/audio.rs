// std
use std::{
	fmt::{Debug, Formatter, Result as FmtResult},
	io::Cursor,
};
// crates.io
use rodio::{
	source::{Buffered, Source as _},
	Decoder, OutputStream, OutputStreamHandle, Sink,
};
// self
use crate::prelude::*;

type Source = Buffered<Decoder<Cursor<&'static [u8]>>>;

pub struct Audio {
	pub notification: Source,
	sink: Sink,
	// Stream must be kept alive.
	_stream: (OutputStream, OutputStreamHandle),
}
impl Audio {
	pub fn new() -> Result<Self> {
		let sound_data = include_bytes!("../../asset/notification.mp3");
		let cursor = Cursor::new(sound_data.as_ref());
		let decoder = Decoder::new(cursor).map_err(RodioError::Decoder)?;
		let notification = decoder.buffered();
		let _stream = OutputStream::try_default().map_err(RodioError::Stream)?;
		let sink = Sink::try_new(&_stream.1).map_err(RodioError::Play)?;

		Ok(Audio { notification, sink, _stream })
	}

	pub fn play_notification(&self) {
		self.sink.append(self.notification.clone());
		self.sink.sleep_until_end();
	}
}
impl Debug for Audio {
	fn fmt(&self, f: &mut Formatter) -> FmtResult {
		write!(f, "Audio(..)")
	}
}
