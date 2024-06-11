use alloc::vec::Vec;

use crate::{
    binary::{EncodeByte, EncodeBytes},
    descriptor::Descriptor,
    descriptor_type::DescriptorType,
    version::Version,
};

use super::device_class::DeviceClass;

pub const DEVICE_DESCRIPTOR_LENGTH: u8 = 18;
pub const DEVICE_DESCRIPTOR_TYPE: DescriptorType = DescriptorType::Device;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceDescriptor {
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
        bytes.push(DEVICE_DESCRIPTOR_LENGTH);
        bytes.push(DEVICE_DESCRIPTOR_TYPE.encode()?);
        bytes.append(self.usb.encode()?.as_mut());
        bytes.push(self.device_class.encode()?);
        bytes.push(self.device_suclass);
        bytes.push(self.device_protocol);
        bytes.push(self.max_packet_size_0);
        bytes.extend_from_slice(&self.id_vendor.to_le_bytes());
        bytes.extend_from_slice(&self.id_product.to_le_bytes());
        bytes.append(self.device.encode()?.as_mut());
        bytes.push(self.manufacturer);
        bytes.push(self.product);
        bytes.push(self.serial_number);
        bytes.push(self.num_configurations);

        if bytes.len() != DEVICE_DESCRIPTOR_LENGTH as usize {
            return Err("device bLength does not match the actual length");
        }

        Ok(bytes)
    }

    fn get_descriptor_type(&self) -> DescriptorType {
        DEVICE_DESCRIPTOR_TYPE
    }
}

#[cfg(test)]
mod tests {
    use crate::version::USB2_0;

    use super::*;

    #[test]
    fn test_encode() {
        let descriptor = DeviceDescriptor {
            usb: USB2_0,
            device_class: DeviceClass::Miscellaneous,
            device_suclass: 0x01,
            device_protocol: 0x02,
            max_packet_size_0: 0x08,
            id_vendor: 0x16c0,
            id_product: 0x05df,
            device: Version {
                major: 0x01,
                minor: 0x00,
            },
            manufacturer: 0x01,
            product: 0x02,
            serial_number: 0x03,
            num_configurations: 0x01,
        };

        assert_eq!(
            descriptor.encode().unwrap(),
            vec![
                18, // bLength
                1,  // bDescriptorType
                0x00, 0x02, // bcdUSB
                0xef, // bDeviceClass
                0x01, // bDeviceSubClass
                0x02, // bDeviceProtocol
                0x08, // bMaxPacketSize0
                0xc0, 0x16, // idVendor
                0xdf, 0x05, // idProduct
                0x00, 0x01, // bcdDevice
                0x01, // iManufacturer
                0x02, // iProduct
                0x03, // iSerialNumber
                0x01, // bNumConfigurations
            ]
        );
    }
}
