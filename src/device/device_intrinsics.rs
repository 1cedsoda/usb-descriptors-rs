use crate::descriptor_type::DescriptorType;

use super::{
    bcd_version::BcdVersion, device_descriptor::DeviceDescriptor,
    device_device_class::DeviceDeviceClass,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceIntrinsics {
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
}

impl DeviceIntrinsics {
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
            length: 18,
            descriptor_type: DescriptorType::Device,
            bcd_usb: self.bcd_usb,
            device_class: self.device_class,
            device_suclass: self.device_suclass,
            device_protocol: self.device_protocol,
            max_packet_size_0: self.max_packet_size_0,
            id_vendor: self.id_vendor,
            id_product: self.id_product,
            bcd_device: self.bcd_device,
            manufacturer: manufacturer,
            product: product,
            serial_number: serial_number,
            num_configurations,
        })
    }
}
