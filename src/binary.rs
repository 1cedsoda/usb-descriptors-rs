use alloc::vec::Vec;

use crate::descriptor_type::DescriptorType;

pub trait EncodeBytes {
    fn encode(&self) -> Result<Vec<u8>, &str>;
    fn validate(&self) -> Result<(), &str> {
        Ok(())
    }
}

pub trait EncodeByte {
    fn encode(&self) -> Result<u8, &str>;
}

pub fn encode_w_value<'a>(descriptor_type: &'a DescriptorType, value: u8) -> Result<u16, &'a str> {
    Ok((descriptor_type.encode()? as u16) << 8 | (value as u16))
}
