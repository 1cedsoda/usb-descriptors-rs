use alloc::vec::Vec;

use crate::{
    binary::{EncodeByte, EncodeBytes},
    descriptor::Descriptor,
    descriptor_type::DescriptorType,
};

use super::string_content::StringContent;

pub const STRING_DESCRIPTOR_TYPE: DescriptorType = DescriptorType::String;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringDescriptor {
    /// The USB standart doesn't define an identifier for string descriptors but we need one for the `get_w_value` function
    pub index: u8,
    /// Turns into `bLength`
    pub length: u8,
    /// Turns into `bString`
    pub string: StringContent,
}

impl Descriptor for StringDescriptor {
    fn encode(&self) -> Result<Vec<u8>, &str> {
        let mut bytes = Vec::<u8>::new();
        bytes.push(self.length);
        bytes.push(STRING_DESCRIPTOR_TYPE.encode()?);
        bytes.append(&mut self.string.encode()?);

        if bytes.len() != self.length as usize {
            return Err("string bLength does not match the actual length");
        }

        Ok(bytes)
    }

    fn get_descriptor_type(&self) -> DescriptorType {
        STRING_DESCRIPTOR_TYPE
    }
}

#[cfg(test)]
mod tests {
    use crate::string::string_builder::StringBuidler;

    use super::*;

    #[test]
    fn test_string_descriptor_encode() {
        let string_builder = StringBuidler::text("test");

        let encoded = string_builder.build(0).encode().unwrap();
        assert_eq!(encoded, vec![10, 3, 116, 0, 101, 0, 115, 0, 116, 0]);
    }
}
