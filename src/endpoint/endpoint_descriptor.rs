use alloc::vec::Vec;

use crate::{binary::EncodeByte, descriptor::Descriptor, descriptor_type::DescriptorType};

use super::{endpoint_address::EndpointAddress, endpoint_attributes::EndpointAttributes};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EndpointDescriptor {
    /// Turns into `bLength`
    pub length: u8,
    /// Turns into `bDescriptorType`
    pub descriptor_type: DescriptorType,
    /// Turns into `bEndpointAddress`
    pub endpoint_address: EndpointAddress,
    /// Turns into `bmAttributes`
    pub attributes: EndpointAttributes,
    /// Turns into `wMaxPacketSize`
    pub max_packet_size: u16,
    /// Turns into `bInterval`
    pub interval: u8,
}

impl Descriptor for EndpointDescriptor {
    fn encode(&self) -> Result<Vec<u8>, &str> {
        let mut bytes = Vec::<u8>::new();
        bytes.push(self.length);
        bytes.push(DescriptorType::Endpoint.encode()?);
        bytes.push(self.endpoint_address.encode()?);
        bytes.push(self.attributes.encode()?);
        bytes.extend_from_slice(&self.max_packet_size.to_le_bytes());
        bytes.push(self.interval);

        if bytes.len() != self.length as usize {
            return Err("length does not match the actual length");
        }

        Ok(bytes)
    }

    fn get_w_value(&self) -> u16 {
        (self.descriptor_type.encode().unwrap() as u16) << 8
            | self.endpoint_address.encode().unwrap() as u16
    }

    fn get_descriptor_type(&self) -> DescriptorType {
        self.descriptor_type
    }
}

#[cfg(test)]
pub mod tests {
    use crate::endpoint::{
        direction::Direction, endpoint_intrinsics::EndpointIntrinsics, sync_type::SyncType,
        transfer_type::TransferType, usage_type::UsageType,
    };

    use super::*;

    #[test]
    fn test_encode() {
        let endpoint_descriptor = EndpointIntrinsics {
            endpoint_address: EndpointAddress {
                endpoint_number: 1,
                direction: Direction::In,
            },
            attributes: EndpointAttributes {
                transfer_type: TransferType::Interrupt,
                sync_type: SyncType::NoSync,
                usage_type: UsageType::Data,
            },
            max_packet_size: 16,
            interval: 10,
        }
        .build()
        .unwrap();
        let endpoint_descriptor_encoded = vec![7, 5, 129, 3, 16, 0, 10];
        assert_eq!(
            endpoint_descriptor.encode().unwrap(),
            endpoint_descriptor_encoded
        );
    }
}
