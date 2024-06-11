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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_milliamperes_encode() {
        assert_eq!(Milliamperes(0).encode().unwrap(), 0);
        assert_eq!(Milliamperes(1).encode().unwrap(), 0);
        assert_eq!(Milliamperes(2).encode().unwrap(), 1);
        assert_eq!(Milliamperes(500).encode().unwrap(), 250);
        assert_eq!(Milliamperes(501).encode(), Err("Milliamperes cannot be greater than 500."));
    }
}