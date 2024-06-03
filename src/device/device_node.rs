use alloc::vec::Vec;

use crate::{
    configuration::configuration_node::ConfigurationNode,
    string::string_intrinsics::StringIntrinsics,
};

use super::{
    bcd_version::BcdVersion, device_device_class::DeviceDeviceClass,
    device_intrinsics::DeviceIntrinsics,
};

pub struct DeviceNode {
    /// Turns into `bcdUSB`
    pub bcd_usb: BcdVersion,
    /// Turns into `bDeviceClass`
    pub device_class: DeviceDeviceClass,
    /// Turns into `bDeviceSubClass`
    pub device_suclass: u8,
    /// Turns into `bDeviceProtocol`
    pub device_protocol: u8,
    /// Turns into `bMaxPacketSize0`
    pub max_packet_size_0: u8,
    /// Turns into `idVendor`
    pub id_vendor: u16,
    /// Turns into `idProduct`
    pub id_product: u16,
    /// Turns into `bcdDevice`
    pub bcd_device: BcdVersion,
    /// Turns into `iManufacturer`
    pub manufacturer: StringIntrinsics,
    /// Turns into `iProduct`
    pub product: StringIntrinsics,
    /// Turns into `iSerialNumber`
    pub serial_number: StringIntrinsics,
    /// Turns into `bNumConfigurations`
    pub configurations: Vec<ConfigurationNode>,
}

impl DeviceNode {
    pub fn get_device(&self) -> DeviceIntrinsics {
        DeviceIntrinsics {
            bcd_usb: self.bcd_usb,
            device_class: self.device_class,
            device_suclass: self.device_suclass,
            device_protocol: self.device_protocol,
            max_packet_size_0: self.max_packet_size_0,
            id_vendor: self.id_vendor,
            id_product: self.id_product,
            bcd_device: self.bcd_device,
        }
    }
}
