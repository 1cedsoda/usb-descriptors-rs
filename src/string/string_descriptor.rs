use alloc::vec::Vec;

use crate::{
    binary::{EncodeByte, EncodeBytes},
    descriptor::Descriptor,
    descriptor_type::DescriptorType,
};

use super::string_content::StringContent;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringDescriptor {
    /// The USB standart doens't define an identifier for string descriptors but we need one
    pub index: u8,
    pub b_length: u8,
    pub b_descriptor_type: DescriptorType,
    pub b_string: StringContent,
}

impl Descriptor for StringDescriptor {
    fn encode(&self) -> Result<Vec<u8>, &str> {
        let mut bytes = Vec::<u8>::new();
        bytes.push(self.b_length);
        bytes.push(self.b_descriptor_type.encode()?);
        bytes.append(&mut self.b_string.encode()?);

        if bytes.len() != self.b_length as usize {
            return Err("b_length does not match the actual length");
        }

        Ok(bytes)
    }

    fn get_w_value(&self) -> u16 {
        (self.b_descriptor_type.encode().unwrap() as u16) << 8 | self.index as u16
    }
    
    fn get_descriptor_type(&self) -> DescriptorType {
        self.b_descriptor_type
    }
}
