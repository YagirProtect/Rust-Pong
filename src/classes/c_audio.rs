use rodio::source::{SineWave, Source};
use std::time::Duration;

pub struct AudioContext {
    stream: rodio::OutputStream,
}

impl AudioContext {
    pub fn new() -> Self {
        let stream = rodio::OutputStreamBuilder::open_default_stream()
            .expect("open default audio stream");
        Self { stream }
    }

    pub fn beep(&self, freq_hz: f32, ms: u64, volume: f32) {
        let sink = rodio::Sink::connect_new(&self.stream.mixer());
        sink.set_volume(volume);

        let src = SineWave::new(freq_hz)
            .take_duration(Duration::from_millis(ms))
            .amplify(0.20);

        sink.append(src);
        sink.detach();
    }

    pub fn beep_button(&self) {
        self.beep(200.0, 80, 0.8);;
    }


    pub fn beep_ball(&self) {
        self.beep(500.0, 80, 0.8);;
    }

    pub fn beep_win(&self) {
        self.beep(700.0, 80, 0.8);;
    }

    pub fn beep_lose(&self) {
        self.beep(300.0, 80, 0.8);;
    }
}
