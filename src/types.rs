use generic_array::{ArrayLength, GenericArray};

pub type ByteArray<N: ArrayLength> = GenericArray<u8, N>;
