extern crate sdl2;
extern crate snes_spc;

use snes_spc::SnesSpc;
use sdl2::audio::{AudioCallback, AudioSpecDesired};
use std::thread;

struct SpcPlayer(SnesSpc);

impl SpcPlayer {
    pub fn new(filename: &str) -> SpcPlayer {
        SpcPlayer(SnesSpc::from_file(filename).unwrap())
    }
}

impl AudioCallback for SpcPlayer {
    type Channel = i16;
    fn callback(&mut self, out: &mut [i16]) {
        self.0.play(out).unwrap();
    }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let sdl_audio = sdl_context.audio().unwrap();

    // Mmmh, classic SNES 32KHz stereo. Nostalgiariffic!
    let desired_spec = AudioSpecDesired {
        freq: Some(32000),
        channels: Some(2),
        samples: None,
    };

    let audio = sdl_audio.open_playback(None, &desired_spec, |spec| {
        println!("Audio initialized: {:?}", spec);
        let mut spc = SpcPlayer::new("examples/surprise.spc");

        // This is an example of an SPC dump with a dirty echo buffer;
        // try commenting out this line and listen to the hot garbage
        // at the beginning of playback!
        spc.0.clear_echo();

        spc
    }).unwrap();

    audio.resume();

    // Loop until terminated!
    loop {
        thread::yield_now();
    }
}
