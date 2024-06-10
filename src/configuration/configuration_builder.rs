use alloc::vec::Vec;

use crate::{
    interface::interface_builder::InterfaceBuilder, string::string_builder::StringBuidler,
};

use super::{
    configuration_attributes::ConfigurationAttributes,
    configuration_descriptor::ConfigurationDescriptor, milliamperes::Milliamperes,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigurationBuilder {
    /// Turns into `bConfigurationValue`
    pub configuration_value: u8,
    /// Turns into `bmAttributes`
    pub attributes: ConfigurationAttributes,
    /// Max power consumption of the USB device from the bus in milliamperes. (Will be divided by to to fit u8 later)
    pub max_power: Milliamperes,
    /// Turns into `configuration`
    pub configuration: StringBuidler,
    /// Turns into `num_interfaces`
    pub interfaces: Vec<InterfaceBuilder>,
}

impl ConfigurationBuilder {
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
            total_length,
            num_interfaces,
            configuration_value,
            configuration,
            attributes: self.attributes.clone(),
            max_power: self.max_power,
        })
    }
}
