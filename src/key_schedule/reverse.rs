use super::KeySchedule;
use std::fmt::Debug;

/// Wrapper struct which enables running a key schedule in reverse.
///
/// Since for a cipher, the number of rounds is known in advance,
/// we can just pre-run the key schedule to generate _round_ keys,
/// and then return them in reverse order as needed.
///
/// This allows us to trivially implement decryption for ciphers
/// where the process is simply running the same (or similar)
/// algorithm but with the round keys in reverse order.
#[derive(Debug)]
pub struct ReverseKeySchedule<B: Debug> {
    sequence: Vec<B>,
}

impl<B: Debug> ReverseKeySchedule<B> {
    pub fn new<K>(mut key_schedule: K, round_count: usize) -> Self
    where
        K: KeySchedule<SubKey = B>,
    {
        let mut sequence = Vec::new();

        for _ in 0..round_count {
            sequence.push(key_schedule.next_key());
        }

        ReverseKeySchedule { sequence }
    }
}

impl<B: Debug> KeySchedule for ReverseKeySchedule<B> {
    type SubKey = B;

    fn next_key(&mut self) -> B {
        self.sequence
            .pop()
            .expect("Attempted to request more keys than prepared")
    }
}
