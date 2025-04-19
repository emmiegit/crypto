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
