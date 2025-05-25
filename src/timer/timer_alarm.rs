use anyhow::Result;
use rodio::{Decoder, OutputStream, Sink};
use std::{fs::File, io::BufReader};

pub struct TimerAlarm {}

impl TimerAlarm {
    pub fn play() -> Result<()> {
        // Set up audio output
        let (_stream, stream_handle) = OutputStream::try_default()?;

        // Create a sink
        let sink = Sink::try_new(&stream_handle)?;

        // Open the WAV file
        let file = File::open("alarm.mp3")?;
        let reader = BufReader::new(file);

        // Decode the WAV file
        let source = Decoder::new(reader)?;

        // Play the sound
        sink.append(source);

        // Optional: Wait until the sound finishes
        sink.sleep_until_end();

        Ok(())
    }
}
