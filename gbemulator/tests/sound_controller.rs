use std::time::Duration;

use gbemulator::system::sound_controller::ToneNSweep;
use rodio::source::{SineWave, Source};
use rodio::{Decoder, OutputStream, Sink};

#[test]
fn test_sound_1() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    let sound1048Hz = ToneNSweep::new(
        Duration::ZERO,
        true,
        0,
        2,
        Some(Duration::from_nanos(250000000)),
        15,
        true,
        3,
        1048,
        true,
        false,
    );
    sink.append(sound1048Hz.amplify(7 as f32));
    std::thread::sleep(std::time::Duration::from_millis(1000));
    sink.clear();
    let sound2080Hz = ToneNSweep::new(
        Duration::ZERO,
        true,
        0,
        2,
        Some(Duration::from_nanos(250000000)),
        15,
        true,
        3,
        2080,
        true,
        false,
    );
    sink.append(sound2080Hz.amplify(7 as f32));
    sink.play();
    std::thread::sleep(std::time::Duration::from_millis(1000));
    sink.clear();
}
