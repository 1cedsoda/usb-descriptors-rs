use crate::descriptor_type::DescriptorType;

use super::{
    endpoint_address::EndpointAddress, endpoint_attributes::EndpointAttributes,
    endpoint_descriptor::EndpointDescriptor,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EndpointIntrinsics {
    /// Turns into `bEndpointAddress`
    pub endpoint_address: EndpointAddress,
    /// Turns into `bmAttributes`
    pub attributes: EndpointAttributes,
    /// Turns into `wMaxPacketSize`
    pub max_packet_size: u16,
    /// Turns into `bInterval`
    pub interval: u8,
}

impl EndpointIntrinsics {
    pub fn build(&self) -> Result<EndpointDescriptor, &str> {
        if self.max_packet_size > 255 {
            return Err("max_packet_size must be less than or equal to 255.");
        }

        Ok(EndpointDescriptor {
            length: 7,
            descriptor_type: DescriptorType::Endpoint,
            endpoint_address: self.endpoint_address,
            attributes: self.attributes.clone(),
            max_packet_size: self.max_packet_size,
            interval: self.interval,
        })
    }
}
