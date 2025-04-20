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

pub type Grass1RoundFn = fn(Block, RoundKey) -> Block;
pub type Grass1Cipher<K> = FeistelCipher<K, RoundKey, Grass1RoundFn, U16>;
pub type Grass1Encrypt = Grass1Cipher<TeaKeySchedule>;
pub type Grass1Decrypt = Grass1Cipher<ReverseKeySchedule<RoundKey>>;

pub const ROUNDS: usize = 4;

pub fn encrypt(plaintext: Plaintext, key: Key) -> Grass1Encrypt {
    FeistelCipher::new(
        ByteArray::from_array(plaintext),
        TeaKeySchedule::new(key),
        round,
        ROUNDS,
    )
}

pub fn decrypt(plaintext: Plaintext, key: Key) -> Grass1Decrypt {
    FeistelCipher::new(
        ByteArray::from_array(plaintext),
        ReverseKeySchedule::new(TeaKeySchedule::new(key), ROUNDS),
        round,
        ROUNDS,
    )
}

pub fn round(block: Block, round_key: RoundKey) -> Block {
    ByteArray::from_array([0x00; 16])
}

#[test]
fn grass_1() {
    let plaintext = b"The secret phrase is 'befuddle'!";
    let key = [0; 16];
    let mut cipher = cipher(*plaintext, key);
    cipher.dump();
    cipher.run_rounds(8);
}
