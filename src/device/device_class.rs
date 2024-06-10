use crate::binary::EncodeByte;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DeviceClass {
    Device,
    CommunicationAndCDCControl,
    Hub,
    Billboard,
    Diagnostic,
    Miscellaneous,
    VendorSpecific,
}

impl EncodeByte for DeviceClass {
    fn encode(&self) -> Result<u8, &str> {
        match *self {
            DeviceClass::Device => Ok(0x00),
            DeviceClass::CommunicationAndCDCControl => Ok(0x02),
            DeviceClass::Hub => Ok(0x09),
            DeviceClass::Billboard => Ok(0x11),
            DeviceClass::Diagnostic => Ok(0xDC),
            DeviceClass::Miscellaneous => Ok(0xEF),
            DeviceClass::VendorSpecific => Ok(0xFF),
        }
    }
}

impl DeviceClass {
    pub fn validate(&self, suclass: u8, protocol: u8) -> Result<(), &str> {
        let error: Result<(), &str> = Err("The device base class is not compatible with the interface subclass and protocol. Pease check https://www.usb.org/defined-class-codes");
        match self {
            DeviceClass::Device => match suclass {
                0x00 => match protocol {
                    0x00 => Ok(()),
                    _ => error,
                },
                _ => error,
            },
            DeviceClass::CommunicationAndCDCControl => Ok(()),
            DeviceClass::Hub => match suclass {
                0x00 => match protocol {
                    0x00 | 0x01 | 0x02 => Ok(()),
                    _ => error,
                },
                _ => error,
            },
            DeviceClass::Billboard => match suclass {
                0x00 => match protocol {
                    0x00 => Ok(()),
                    _ => error,
                },
                _ => error,
            },
            DeviceClass::Diagnostic => match suclass {
                0x01 => match protocol {
                    0x01 => Ok(()),
                    _ => error,
                },
                0x02 | 0x03 | 0x04 | 0x05 | 0x06 | 0x07 => match protocol {
                    0x00 | 0x01 => Ok(()),
                    _ => error,
                },
                0x08 => match protocol {
                    0x00 => Ok(()),
                    _ => error,
                },
                _ => error,
            },
            DeviceClass::Miscellaneous => match suclass {
                0x01 | 0x02 | 0x06 => match protocol {
                    0x01 => Ok(()),
                    0x02 => Ok(()),
                    _ => error,
                },
                0x03 => match protocol {
                    0x01 => Ok(()),
                    _ => error,
                },
                0x04 => match protocol {
                    0x01 | 0x02 | 0x03 | 0x04 | 0x05 | 0x06 | 0x07 => Ok(()),
                    _ => error,
                },
                0x05 | 0x07 => match protocol {
                    0x00 | 0x01 | 0x02 => Ok(()),
                    _ => error,
                },
                _ => error,
            },
            DeviceClass::VendorSpecific => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DEVICE_DEVICE: u8 = 0x00;
    const DEVICE_COMMUNICATION_AND_CDC_CONTROL: u8 = 0x02;
    const DEVICE_HUB: u8 = 0x09;
    const DEVICE_BILLBOARD: u8 = 0x11;
    const DEVICE_DIAGNOSTIC: u8 = 0xDC;
    const DEVICE_MISCELLANEOUS: u8 = 0xEF;
    const DEVICE_VENDOR_SPECIFIC: u8 = 0xFF;

    #[test]
    fn test_device_device_class() {
        assert_eq!(DeviceClass::Device.encode().unwrap(), DEVICE_DEVICE);
        assert_eq!(
            DeviceClass::CommunicationAndCDCControl.encode().unwrap(),
            DEVICE_COMMUNICATION_AND_CDC_CONTROL
        );
        assert_eq!(DeviceClass::Hub.encode().unwrap(), DEVICE_HUB);
        assert_eq!(DeviceClass::Billboard.encode().unwrap(), DEVICE_BILLBOARD);
        assert_eq!(DeviceClass::Diagnostic.encode().unwrap(), DEVICE_DIAGNOSTIC);
        assert_eq!(
            DeviceClass::Miscellaneous.encode().unwrap(),
            DEVICE_MISCELLANEOUS
        );
        assert_eq!(
            DeviceClass::VendorSpecific.encode().unwrap(),
            DEVICE_VENDOR_SPECIFIC
        );
    }
}
