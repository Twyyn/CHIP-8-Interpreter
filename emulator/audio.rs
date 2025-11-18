use rodio::{OutputStream, OutputStreamBuilder, Sink, source::SineWave};

pub struct Audio {
    _stream_handle: OutputStream,
    sink: Sink,
}

impl Audio {
    pub fn new() -> Self {
        let _stream_handle =
            OutputStreamBuilder::open_default_stream().expect("Open Default Audio Stream");
        let sink = rodio::Sink::connect_new(&_stream_handle.mixer());

        Self {
            _stream_handle,
            sink,
        }
    }
    pub fn play_beep(&self) {
        let beep = SineWave::new(440 as f32);
        self.sink.append(beep);
        self.sink.play();
    }
    pub fn stop_beep(&self) {
        self.sink.stop();
    }
}
