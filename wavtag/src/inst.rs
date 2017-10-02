use { RiffChunk, ChunkType, RiffFile };

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InstrumentChunk {
    /// The unshifted note field has the same meaning as the sampler chunk's MIDI Unity Note which specifies the
    /// musical note at which the sample will be played at it's original sample rate (the sample rate specified
    /// in the format chunk). (0-127)
    pub unshifted_note: u8,

    /// Fine Tune (dB)
    /// The fine tune value specifies how much the sample's pitch should be altered when the sound is played back
    /// in cents (1/100 of a semitone). A negative value means that the pitch should be played lower and a positive
    /// value means that it should be played at a higher pitch.
    pub fine_tune: u8, // -50 - +50

    /// The gain value specifies the number of decibels to adjust the output when it is played. A value of 0dB
    /// means no change, 6dB means double the amplitude of each sample and -6dB means to halve the amplitude of
    /// each sample. Every additional +/-6dB will double or halve the amplitude again.
    pub gain: u8, // -64 - +64
    pub low_note: u8,
    pub high_note: u8,
    pub low_vel: u8,
    pub high_vel: u8,
}

impl Default for InstrumentChunk {
    fn default() -> Self {
        InstrumentChunk {
            unshifted_note: 0,
            fine_tune: 0,
            gain: 0,
            low_note: 0,
            high_note: 0,
            low_vel: 0,
            high_vel: 0,
        }
    }
}

impl InstrumentChunk {
    // pub fn from_chunk(chunk: RiffChunk) -> InstrumentChunk {}

    pub fn serialise(&self) -> Vec<u8> {
        vec![
            self.unshifted_note,
            self.fine_tune,
            self.gain,
            self.low_note,
            self.high_note,
            self.low_vel,
            self.high_vel,
            0   // zero padding to 8 bytes
        ]
    }
}

impl RiffFile {
    pub fn get_instrument_chunk(&self) -> InstrumentChunk {
        match self.find_chunk_by_type(ChunkType::Instrument) {
            _ => InstrumentChunk::default(),
        }
    }

    pub fn set_instrument_chunk(&mut self, chunk: InstrumentChunk) {}
}