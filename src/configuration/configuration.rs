use crate::{binary::EncodeByte, descriptor_type::DescriptorType};

use super::{
    configuration_attributes::ConfigurationAttributes,
    configuration_descriptor::ConfigurationDescriptor, milliamperes::Milliamperes,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Configuration {
    pub b_descriptor_type: DescriptorType,
    pub bm_attributes: ConfigurationAttributes,
    pub b_max_power: Milliamperes,
}

impl Configuration {
    pub fn build(
        &self,
        w_total_length: u16,
        b_num_interfaces: u8,
        b_configuration_value: u8,
        i_configuration: u8,
    ) -> Result<ConfigurationDescriptor, &str> {
        if b_num_interfaces == 0 {
            return Err("ConfigurationDescriptor needs at least 1 interface.");
        }

        Ok(ConfigurationDescriptor {
            b_length: 9,
            b_descriptor_type: self.b_descriptor_type,
            w_total_length,
            b_num_interfaces,
            b_configuration_value,
            i_configuration,
            bm_attributes: self.bm_attributes.clone(),
            b_max_power: self.b_max_power.encode()?,
        })
    }
}
