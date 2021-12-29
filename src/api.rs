use anyhow::{Result};
use flutter_rust_bridge::{StreamSink};

pub struct MidiMessage {
    pub message: u8,
    pub stamp: u32,
}

pub fn stream_sink_with_struct(sink: StreamSink<MidiMessage>) -> Result<()> {
    sink.add(MidiMessage {
        message: 0,
        stamp: 0,
    });

    Ok(())
}
