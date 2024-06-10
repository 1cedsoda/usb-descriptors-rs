use alloc::vec::Vec;

use crate::{
    descriptor_type::DescriptorType, endpoint::endpoint_builder::EndpointBuilder,
    string::string_builder::StringBuidler,
};

use super::{interface_class::InterfaceClass, interface_descriptor::{InterfaceDescriptor, INTERFACE_DESCRIPTOR_TYPE}};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InterfaceBuilder {
    /// Turns into `bAlternateSetting`
    pub alternate_setting: u8,
    /// Turns into `bInterfaceClass`
    pub interface_class: InterfaceClass,
    /// Turns into `bInterfaceSubClass`
    pub interface_suclass: u8,
    /// Turns into `bInterfaceProtocol`
    pub interface_protocol: u8,
    /// Turns into `iInterface`
    pub interface: StringBuidler,
    /// Turns into `bNumEndpoints`
    pub endpoints: Vec<EndpointBuilder>,
}

impl InterfaceBuilder {
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
            interface_number,
            alternate_setting: self.alternate_setting,
            num_endpoints,
            interface_class: self.interface_class,
            interface_suclass: self.interface_suclass,
            interface_protocol: self.interface_protocol,
            interface,
        })
    }
}
