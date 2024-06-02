use alloc::vec::Vec;

use crate::{configuration::configuration_node::ConfigurationNode, string::string::String};

use super::{bcd_version::BcdVersion, device::Device, device_device_class::DeviceDeviceClass};

pub struct DeviceNode {
    pub bcd_usb: BcdVersion,
    pub b_device_class: DeviceDeviceClass,
    pub b_device_sub_class: u8,
    pub b_device_protocol: u8,
    pub b_max_packet_size_0: u8,
    pub id_vendor: u16,
    pub id_product: u16,
    pub bcd_device: BcdVersion,
    pub manufacturer: String,
    pub product: String,
    pub serial_number: String,
    pub configurations: Vec<ConfigurationNode>,
}

impl DeviceNode {
    pub fn get_device(&self) -> Device {
        Device {
            bcd_usb: self.bcd_usb,
            b_device_class: self.b_device_class,
            b_device_sub_class: self.b_device_sub_class,
            b_device_protocol: self.b_device_protocol,
            b_max_packet_size_0: self.b_max_packet_size_0,
            id_vendor: self.id_vendor,
            id_product: self.id_product,
            bcd_device: self.bcd_device,
        }
    }
}
