use rodio::{OutputStream, Sink};
use std::sync::Mutex;

pub mod item;

pub struct Player {
	pub sink: Mutex<Sink>,
}

impl Default for Player {
	fn default() -> Self {
		let (_stream, stream_handle) = OutputStream::try_default().unwrap();
		let sink = Sink::try_new(&stream_handle).unwrap_or_else(|_| panic!("Failed to create sound channel!"));

		Self {
			sink: Mutex::new(sink),
		}
	}
}

impl Player {
	pub fn new() -> Self {
		Default::default()
	}

	pub fn load_and_play(&self, file: &str) {
		let source = item::get_source(&file);

		let sink = &self.sink.lock().unwrap();
		sink.append(source);
	}
}
