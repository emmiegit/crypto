//! Garbage cipher with a round function of all zeroes.
//! Effectively, all this does is cycle the two halves back and forth.

use super::FeistelCipher;
use crate::{
    key_schedule::{ReverseKeySchedule, TeaKeySchedule},
    types::ByteArray,
};
use generic_array::typenum::U16;

pub type Plaintext = [u8; 32];
pub type Key = [u8; 16];

pub type Block = ByteArray<U16>;
pub type RoundKey = [u8; 4];

pub type Grass0RoundFn = fn(Block, RoundKey) -> Block;
pub type Grass0Cipher<K> = FeistelCipher<K, RoundKey, Grass0RoundFn, U16>;
pub type Grass0Encrypt = Grass0Cipher<TeaKeySchedule>;
pub type Grass0Decrypt = Grass0Cipher<ReverseKeySchedule<RoundKey>>;

pub const ROUNDS: usize = 4;

pub fn encrypt(plaintext: Plaintext, key: Key) -> Grass0Encrypt {
    FeistelCipher::new(
        ByteArray::from_array(plaintext),
        TeaKeySchedule::new(key),
        round,
        ROUNDS,
    )
}

pub fn decrypt(plaintext: Plaintext, key: Key) -> Grass0Decrypt {
    FeistelCipher::new(
        ByteArray::from_array(plaintext),
        ReverseKeySchedule::new(TeaKeySchedule::new(key), ROUNDS),
        round,
        ROUNDS,
    )
}

pub fn round(_block: Block, _round_key: RoundKey) -> Block {
    ByteArray::from_array([0x00; 16])
}

#[test]
fn grass_0() {
    let plaintext = b"The secret phrase is 'befuddle'!";
    let key = [0x00; 16];
    let mut cipher = cipher(*plaintext, key);
    cipher.dump();
    cipher.run_rounds(8);
}
