use super::KeySchedule;

pub type RoundKey = [u8; 4];

/// Key schedule from TEA.
///
/// 128-bit key which continually yields each 32-bit chunk.
#[derive(Debug)]
pub struct TeaKeySchedule {
    key: [u8; 16],
    index: u8,
}

impl TeaKeySchedule {
    pub fn new(key: [u8; 16]) -> Self {
        TeaKeySchedule { key, index: 0 }
    }
}

impl KeySchedule<RoundKey> for TeaKeySchedule {
    fn next_key(&mut self) -> [u8; 4] {
        let start = usize::from(self.index * 4);
        let slice = &self.key[start..start + 4];
        self.index = (self.index + 1) % 4;

        let mut block = [0; 4];
        block.as_mut_slice().copy_from_slice(slice);
        block
    }
}

#[test]
fn tea() {
    let mut key_sched = TeaKeySchedule::new([
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
        0x0f,
    ]);

    assert_eq!(key_sched.next_key(), [0x00, 0x01, 0x02, 0x03]);
    assert_eq!(key_sched.next_key(), [0x04, 0x05, 0x06, 0x07]);
    assert_eq!(key_sched.next_key(), [0x08, 0x09, 0x0a, 0x0b]);
    assert_eq!(key_sched.next_key(), [0x0c, 0x0d, 0x0e, 0x0f]);
    assert_eq!(key_sched.next_key(), [0x00, 0x01, 0x02, 0x03]);
}
