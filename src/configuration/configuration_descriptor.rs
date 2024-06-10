use alloc::vec::Vec;

use crate::{
    binary::{EncodeByte, EncodeBytes},
    descriptor::Descriptor,
    descriptor_type::DescriptorType,
};

use super::{configuration_attributes::ConfigurationAttributes, milliamperes::Milliamperes};

pub const CONFIGURATION_DESCRIPTOR_LENGTH: u8 = 9;
pub const CONFIGURATION_DESCRIPTOR_TYPE: DescriptorType = DescriptorType::Configuration;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigurationDescriptor {
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
    pub max_power: Milliamperes,
}

impl Descriptor for ConfigurationDescriptor {
    fn encode(&self) -> Result<Vec<u8>, &str> {
        let mut bytes = Vec::<u8>::new();
        bytes.push(CONFIGURATION_DESCRIPTOR_LENGTH);
        bytes.push(CONFIGURATION_DESCRIPTOR_TYPE.encode()?);
        bytes.extend_from_slice(&self.total_length.to_le_bytes());
        bytes.push(self.num_interfaces);
        bytes.push(self.configuration_value);
        bytes.push(self.configuration);
        bytes.append(self.attributes.encode()?.as_mut());
        bytes.push(self.max_power.encode()?);

        if bytes.len() != CONFIGURATION_DESCRIPTOR_LENGTH as usize {
            return Err("configuration bLength not match the actual length");
        }

        Ok(bytes)
    }

    fn get_descriptor_type(&self) -> DescriptorType {
        CONFIGURATION_DESCRIPTOR_TYPE
    }
}
