use alloc::vec::Vec;

use crate::{binary::EncodeByte, descriptor::Descriptor, descriptor_type::DescriptorType};

use super::{endpoint_address::EndpointAddress, endpoint_attributes::EndpointAttributes};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EndpointDescriptor {
    pub b_length: u8,
    pub b_descriptor_type: DescriptorType,
    pub b_endpoint_address: EndpointAddress,
    pub bm_attributes: EndpointAttributes,
    pub w_max_packet_size: u16,
    pub b_interval: u8,
}

impl Descriptor for EndpointDescriptor {
    fn encode(&self) -> Result<Vec<u8>, &str> {
        let mut bytes = Vec::<u8>::new();
        bytes.push(self.b_length);
        bytes.push(DescriptorType::Endpoint.encode()?);
        bytes.push(self.b_endpoint_address.encode()?);
        bytes.push(self.bm_attributes.encode()?);
        bytes.extend_from_slice(&self.w_max_packet_size.to_le_bytes());
        bytes.push(self.b_interval);
        
        if bytes.len() != self.b_length as usize {
            return Err("b_length does not match the actual length");
        }

        Ok(bytes)
    }

    fn get_w_value(&self) -> u16 {
        (self.b_descriptor_type.encode().unwrap() as u16) << 8
            | self.b_endpoint_address.encode().unwrap() as u16
    }
    
    fn get_descriptor_type(&self) -> DescriptorType {
        self.b_descriptor_type
    }
}

#[cfg(test)]
pub mod tests {
    use crate::endpoint::{
        direction::Direction, endpoint::Endpoint, sync_type::SyncType, transfer_type::TransferType,
        usage_type::UsageType,
    };

    use super::*;

    #[test]
    fn test_encode() {
        let endpoint_descriptor = Endpoint {
            b_endpoint_address: EndpointAddress {
                endpoint_number: 1,
                direction: Direction::In,
            },
            bm_attributes: EndpointAttributes {
                transfer_type: TransferType::Interrupt,
                sync_type: SyncType::NoSync,
                usage_type: UsageType::Data,
            },
            w_max_packet_size: 16,
            b_interval: 10,
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
