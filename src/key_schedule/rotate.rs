use super::KeySchedule;

pub type RoundKey = [u8; 16];

#[derive(Debug)]
pub struct SimpleRotateKeySchedule {
    left: u64,
    right: u64,
}

impl SimpleRotateKeySchedule {
    pub fn new(key: RoundKey) -> Self {
        SimpleRotateKeySchedule {
            left: slice_to_u64(&key[..8]),
            right: slice_to_u64(&key[8..]),
        }
    }
}

impl KeySchedule<RoundKey> for SimpleRotateKeySchedule {
    fn next_key(&mut self) -> RoundKey {
        self.left = self.left.rotate_left(1);
        self.right = self.right.rotate_left(1);

        let mut round_key = [0; 16];
        (&mut round_key[..8]).copy_from_slice(&self.left.to_be_bytes());
        (&mut round_key[8..]).copy_from_slice(&self.right.to_be_bytes());
        round_key
    }
}

fn slice_to_u64(bytes: &[u8]) -> u64 {
    let mut array = [0; 8];
    array.as_mut_slice().copy_from_slice(&bytes);
    u64::from_be_bytes(array)
}

#[test]
fn simple() {
    let mut key_sched = SimpleRotateKeySchedule::new([
        0xde, 0xad, 0xbe, 0xef, 0xfe, 0xed, 0xfa, 0xce, 0xca, 0xfe, 0xba, 0xbe, 0x00, 0x00, 0xff,
        0xff,
    ]);

    assert_eq!(
        key_sched.next_key(),
        [
            0xbd, 0x5b, 0x7d, 0xdf, 0xfd, 0xdb, 0xf5, 0x9d, 0x95, 0xfd, 0x75, 0x7c, 0x00, 0x01,
            0xff, 0xff,
        ]
    );

    assert_eq!(
        key_sched.next_key(),
        [
            0x7a, 0xb6, 0xfb, 0xbf, 0xfb, 0xb7, 0xeb, 0x3b, 0x2b, 0xfa, 0xea, 0xf8, 0x00, 0x03,
            0xff, 0xff,
        ]
    );
}

#[test]
fn test_bytes() {
    macro_rules! check {
        ($num:expr, $bytes:expr $(,)?) => {
            assert_eq!(
                slice_to_u64(&$bytes),
                $num,
                "Actual extracted value does not match expected",
            );
        };
    }

    check!(0xdeadbeef00ff11ee, [0xde, 0xad, 0xbe, 0xef, 0x00, 0xff, 0x11, 0xee]);
    check!(0x0123456789abcdef, [0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef]);
    check!(0x00000000ffffffff, [0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff]);
}
