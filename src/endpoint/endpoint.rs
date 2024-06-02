use crate::descriptor_type::DescriptorType;

use super::{
    endpoint_address::EndpointAddress, endpoint_attributes::EndpointAttributes,
    endpoint_descriptor::EndpointDescriptor,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Endpoint {
    pub b_endpoint_address: EndpointAddress,
    pub bm_attributes: EndpointAttributes,
    pub w_max_packet_size: u16,
    pub b_interval: u8,
}

impl Endpoint {
    pub fn build(&self) -> Result<EndpointDescriptor, &str> {
        if self.w_max_packet_size > 255 {
            return Err("w_max_packet_size must be less than or equal to 255.");
        }

        Ok(EndpointDescriptor {
            b_length: 7,
            b_descriptor_type: DescriptorType::Endpoint,
            b_endpoint_address: self.b_endpoint_address,
            bm_attributes: self.bm_attributes.clone(),
            w_max_packet_size: self.w_max_packet_size,
            b_interval: self.b_interval,
        })
    }
}
