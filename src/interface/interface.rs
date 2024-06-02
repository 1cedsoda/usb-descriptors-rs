use crate::descriptor_type::DescriptorType;

use super::{
    interface_descriptor::InterfaceDescriptor, interface_device_class::InterfaceDeviceClass,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Interface {
    pub b_alternate_setting: u8,
    pub b_interface_class: InterfaceDeviceClass,
    pub b_interface_sub_class: u8,
    pub b_interface_protocol: u8,
}

impl Interface {
    pub fn build(
        &self,
        b_interface_number: u8,
        i_interface: u8,
        b_num_endpoints: u8,
    ) -> Result<InterfaceDescriptor, &str> {
        if b_num_endpoints == 0 {
            return Err("InterfaceDescript need at least 1 endpoint.");
        }
        self.b_interface_class
            .validate(self.b_interface_sub_class, self.b_interface_protocol)?;

        Ok(InterfaceDescriptor {
            b_length: 9,
            b_descriptor_type: DescriptorType::Interface,
            b_interface_number,
            b_alternate_setting: self.b_alternate_setting,
            b_num_endpoints: b_num_endpoints,
            b_interface_class: self.b_interface_class,
            b_interface_sub_class: self.b_interface_sub_class,
            b_interface_protocol: self.b_interface_protocol,
            i_interface: i_interface,
        })
    }
}
