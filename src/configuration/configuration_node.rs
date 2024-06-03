use alloc::vec::Vec;

use crate::{
    descriptor_type::DescriptorType, interface::interface_node::InterfaceNode,
    string::string_intrinsics::StringIntrinsics,
};

use super::{
    configuration_attributes::ConfigurationAttributes,
    configuration_intrinsics::ConfigurationIntrinsics, milliamperes::Milliamperes,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigurationNode {
    /// Turns into `bConfigurationValue`
    pub configuration_value: u8,
    /// Turns into `bmAttributes`
    pub attributes: ConfigurationAttributes,
    /// Max power consumption of the USB device from the bus in milliamperes. (Will be divided by to to fit u8 later)
    pub max_power: Milliamperes,
    /// Turns into `configuration`
    pub configuration: StringIntrinsics,
    /// Turns into `num_interfaces`
    pub interfaces: Vec<InterfaceNode>,
}

impl ConfigurationNode {
    pub fn get_configuration(&self) -> ConfigurationIntrinsics {
        ConfigurationIntrinsics {
            descriptor_type: DescriptorType::Configuration,
            attributes: self.attributes,
            max_power: self.max_power,
        }
    }
}
