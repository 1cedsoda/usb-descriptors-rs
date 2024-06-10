use alloc::vec::Vec;

use crate::{
    configuration::configuration_builder::ConfigurationBuilder,
    string::string_builder::StringBuidler, version::Version,
};

use super::{device_class::DeviceClass, device_descriptor::DeviceDescriptor};

pub struct DeviceBuilder {
    /// Turns into `bcdUSB`
    pub usb: Version,
    /// Turns into `bDeviceClass`
    pub device_class: DeviceClass,
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
    pub device: Version,
    /// Turns into `iManufacturer`
    pub manufacturer: StringBuidler,
    /// Turns into `iProduct`
    pub product: StringBuidler,
    /// Turns into `iSerialNumber`
    pub serial_number: StringBuidler,
    /// Turns into `bNumConfigurations`
    pub configurations: Vec<ConfigurationBuilder>,
}

impl DeviceBuilder {
    pub fn build(
        &self,
        num_configurations: u8,
        manufacturer: u8,
        product: u8,
        serial_number: u8,
    ) -> Result<DeviceDescriptor, &str> {
        self.device_class
            .validate(self.device_suclass, self.device_protocol)?;

        Ok(DeviceDescriptor {
            usb: self.usb,
            device_class: self.device_class,
            device_suclass: self.device_suclass,
            device_protocol: self.device_protocol,
            max_packet_size_0: self.max_packet_size_0,
            id_vendor: self.id_vendor,
            id_product: self.id_product,
            device: self.device,
            manufacturer: manufacturer,
            product: product,
            serial_number: serial_number,
            num_configurations,
        })
    }
}
