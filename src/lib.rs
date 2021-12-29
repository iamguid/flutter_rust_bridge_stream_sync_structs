use anyhow::{Result};
use flutter_rust_bridge::{StreamSink};

pub struct MidiMessage {
    message: u8,
    stamp: u32,
}

pub fn stream_sink_with_struct(sink: StreamSink<MidiMessage>) -> Result<()> {
    sink.add({
        message: 1,
        stamp: 2,
    });

    Ok(())
}
