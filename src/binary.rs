use alloc::vec::Vec;

pub trait EncodeBytes {
    fn encode(&self) -> Result<Vec<u8>, &str>;
    fn validate(&self) -> Result<(), &str> {
        Ok(())
    }
}

pub trait EncodeByte {
    fn encode(&self) -> Result<u8, &str>;
}

pub trait DecodeBytes {
    fn decode(bytes: &[u8]) -> Result<Self, &str>
    where
        Self: Sized;
    fn size(&self) -> Result<usize, &str>;
}

pub trait DecodeByte {
    fn decode(bytes: &[u8]) -> Result<Self, &str>
    where
        Self: Sized;
}
