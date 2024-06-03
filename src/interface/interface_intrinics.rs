use crate::descriptor_type::DescriptorType;

use super::{
    interface_descriptor::InterfaceDescriptor, interface_device_class::InterfaceDeviceClass,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct InterfaceIntrinics {
    /// Turns into `bAlternateSetting`
    pub alternate_setting: u8,
    /// Turns into `bInterfaceClass`
    pub interface_class: InterfaceDeviceClass,
    /// Turns into `bInterfaceSubClass`
    pub interface_suclass: u8,
    /// Turns into `bInterfaceProtocol`
    pub interface_protocol: u8,
}

impl InterfaceIntrinics {
    pub fn build(
        &self,
        interface_number: u8,
        interface: u8,
        num_endpoints: u8,
    ) -> Result<InterfaceDescriptor, &str> {
        if num_endpoints == 0 {
            return Err("InterfaceDescript need at least 1 endpoint.");
        }
        self.interface_class
            .validate(self.interface_suclass, self.interface_protocol)?;

        Ok(InterfaceDescriptor {
            length: 9,
            descriptor_type: DescriptorType::Interface,
            interface_number,
            alternate_setting: self.alternate_setting,
            num_endpoints: num_endpoints,
            interface_class: self.interface_class,
            interface_suclass: self.interface_suclass,
            interface_protocol: self.interface_protocol,
            interface: interface,
        })
    }
}
