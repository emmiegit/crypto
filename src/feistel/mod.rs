pub mod grass_1;

use super::key_schedule::KeySchedule;
use crate::{types::ByteArray, utils::xor};
use generic_array::{
    ArrayLength,
    typenum::{Prod, U2},
};
use std::ops::Mul;

#[derive(Debug)]
pub struct FeistelCipher<K, R, N: ArrayLength> {
    left: ByteArray<N>,
    right: ByteArray<N>,
    key_schedule: K,
    round_function: R,
}

impl<K, R, N> FeistelCipher<K, R, N>
where
    N: ArrayLength + Mul<U2>,
    Prod<N, U2>: ArrayLength,
{
    pub fn new(plaintext: ByteArray<Prod<N, U2>>, key_schedule: K, round_function: R) -> Self {
        let half = plaintext.len() / 2;

        FeistelCipher {
            key_schedule,
            round_function,
            left: ByteArray::from_slice(&plaintext[..half]).clone(),
            right: ByteArray::from_slice(&plaintext[half..]).clone(),
        }
    }

    /// Crudely prints the internal state of the cipher for testing purposes.
    #[cfg(test)]
    pub fn dump(&self) {
        println!(
            "'{}{}'",
            String::from_utf8_lossy(self.left.as_slice()),
            String::from_utf8_lossy(self.right.as_slice()),
        );
    }
}

impl<K, R, N> FeistelCipher<K, R, N>
where
    K: KeySchedule,
    R: FnMut(ByteArray<N>, K::SubKey) -> ByteArray<N>,
    N: ArrayLength,
{
    pub fn round(&mut self) {
        let round_key = self.key_schedule.next_key();
        let mask = (self.round_function)(self.right.clone(), round_key);
        let new_left = self.right.clone();
        let new_right = xor(self.left.clone(), mask);
        self.left = new_left;
        self.right = new_right;
    }
}
