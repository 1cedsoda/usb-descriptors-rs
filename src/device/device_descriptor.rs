use alloc::vec::Vec;

use crate::{
    binary::{EncodeByte, EncodeBytes},
    descriptor::Descriptor,
    descriptor_type::DescriptorType,
};

use super::{bcd_version::BcdVersion, device_device_class::DeviceDeviceClass};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceDescriptor {
    pub b_length: u8,
    pub b_descriptor_type: DescriptorType,
    pub bcd_usb: BcdVersion,
    pub b_device_class: DeviceDeviceClass,
    pub b_device_sub_class: u8,
    pub b_device_protocol: u8,
    pub b_max_packet_size_0: u8,
    pub id_vendor: u16,
    pub id_product: u16,
    pub bcd_device: BcdVersion,
    pub i_manufacturer: u8,
    pub i_product: u8,
    pub i_serial_number: u8,
    pub b_num_configurations: u8,
}

impl Descriptor for DeviceDescriptor {
    fn encode(&self) -> Result<Vec<u8>, &str> {
        self.b_device_class
            .validate(self.b_device_sub_class, self.b_device_protocol)?;

        let mut bytes = Vec::<u8>::new();
        bytes.push(self.b_length);
        bytes.push(self.b_descriptor_type.encode()?);
        bytes.append(self.bcd_usb.encode()?.as_mut());
        bytes.push(self.b_device_class.encode()?);
        bytes.push(self.b_device_sub_class);
        bytes.push(self.b_device_protocol);
        bytes.push(self.b_max_packet_size_0);
        bytes.extend_from_slice(&self.id_vendor.to_le_bytes());
        bytes.extend_from_slice(&self.id_product.to_le_bytes());
        bytes.append(self.bcd_device.encode()?.as_mut());
        bytes.push(self.i_manufacturer);
        bytes.push(self.i_product);
        bytes.push(self.i_serial_number);
        bytes.push(self.b_num_configurations);
        
        if bytes.len() != self.b_length as usize {
            return Err("b_length does not match the actual length");
        }
        
        Ok(bytes)
    }

    fn get_w_value(&self) -> u16 {
        (self.b_descriptor_type.encode().unwrap() as u16) << 8
    }
    
    fn get_descriptor_type(&self) -> DescriptorType {
        self.b_descriptor_type
    }
}
