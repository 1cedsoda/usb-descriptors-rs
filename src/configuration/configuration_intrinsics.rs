use crate::{binary::EncodeByte, descriptor_type::DescriptorType};

use super::{
    configuration_attributes::ConfigurationAttributes,
    configuration_descriptor::ConfigurationDescriptor, milliamperes::Milliamperes,
};

pub struct ConfigurationIntrinsics {
    /// Turns into `bDescriptorType`
    pub descriptor_type: DescriptorType,
    /// Turns into `bmAttributes`
    pub attributes: ConfigurationAttributes,
    /// Turns into `bMaxPower`
    pub max_power: Milliamperes,
}

impl ConfigurationIntrinsics {
    pub fn build(
        &self,
        total_length: u16,
        num_interfaces: u8,
        configuration_value: u8,
        configuration: u8,
    ) -> Result<ConfigurationDescriptor, &str> {
        if num_interfaces == 0 {
            return Err("ConfigurationDescriptor needs at least 1 interface.");
        }

        Ok(ConfigurationDescriptor {
            length: 9,
            descriptor_type: self.descriptor_type,
            total_length,
            num_interfaces,
            configuration_value,
            configuration,
            attributes: self.attributes.clone(),
            max_power: self.max_power.encode()?,
        })
    }
}
