use crate::binary::EncodeByte;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum UsageType {
    Data,
    Feedback,
    ImplicitFeedbackData,
}

impl EncodeByte for UsageType {
    fn encode(&self) -> Result<u8, &str> {
        match *self {
            UsageType::Data => Ok(0x00),
            UsageType::Feedback => Ok(0x01),
            UsageType::ImplicitFeedbackData => Ok(0x02),
        }
    }
}
