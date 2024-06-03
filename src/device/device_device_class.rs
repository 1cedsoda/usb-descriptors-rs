use crate::binary::EncodeByte;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DeviceDeviceClass {
    Device = 0x00,
    CommunicationAndCDCControl = 0x02,
    Hub = 0x09,
    Billboard = 0x11,
    Diagnostic = 0xDC,
    Miscellaneous = 0xEF,
    VendorSpecific = 0xFF,
}

impl EncodeByte for DeviceDeviceClass {
    fn encode(&self) -> Result<u8, &str> {
        Ok(*self as u8)
    }
}

impl DeviceDeviceClass {
    pub fn validate(&self, suclass: u8, protocol: u8) -> Result<(), &str> {
        let error: Result<(), &str> = Err("The device base class is not compatible with the interface subclass and protocol. Pease check https://www.usb.org/defined-class-codes");
        match self {
            DeviceDeviceClass::Device => match suclass {
                0x00 => match protocol {
                    0x00 => Ok(()),
                    _ => error,
                },
                _ => error,
            },
            DeviceDeviceClass::CommunicationAndCDCControl => Ok(()),
            DeviceDeviceClass::Hub => match suclass {
                0x00 => match protocol {
                    0x00 | 0x01 | 0x02 => Ok(()),
                    _ => error,
                },
                _ => error,
            },
            DeviceDeviceClass::Billboard => match suclass {
                0x00 => match protocol {
                    0x00 => Ok(()),
                    _ => error,
                },
                _ => error,
            },
            DeviceDeviceClass::Diagnostic => match suclass {
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
            DeviceDeviceClass::Miscellaneous => match suclass {
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
            DeviceDeviceClass::VendorSpecific => Ok(()),
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
        assert_eq!(DeviceDeviceClass::Device.encode().unwrap(), DEVICE_DEVICE);
        assert_eq!(
            DeviceDeviceClass::CommunicationAndCDCControl
                .encode()
                .unwrap(),
            DEVICE_COMMUNICATION_AND_CDC_CONTROL
        );
        assert_eq!(DeviceDeviceClass::Hub.encode().unwrap(), DEVICE_HUB);
        assert_eq!(
            DeviceDeviceClass::Billboard.encode().unwrap(),
            DEVICE_BILLBOARD
        );
        assert_eq!(
            DeviceDeviceClass::Diagnostic.encode().unwrap(),
            DEVICE_DIAGNOSTIC
        );
        assert_eq!(
            DeviceDeviceClass::Miscellaneous.encode().unwrap(),
            DEVICE_MISCELLANEOUS
        );
        assert_eq!(
            DeviceDeviceClass::VendorSpecific.encode().unwrap(),
            DEVICE_VENDOR_SPECIFIC
        );
    }
}
