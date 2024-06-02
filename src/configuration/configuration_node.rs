use alloc::vec::Vec;

use crate::{
    descriptor_type::DescriptorType, interface::interface_node::InterfaceNode,
    string::string::String,
};

use super::{
    configuration::Configuration, configuration_attributes::ConfigurationAttributes,
    milliamperes::Milliamperes,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigurationNode {
    pub b_configuration_value: u8,
    pub bm_attributes: ConfigurationAttributes,
    /// Max power consumption of the USB device from the bus in milliamperes. (Will be divided by to to fit u8 later)
    pub b_max_power: Milliamperes,
    /// Turns into i_configuration
    pub configuration: String,
    /// Turns into b_num_interfaces
    pub interfaces: Vec<InterfaceNode>,
}

impl ConfigurationNode {
    pub fn get_configuration(&self) -> Configuration {
        Configuration {
            b_descriptor_type: DescriptorType::Configuration,
            bm_attributes: self.bm_attributes,
            b_max_power: self.b_max_power,
        }
    }
}
