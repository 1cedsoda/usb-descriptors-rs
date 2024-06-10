use crate::binary::EncodeByte;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Milliamperes(pub u16);

impl EncodeByte for Milliamperes {
    fn encode(&self) -> Result<u8, &str> {
        if self.0 > 500 {
            return Err("Milliamperes cannot be greater than 500.");
        }

        let half = self.0 / 2;

        Ok(half as u8)
    }
}
