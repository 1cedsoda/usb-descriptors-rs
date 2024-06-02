use crate::binary::EncodeByte;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum UsageType {
    Data = 0x00,
    Feedback = 0x01,
    ImplicitFeedbackData = 0x02,
}
impl EncodeByte for UsageType {
    fn encode(&self) -> Result<u8, &str> {
        Ok(*self as u8)
    }
}
