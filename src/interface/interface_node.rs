use alloc::vec::Vec;

use crate::{endpoint::endpoint::Endpoint, string::string::String};

use super::{interface::Interface, interface_device_class::InterfaceDeviceClass};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InterfaceNode {
    pub b_alternate_setting: u8,
    pub b_interface_class: InterfaceDeviceClass,
    pub b_interface_sub_class: u8,
    pub b_interface_protocol: u8,
    /// Turns into i_interface
    pub interface: String,
    /// Turns into b_num_endpoints
    pub endpoints: Vec<Endpoint>,
}

impl InterfaceNode {
    pub fn get_interface(&self) -> Interface {
        Interface {
            b_alternate_setting: self.b_alternate_setting,
            b_interface_class: self.b_interface_class,
            b_interface_sub_class: self.b_interface_sub_class,
            b_interface_protocol: self.b_interface_protocol,
        }
    }
}
