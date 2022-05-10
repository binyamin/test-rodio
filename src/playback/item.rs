use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder};

// pub struct PlaybackItem {
// 	pub source: Decoder<BufReader<File>>,
// }

pub fn get_source(file_path: &str) -> Decoder<BufReader<File>> {
	// Load a sound from a file, using a path relative to Cargo.toml
	let file = BufReader::new(File::open(file_path).unwrap());

	// Decode that sound file into a source
	Decoder::new(file).unwrap()
}

// impl PlaybackItem {
// 	pub fn new(file_path: &str) -> Self {


// 		Self {
// 			source: source,
// 		}
// 	}
// }
