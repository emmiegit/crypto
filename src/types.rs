use generic_array::GenericArray;

pub type ByteArray<N> = GenericArray<u8, N>;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CipherMode {
    Encrypt,
    Decrypt,
}
