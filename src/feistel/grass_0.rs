//! Garbage cipher with a round function of all zeroes.
//! Effectively, all this does is cycle the two halves back and forth.

use super::FeistelCipher;
use crate::{
    key_schedule::{ReverseKeySchedule, TeaKeySchedule},
    types::{ByteArray, CipherMode},
};
use generic_array::typenum::{U16, U32};

pub type Text = ByteArray<U32>;
pub type Key = [u8; 16];

pub type Block = ByteArray<U16>;
pub type RoundKey = [u8; 4];

pub type Grass0RoundFn = fn(Block, RoundKey) -> Block;
pub type Grass0Cipher<K> = FeistelCipher<K, RoundKey, Grass0RoundFn, U16>;
pub type Grass0Encrypt = Grass0Cipher<TeaKeySchedule>;
pub type Grass0Decrypt = Grass0Cipher<ReverseKeySchedule<RoundKey>>;

pub const ROUNDS: usize = 4;

pub fn encrypt(plaintext: Text, key: Key) -> Grass0Encrypt {
    FeistelCipher::new(
        plaintext,
        TeaKeySchedule::new(key),
        round,
        ROUNDS,
        CipherMode::Encrypt,
    )
}

pub fn decrypt(ciphertext: Text, key: Key) -> Grass0Decrypt {
    FeistelCipher::new(
        ciphertext,
        ReverseKeySchedule::new(TeaKeySchedule::new(key), ROUNDS),
        round,
        ROUNDS,
        CipherMode::Decrypt,
    )
}

pub fn round(_block: Block, _round_key: RoundKey) -> Block {
    ByteArray::from_array([0x00; 16])
}

#[test]
fn grass_0() {
    let plaintext = b"The secret phrase is 'befuddle'!";
    let key = [0x00; 16];

    let plaintext = ByteArray::from_slice(plaintext).clone();
    let mut cipher = encrypt(plaintext, key);
    cipher.dump();
    cipher.run();
    let ciphertext = cipher.result();

    println!(
        "----------------------------------------------------------------------------------------------------"
    );

    let mut cipher = decrypt(ciphertext, key);
    cipher.dump();
    cipher.run();
    let new_plaintext = cipher.result();

    assert_eq!(plaintext, new_plaintext);
}
