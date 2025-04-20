//! Garbage cipher with substition but no (meaningful) permutations.
//! This has an actual round function, but the output quality is limited.

use super::FeistelCipher;
use crate::{
    aes::S_BOX,
    key_schedule::{ReverseKeySchedule, SimpleRotateKeySchedule},
    types::{ByteArray, CipherMode},
};
use generic_array::typenum::{U16, U32};

pub type Text = ByteArray<U32>;
pub type Key = [u8; 16];

pub type Block = ByteArray<U16>;
pub type RoundKey = [u8; 16];

pub type Grass1RoundFn = fn(Block, RoundKey) -> Block;
pub type Grass1Cipher<K> = FeistelCipher<K, RoundKey, Grass1RoundFn, U16>;
pub type Grass1Encrypt = Grass1Cipher<SimpleRotateKeySchedule>;
pub type Grass1Decrypt = Grass1Cipher<ReverseKeySchedule<RoundKey>>;

pub const ROUNDS: usize = 16;

pub fn encrypt(plaintext: Text, key: Key) -> Grass1Encrypt {
    FeistelCipher::new(
        plaintext,
        SimpleRotateKeySchedule::new(key),
        round,
        ROUNDS,
        CipherMode::Encrypt,
    )
}

pub fn decrypt(ciphertext: Text, key: Key) -> Grass1Decrypt {
    FeistelCipher::new(
        ciphertext,
        ReverseKeySchedule::new(SimpleRotateKeySchedule::new(key), ROUNDS),
        round,
        ROUNDS,
        CipherMode::Decrypt,
    )
}

pub fn round(mut block: Block, round_key: RoundKey) -> Block {
    block
        .as_mut_slice()
        .iter_mut()
        .zip(round_key.as_slice().iter().copied())
        .for_each(|(a, b)| *a ^= S_BOX[usize::from(b)]);

    block
}

#[test]
fn grass_1() {
    let plaintext = b"The secret phrase is 'befuddle'!";
    let key = [
        0xde, 0xad, 0xbe, 0xef, 0x00, 0xff, 0x11, 0x22, 0x33, 0xee, 0xdd, 0xcc, 0xca, 0xfe, 0xba,
        0xbe,
    ];

    let plaintext = ByteArray::from_slice(plaintext).clone();
    let mut cipher = encrypt(plaintext, key);
    cipher.dump();
    cipher.run();
    let ciphertext = cipher.result();

    println!(
        "----------------------------------------------------------------------------------------------------"
    );

    let mut cipher = decrypt(ciphertext, key);
    cipher.run();
    let new_plaintext = cipher.result();

    assert_eq!(plaintext, new_plaintext);
}
