use crate::types::ByteArray;
use generic_array::ArrayLength;

pub fn xor<N: ArrayLength>(x: ByteArray<N>, y: ByteArray<N>) -> ByteArray<N> {
    let mut output = x;
    output.iter_mut().zip(y.iter()).for_each(|(a, b)| *a ^= b);
    output
}

#[test]
fn test_xor() {
    use generic_array::GenericArray;

    macro_rules! check {
        ($a:expr, $b:expr, $expected:expr $(,)?) => {{
            let a = GenericArray::from_array($a);
            let b = GenericArray::from_array($b);
            let expected = GenericArray::from_array($expected);
            let actual = xor(a, b);
            assert_eq!(actual, expected, "Actual array does not match expected");
        }};
    }

    check!([], [], []);
    check!(
        [0xa5, 0xb4, 0xc3, 0xd2],
        [0x12, 0x34, 0x56, 0x78],
        [0xb7, 0x80, 0x95, 0xaa],
    );
    check!(
        [0xff, 0xee, 0xdd, 0xcc, 0xbb, 0xaa],
        [0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        [0xff, 0xee, 0xdd, 0xcc, 0xbb, 0xaa],
    );
    check!(
        [0x12, 0x34, 0x56, 0x78],
        [0xff, 0xff, 0xff, 0xff],
        [0xed, 0xcb, 0xa9, 0x87],
    );
}

pub fn slice_to_u64(bytes: &[u8]) -> u64 {
    let mut array = [0; 8];
    array.as_mut_slice().copy_from_slice(&bytes);
    u64::from_be_bytes(array)
}

#[test]
fn test_slice_to_u64() {
    macro_rules! check {
        ($num:expr, $bytes:expr $(,)?) => {
            assert_eq!(
                slice_to_u64(&$bytes),
                $num,
                "Actual extracted value does not match expected",
            );
        };
    }

    check!(
        0xdeadbeef00ff11ee,
        [0xde, 0xad, 0xbe, 0xef, 0x00, 0xff, 0x11, 0xee]
    );
    check!(
        0x0123456789abcdef,
        [0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef]
    );
    check!(
        0x00000000ffffffff,
        [0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff]
    );
}
