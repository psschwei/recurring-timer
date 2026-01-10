use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::io::Cursor;

const CHIME_AUDIO: &[u8] = include_bytes!("../assets/chime.wav");

pub struct AudioPlayer {
    _stream: Option<OutputStream>,
    stream_handle: Option<OutputStreamHandle>,
}

impl AudioPlayer {
    pub fn new() -> Self {
        // In test mode, skip audio initialization to avoid platform-specific issues (especially Windows CI)
        #[cfg(test)]
        {
            return Self {
                _stream: None,
                stream_handle: None,
            };
        }

        // In production, try to create audio output, but don't panic if it fails (e.g., in CI environments)
        #[cfg(not(test))]
        {
            let (stream, stream_handle) = match OutputStream::try_default() {
                Ok((s, h)) => (Some(s), Some(h)),
                Err(_) => (None, None),
            };

            Self {
                _stream: stream,
                stream_handle,
            }
        }
    }

    pub fn play_chime(&self) {
        // Only attempt to play if we have a valid audio stream
        if let Some(handle) = &self.stream_handle {
            let cursor = Cursor::new(CHIME_AUDIO);

            if let Ok(source) = Decoder::new(cursor) {
                if let Ok(sink) = Sink::try_new(handle) {
                    sink.append(source);
                    sink.detach();
                }
            }
        }
    }
}
