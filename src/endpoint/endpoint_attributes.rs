use crate::binary::EncodeByte;

use super::{sync_type::SyncType, transfer_type::TransferType, usage_type::UsageType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EndpointAttributes {
    pub transfer_type: TransferType,
    pub sync_type: SyncType,
    pub usage_type: UsageType,
}

impl EncodeByte for EndpointAttributes {
    // Bits 0..1 Transfer Type
    //   00 = Control
    //   01 = Isochronous
    //   10 = Bulk
    //   11 = Interrupt
    // Bits 2..7 are reserved. If Isochronous endpoint,
    // Bits 3..2 = Synchronisation Type (Iso Mode)
    //   00 = No Synchonisation
    //   01 = Asynchronous
    //   10 = Adaptive
    //   11 = Synchronous
    // Bits 5..4 = Usage Type (Iso Mode)
    //   00 = Data Endpoint
    //   01 = Feedback Endpoint
    //   10 = Explicit Feedback Data Endpoint
    //   11 = Reserved
    fn encode(&self) -> Result<u8, &str> {
        let transfer_type = self.transfer_type as u8;
        let sync_type = self.sync_type as u8;
        let usage_type = self.usage_type as u8;

        Ok(transfer_type | (sync_type << 2) | (usage_type << 4))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        let endpoint_attributes = EndpointAttributes {
            transfer_type: TransferType::Interrupt,
            sync_type: SyncType::NoSync,
            usage_type: UsageType::Data,
        }
        .encode()
        .unwrap();
        let endpoint_attributes_encoded = 0b00000011;
        assert_eq!(endpoint_attributes, endpoint_attributes_encoded);
    }
}
