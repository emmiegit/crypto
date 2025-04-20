pub mod grass_0;
pub mod grass_1;

use super::key_schedule::KeySchedule;
use crate::{types::ByteArray, utils::xor};
use generic_array::{
    ArrayLength,
    typenum::{Prod, U2},
};
use std::marker::PhantomData;
use std::ops::Mul;

#[derive(Debug)]
pub struct FeistelCipher<KS, K, R, N: ArrayLength> {
    left: ByteArray<N>,
    right: ByteArray<N>,
    key_schedule: KS,
    round_function: R,
    rounds: usize,
    _round_key: PhantomData<K>,
}

impl<KS, K, R, N> FeistelCipher<KS, K, R, N>
where
    KS: KeySchedule<K>,
    N: ArrayLength + Mul<U2>,
    Prod<N, U2>: ArrayLength,
{
    pub fn new(
        plaintext: ByteArray<Prod<N, U2>>,
        key_schedule: KS,
        round_function: R,
        rounds: usize,
    ) -> Self {
        let half = plaintext.len() / 2;

        FeistelCipher {
            left: ByteArray::from_slice(&plaintext[..half]).clone(),
            right: ByteArray::from_slice(&plaintext[half..]).clone(),
            key_schedule,
            round_function,
            rounds,
            _round_key: PhantomData,
        }
    }
}

impl<KS, K, R, N> FeistelCipher<KS, K, R, N>
where
    KS: KeySchedule<K>,
    R: FnMut(ByteArray<N>, K) -> ByteArray<N>,
    N: ArrayLength,
{
    pub fn round(&mut self) {
        let round_key = self.key_schedule.next_key();
        let mask = (self.round_function)(self.right.clone(), round_key);
        let new_left = self.right.clone();
        let new_right = xor(self.left.clone(), mask);
        self.left = new_left;
        self.right = new_right;
        self.rounds -= 1;
    }

    pub fn run_rounds(&mut self) {
        for _ in 0..self.rounds {
            self.round();

            #[cfg(test)]
            self.dump();
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
