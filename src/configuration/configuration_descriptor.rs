use alloc::vec::Vec;

use crate::{
    binary::{EncodeByte, EncodeBytes},
    descriptor::Descriptor,
    descriptor_type::DescriptorType,
};

use super::configuration_attributes::ConfigurationAttributes;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigurationDescriptor {
    /// Turns into `bLength`
    pub length: u8,
    /// Turns into `bDescriptorType`
    pub descriptor_type: DescriptorType,
    /// Turns into `wTotalLength`
    pub total_length: u16,
    /// Turns into `bNumInterfaces`
    pub num_interfaces: u8,
    /// Turns into `bConfigurationValue`
    pub configuration_value: u8,
    /// Turns into `iConfiguration`
    pub configuration: u8,
    /// Turns into `bmAttributes`
    pub attributes: ConfigurationAttributes,
    /// Turns into `bMaxPower`
    pub max_power: u8,
}

impl Descriptor for ConfigurationDescriptor {
    fn encode(&self) -> Result<Vec<u8>, &str> {
        let mut bytes = Vec::<u8>::new();
        bytes.push(self.length);
        bytes.push(self.descriptor_type.encode()?);
        bytes.extend_from_slice(&self.total_length.to_le_bytes());
        bytes.push(self.num_interfaces);
        bytes.push(self.configuration_value);
        bytes.push(self.configuration);
        bytes.append(self.attributes.encode()?.as_mut());
        bytes.push(self.max_power);

        if bytes.len() != self.length as usize {
            return Err("length does not match the actual length");
        }

        Ok(bytes)
    }

    fn get_w_value(&self) -> u16 {
        (self.descriptor_type.encode().unwrap() as u16) << 8 | self.configuration_value as u16
    }

    fn get_descriptor_type(&self) -> DescriptorType {
        self.descriptor_type
    }
}
