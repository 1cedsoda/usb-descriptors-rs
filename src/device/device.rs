use crate::descriptor_type::DescriptorType;

use super::{
    bcd_version::BcdVersion, device_descriptor::DeviceDescriptor,
    device_device_class::DeviceDeviceClass,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Device {
    pub bcd_usb: BcdVersion,
    pub b_device_class: DeviceDeviceClass,
    pub b_device_sub_class: u8,
    pub b_device_protocol: u8,
    pub b_max_packet_size_0: u8,
    pub id_vendor: u16,
    pub id_product: u16,
    pub bcd_device: BcdVersion,
}

impl Device {
    pub fn build(
        &self,
        b_num_configurations: u8,
        i_manufacturer: u8,
        i_product: u8,
        i_serial_number: u8,
    ) -> Result<DeviceDescriptor, &str> {
        self.b_device_class
            .validate(self.b_device_sub_class, self.b_device_protocol)?;

        Ok(DeviceDescriptor {
            b_length: 18,
            b_descriptor_type: DescriptorType::Device,
            bcd_usb: self.bcd_usb,
            b_device_class: self.b_device_class,
            b_device_sub_class: self.b_device_sub_class,
            b_device_protocol: self.b_device_protocol,
            b_max_packet_size_0: self.b_max_packet_size_0,
            id_vendor: self.id_vendor,
            id_product: self.id_product,
            bcd_device: self.bcd_device,
            i_manufacturer: i_manufacturer,
            i_product: i_product,
            i_serial_number: i_serial_number,
            b_num_configurations,
        })
    }
}
