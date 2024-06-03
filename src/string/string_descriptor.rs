use alloc::vec::Vec;

use crate::{
    binary::{EncodeByte, EncodeBytes},
    descriptor::Descriptor,
    descriptor_type::DescriptorType,
};

use super::string_content::StringContent;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringDescriptor {
    /// The USB standart doesn't define an identifier for string descriptors but we need one for the `get_w_value` function
    pub index: u8,
    /// Turns into `bLength`
    pub length: u8,
    /// Turns into `bDescriptorType`
    pub descriptor_type: DescriptorType,
    /// Turns into `bString`
    pub string: StringContent,
}

impl Descriptor for StringDescriptor {
    fn encode(&self) -> Result<Vec<u8>, &str> {
        let mut bytes = Vec::<u8>::new();
        bytes.push(self.length);
        bytes.push(self.descriptor_type.encode()?);
        bytes.append(&mut self.string.encode()?);

        if bytes.len() != self.length as usize {
            return Err("length does not match the actual length");
        }

        Ok(bytes)
    }

    fn get_w_value(&self) -> u16 {
        (self.descriptor_type.encode().unwrap() as u16) << 8 | self.index as u16
    }
    
    fn get_descriptor_type(&self) -> DescriptorType {
        self.descriptor_type
    }
}
