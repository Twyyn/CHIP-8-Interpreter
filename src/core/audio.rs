use rodio::{OutputStream, OutputStreamBuilder, Sink, Source};
use std::time::Duration;

#[allow(non_snake_case)]
pub struct Audio {
    stream_handle: OutputStream,
    sink: Sink,
}
#[allow(non_snake_case)]
impl Audio {
    pub fn new() -> Audio {
        let stream_handle =
            OutputStreamBuilder::open_default_stream().expect("Failed to get audio stream");
        let sink = Sink::connect_new(&stream_handle.mixer());
        Audio {
            stream_handle,
            sink,
        }
    }
    pub fn play_beep(&mut self) {
        self.sink.clear();
        let source = rodio::source::SineWave::new(400.0)
            .take_duration(Duration::from_millis(100))
            .fade_out(Duration::from_millis(10));
        self.stream_handle.mixer().add(source);
    }
    pub fn stop_beep(&self) {
        self.sink.clear();
    }
}
