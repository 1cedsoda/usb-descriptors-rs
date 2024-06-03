use alloc::vec::Vec;

use crate::{
    binary::{EncodeByte, EncodeBytes},
    descriptor::Descriptor,
    descriptor_type::DescriptorType,
};

use super::{bcd_version::BcdVersion, device_device_class::DeviceDeviceClass};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceDescriptor {
    /// Turns into `bLength`
    pub length: u8,
    /// Turns into `bDescriptorType`
    pub descriptor_type: DescriptorType,
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
    pub manufacturer: u8,
    /// Turns into `iProduct`
    pub product: u8,
    /// Turns into `iSerialNumber`
    pub serial_number: u8,
    /// Turns into `bNumConfigurations`
    pub num_configurations: u8,
}

impl Descriptor for DeviceDescriptor {
    fn encode(&self) -> Result<Vec<u8>, &str> {
        self.device_class
            .validate(self.device_suclass, self.device_protocol)?;

        let mut bytes = Vec::<u8>::new();
        bytes.push(self.length);
        bytes.push(self.descriptor_type.encode()?);
        bytes.append(self.bcd_usb.encode()?.as_mut());
        bytes.push(self.device_class.encode()?);
        bytes.push(self.device_suclass);
        bytes.push(self.device_protocol);
        bytes.push(self.max_packet_size_0);
        bytes.extend_from_slice(&self.id_vendor.to_le_bytes());
        bytes.extend_from_slice(&self.id_product.to_le_bytes());
        bytes.append(self.bcd_device.encode()?.as_mut());
        bytes.push(self.manufacturer);
        bytes.push(self.product);
        bytes.push(self.serial_number);
        bytes.push(self.num_configurations);
        
        if bytes.len() != self.length as usize {
            return Err("length does not match the actual length");
        }
        
        Ok(bytes)
    }

    fn get_w_value(&self) -> u16 {
        (self.descriptor_type.encode().unwrap() as u16) << 8
    }
    
    fn get_descriptor_type(&self) -> DescriptorType {
        self.descriptor_type
    }
}
