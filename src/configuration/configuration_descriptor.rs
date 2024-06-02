use alloc::vec::Vec;

use crate::{
    binary::{EncodeByte, EncodeBytes},
    descriptor::Descriptor,
    descriptor_type::DescriptorType,
};

use super::configuration_attributes::ConfigurationAttributes;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigurationDescriptor {
    pub b_length: u8,
    pub b_descriptor_type: DescriptorType,
    pub w_total_length: u16,
    pub b_num_interfaces: u8,
    pub b_configuration_value: u8,
    pub i_configuration: u8,
    pub bm_attributes: ConfigurationAttributes,
    pub b_max_power: u8,
}

impl Descriptor for ConfigurationDescriptor {
    fn encode(&self) -> Result<Vec<u8>, &str> {
        let mut bytes = Vec::<u8>::new();
        bytes.push(self.b_length);
        bytes.push(self.b_descriptor_type.encode()?);
        bytes.extend_from_slice(&self.w_total_length.to_le_bytes());
        bytes.push(self.b_num_interfaces);
        bytes.push(self.b_configuration_value);
        bytes.push(self.i_configuration);
        bytes.append(self.bm_attributes.encode()?.as_mut());
        bytes.push(self.b_max_power);

        if bytes.len() != self.b_length as usize {
            return Err("b_length does not match the actual length");
        }

        Ok(bytes)
    }

    fn get_w_value(&self) -> u16 {
        (self.b_descriptor_type.encode().unwrap() as u16) << 8 | self.b_configuration_value as u16
    }
    
    fn get_descriptor_type(&self) -> DescriptorType {
        self.b_descriptor_type
    }
}
