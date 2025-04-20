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
pub struct ReverseKeySchedule<K: Debug> {
    sequence: Vec<K>,
}

impl<K: Debug> ReverseKeySchedule<K> {
    pub fn new<KS>(mut key_schedule: KS, round_count: usize) -> Self
    where
        KS: KeySchedule<K>,
    {
        let mut sequence = Vec::new();

        for _ in 0..round_count {
            sequence.push(key_schedule.next_key());
        }

        ReverseKeySchedule { sequence }
    }
}

impl<K: Debug> KeySchedule<K> for ReverseKeySchedule<K> {
    fn next_key(&mut self) -> K {
        self.sequence
            .pop()
            .expect("Attempted to request more keys than prepared")
    }
}
