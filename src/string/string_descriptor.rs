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
    /// Turns into `bString`
    pub string: StringContent,
}

impl Descriptor for StringDescriptor {
    fn encode(&self) -> Result<Vec<u8>, &str> {
        let mut bytes = Vec::<u8>::new();
        let string_encoded = self.string.encode()?;
        let length = string_encoded.len() as u8 + 2;
        bytes.push(length);
        bytes.push(STRING_DESCRIPTOR_TYPE.encode()?);
        bytes.append(&mut self.string.encode()?);

        if bytes.len() != length as usize {
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
    use alloc::string::ToString;

    use super::*;

    #[test]
    fn test_encode() {
        let descriptor = StringDescriptor {
            index: 1,
            string: StringContent::Text("Hello".to_string()),
        };

        let bytes = descriptor.encode().unwrap();
        assert_eq!(
            bytes,
            vec![
                12,  // bLength
                3,   // bDescriptorType
                72,  // H
                0,   //
                101, // e
                0,   //
                108, // l
                0,   //
                108, // l
                0,   //
                111, // o
                0,   //
            ]
        );
    }
}
