use std::time::Duration;

use gbemulator::system::sound_controller::{ToneNSweep, WhiteNoise};
use rodio::source::{SineWave, Source};
use rodio::{Decoder, OutputStream, Sink};

#[test]
fn test_sound_1() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    let sound1048Hz = ToneNSweep::new(Duration::ZERO, true, 0, 2, None, 15, true, 3, 1048, true);
    sink.append(sound1048Hz.amplify(7 as f32));
    std::thread::sleep(std::time::Duration::from_millis(1000));
    sink.clear();
    let sound2080Hz = ToneNSweep::new(Duration::ZERO, true, 0, 2, None, 15, true, 3, 2080, true);
    sink.append(sound2080Hz.amplify(7 as f32));
    sink.play();
    std::thread::sleep(std::time::Duration::from_millis(1000));
    sink.clear();
}

#[test]
fn test_white_noise() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    let white_noise = WhiteNoise::new(None, 15, true, 15, false, 2080.0f32, true);
    sink.append(white_noise.amplify(7 as f32));
    sink.play();
    std::thread::sleep(std::time::Duration::from_millis(2000));
    sink.clear();
    std::thread::sleep(std::time::Duration::from_millis(1000));
    let white_noise = WhiteNoise::new(None, 15, true, 15, true, 2080.0f32, true);
    sink.append(white_noise.amplify(7 as f32));
    sink.play();
    std::thread::sleep(std::time::Duration::from_millis(2000));
}
