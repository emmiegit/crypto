use super::FeistelCipher;
use crate::{key_schedule::TeaKeySchedule, types::ByteArray};
use generic_array::typenum::U64;

pub type Grass1RoundFn = fn([u8; 4], ByteArray<U64>) -> ByteArray<U64>;
pub type Grass1Cipher = FeistelCipher<TeaKeySchedule, Grass1RoundFn, U64>;

pub fn grass_1_round() {
    todo!()
}
