use super::{
    endpoint_address::EndpointAddress,
    endpoint_attributes::EndpointAttributes,
    endpoint_descriptor::{EndpointDescriptor, ENDPOINT_DESCRIPTOR_LENGTH},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EndpointBuilder {
    /// Turns into `bEndpointAddress`
    pub endpoint_address: EndpointAddress,
    /// Turns into `bmAttributes`
    pub attributes: EndpointAttributes,
    /// Turns into `wMaxPacketSize`
    pub max_packet_size: u16,
    /// Turns into `bInterval`
    pub interval: u8,
}

impl EndpointBuilder {
    pub fn build(&self) -> Result<EndpointDescriptor, &str> {
        if self.max_packet_size > 255 {
            return Err("max_packet_size must be less than or equal to 255.");
        }

        Ok(EndpointDescriptor {
            length: ENDPOINT_DESCRIPTOR_LENGTH,
            endpoint_address: self.endpoint_address,
            attributes: self.attributes.clone(),
            max_packet_size: self.max_packet_size,
            interval: self.interval,
        })
    }
}
