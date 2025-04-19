use super::FeistelCipher;
use crate::{key_schedule::TeaKeySchedule, types::ByteArray};
use generic_array::typenum::U16;

pub type Plaintext = [u8; 32];
pub type Key = [u8; 16];

pub type Block = ByteArray<U16>;
pub type RoundKey = [u8; 4];

pub type Grass1RoundFn = fn(Block, RoundKey) -> Block;
pub type Grass1Cipher = FeistelCipher<TeaKeySchedule, Grass1RoundFn, U16>;

pub fn cipher(plaintext: Plaintext, key: Key) -> Grass1Cipher {
    FeistelCipher::new(
        ByteArray::from_array(plaintext),
        TeaKeySchedule::new(key),
        round,
    )
}

pub fn round(block: Block, round_key: RoundKey) -> Block {
    [0x01; 16].into()
}

#[test]
fn grass_1() {
    let plaintext = b"The secret phrase is 'befuddle'!";
    let key = [0; 16];
    let mut cipher = cipher(*plaintext, key);
    cipher.dump();

    cipher.round();
    cipher.dump();

    cipher.round();
    cipher.dump();
}
