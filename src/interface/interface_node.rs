use alloc::vec::Vec;

use crate::{
    endpoint::endpoint_intrinsics::EndpointIntrinsics, string::string_intrinsics::StringIntrinsics,
};

use super::{
    interface_device_class::InterfaceDeviceClass, interface_intrinics::InterfaceIntrinics,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InterfaceNode {
    /// Turns into `bAlternateSetting`
    pub alternate_setting: u8,
    /// Turns into `bInterfaceClass`
    pub interface_class: InterfaceDeviceClass,
    /// Turns into `bInterfaceSubClass`
    pub interface_suclass: u8,
    /// Turns into `bInterfaceProtocol`
    pub interface_protocol: u8,
    /// Turns into `iInterface`
    pub interface: StringIntrinsics,
    /// Turns into `bNumEndpoints`
    pub endpoints: Vec<EndpointIntrinsics>,
}

impl InterfaceNode {
    pub fn get_interface(&self) -> InterfaceIntrinics {
        InterfaceIntrinics {
            alternate_setting: self.alternate_setting,
            interface_class: self.interface_class,
            interface_suclass: self.interface_suclass,
            interface_protocol: self.interface_protocol,
        }
    }
}
