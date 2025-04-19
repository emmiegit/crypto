//! Garbage cipher with a round function of all zeroes.
//! Effectively, all this does is cycle the two halves back and forth.

use super::FeistelCipher;
use crate::{key_schedule::TeaKeySchedule, types::ByteArray};
use generic_array::typenum::U16;

pub type Plaintext = [u8; 32];
pub type Key = [u8; 16];

pub type Block = ByteArray<U16>;
pub type RoundKey = [u8; 4];

pub type Grass0RoundFn = fn(Block, RoundKey) -> Block;
pub type Grass0Cipher = FeistelCipher<TeaKeySchedule, Grass0RoundFn, U16>;

pub fn cipher(plaintext: Plaintext, key: Key) -> Grass0Cipher {
    FeistelCipher::new(
        ByteArray::from_array(plaintext),
        TeaKeySchedule::new(key),
        round,
    )
}

pub fn round(block: Block, round_key: RoundKey) -> Block {
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
