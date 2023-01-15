//! Effects

/// Mutates provided bytes
pub trait Mosh {
    fn glitch(&self, value: &mut [u8]);
}

/// Chunk mutations
pub enum MoshChunk {
    ChannelSwap(usize, usize, usize),
    Flip,
}

/// Line mutations within a chunk
pub enum MoshLine {
    ChannelShift(usize, usize, usize),
    Shift(usize),
    Reverse,
}

impl Mosh for MoshChunk {
    fn glitch(&self, chunk: &mut [u8]) {
        match self {
            Self::ChannelSwap(channel_1, channel_2, channel_count) => {
                let chunk_length = chunk.len();
                let channel_value_count = chunk_length / channel_count;

                for i in 0..channel_value_count {
                    let channel_1_index = (i * channel_count) + channel_1;
                    let channel_2_index = (i * channel_count) + channel_2;

                    chunk.swap(channel_1_index, channel_2_index);
                }
            }

            Self::Flip => chunk.reverse(),
        }
    }
}

impl Mosh for MoshLine {
    fn glitch(&self, line: &mut [u8]) {
        match self {
            Self::ChannelShift(amount, channel, channel_count) => {
                let line_length = line.len();
                let channel_value_count = line_length / channel_count;

                for i in 0..channel_value_count {
                    let current_index = (i * channel_count + channel) % line_length;
                    let target_index =
                        (i * channel_count + channel + (channel + 1) * amount) % line_length;

                    line.swap(current_index, target_index);
                }
            }

            Self::Shift(amount) => line.rotate_left(*amount),
            Self::Reverse => line.reverse(),
        }
    }
}
