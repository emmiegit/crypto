pub mod grass_0;
pub mod grass_1;

use super::key_schedule::KeySchedule;
use crate::{types::ByteArray, utils::xor};
use generic_array::{
    ArrayLength,
    typenum::{Prod, U2},
};
use std::{marker::PhantomData, mem, ops::Mul};

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

    pub fn result(&self) -> ByteArray<Prod<N, U2>> {
        let mut ciphertext = ByteArray::default();
        let half = ciphertext.len() / 2;
        (&mut ciphertext[..half]).copy_from_slice(&self.left);
        (&mut ciphertext[half..]).copy_from_slice(&self.right);
        ciphertext
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

    pub fn flip(&mut self) {
        mem::swap(&mut self.left, &mut self.right);

        #[cfg(test)]
        self.dump();
    }

    pub fn run(&mut self) {
        for _ in 0..self.rounds {
            self.round();

            #[cfg(test)]
            self.dump();
        }
    }

    /// Crudely prints the internal state of the cipher for testing purposes.
    #[cfg(test)]
    pub fn dump(&self) {
        fn print_bytes(bytes: &[u8]) {
            fn get_char(b: u8) -> Option<char> {
                match b {
                    // Printable range for ASCII
                    0x20..0x7e => char::from_u32(u32::from(b)), // Should always be Some(), but we can avoid a rewrap here.
                    _ => None,
                }
            }

            for b in bytes.iter().copied() {
                let c = get_char(b).unwrap_or('.');
                print!("{c}");
            }
        }

        fn print_hex(bytes: &[u8]) {
            for b in bytes.iter().copied() {
                print!("{b:02x}");
            }
        }

        print!("\"");
        print_bytes(self.left.as_slice());
        print_bytes(self.right.as_slice());
        print!("\" ");
        print_hex(self.left.as_slice());
        print!(" ");
        print_hex(self.right.as_slice());
        println!();
    }
}
