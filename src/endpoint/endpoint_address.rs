use crate::binary::EncodeByte;

use super::direction::Direction;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct EndpointAddress {
    pub endpoint_number: u8,
    /// 0 = Out, 1 = In (Ignored for Control Endpoints)
    pub direction: Direction,
}

impl EncodeByte for EndpointAddress {
    //     Endpoint Address
    // Bits 0..3b Endpoint Number.
    // Bits 4..6b Reserved. Set to Zero
    // Bits 7 Direction 0 = Out, 1 = In (Ignored for Control Endpoints)
    fn encode(&self) -> Result<u8, &str> {
        let byte: u8 = 0;
        Ok(byte | self.endpoint_number | (self.direction as u8) << 7)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        let endpoint_address1 = EndpointAddress {
            endpoint_number: 1,
            direction: Direction::In,
        };
        assert_eq!(endpoint_address1.encode().unwrap(), 129);

        let endpoint_address2 = EndpointAddress {
            endpoint_number: 2,
            direction: Direction::Out,
        };
        assert_eq!(endpoint_address2.encode().unwrap(), 2);

        let endpoint_address3 = EndpointAddress {
            endpoint_number: 3,
            direction: Direction::In,
        };
        assert_eq!(endpoint_address3.encode().unwrap(), 131);
    }
}
