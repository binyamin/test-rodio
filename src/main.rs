// use std::fs::File;
// use std::io::BufReader;
use rodio::{OutputStream, Sink};

mod playback;

const SONG_FILE: &str = "/home/binyamin/Music/library/Ari Goldwag/A_Cappella_Soul_4-02-Ve'ahavta.mp3";

// This code works
// Note - Copied from Rodio docs.
fn case_a() {
	let (_stream, stream_handle) = OutputStream::try_default().unwrap();
	let sink = Sink::try_new(&stream_handle).unwrap();

	// Add a dummy source of the sake of the example.
	let source = playback::item::get_source(SONG_FILE);
	sink.append(source);

	// The sound plays in a separate thread. This call will block the current thread until the sink
	// has finished playing all its queued sounds.
	sink.sleep_until_end();
}

// This code doesn't
fn case_b() {
	eprintln!("Starting...");
	let p = playback::Player::new();
	eprintln!("Loading...");
	p.load_and_play(SONG_FILE);
	// thread::sleep(Duration::from_secs(5));
	eprintln!("Playing...");
	p.sink.lock().unwrap().sleep_until_end();
	eprintln!("Exiting...");
}

fn main() {
	// case_a();
	case_b();
}
