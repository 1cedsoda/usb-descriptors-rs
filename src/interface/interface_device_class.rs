use crate::binary::EncodeByte;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum InterfaceDeviceClass {
    Audio = 0x01,
    CommunicationAndCDCControl = 0x02,
    HumanInterfaceDevice = 0x03,
    Physical = 0x05,
    Image = 0x06,
    Printer = 0x07,
    MassStorage = 0x08,
    CdcData = 0x0A,
    SmartCard = 0x0B,
    ContentSecurity = 0x0D,
    Video = 0x0E,
    PersonalHealthcare = 0x0F,
    AudioVideo = 0x10,
    UsbTypeCBridge = 0x12,
    UsbBulkDisplay = 0x13,
    MctpOverUSB = 0x14,
    I3c = 0x15,
    Diagnostic = 0xDC,
    WirelessController = 0xE0,
    Miscellaneous = 0xEF,
    ApplicationSpecific = 0xFE,
    VendorSpecific = 0xFF,
}

impl EncodeByte for InterfaceDeviceClass {
    fn encode(&self) -> Result<u8, &str> {
        Ok(*self as u8)
    }
}

impl InterfaceDeviceClass {
    pub fn validate(&self, b_sub_class: u8, b_protocol: u8) -> Result<(), &str> {
        let error = Err("The interface base class is not compatible with the interface subclass and protocol. Pease check https://www.usb.org/defined-class-codes");
        match self {
            InterfaceDeviceClass::Audio => Ok(()),
            InterfaceDeviceClass::CommunicationAndCDCControl => Ok(()),
            InterfaceDeviceClass::HumanInterfaceDevice => Ok(()),
            InterfaceDeviceClass::Physical => Ok(()),
            InterfaceDeviceClass::Image => match b_sub_class {
                0x01 => match b_protocol {
                    0x01 => Ok(()),
                    _ => error,
                },

                _ => error,
            },
            InterfaceDeviceClass::Printer => Ok(()),
            InterfaceDeviceClass::MassStorage => Ok(()),
            InterfaceDeviceClass::CdcData => Ok(()),
            InterfaceDeviceClass::SmartCard => Ok(()),
            InterfaceDeviceClass::ContentSecurity => match b_sub_class {
                0x00 => match b_protocol {
                    0x00 => Ok(()),
                    _ => error,
                },
                _ => error,
            },
            InterfaceDeviceClass::Video => Ok(()),
            InterfaceDeviceClass::PersonalHealthcare => Ok(()),
            InterfaceDeviceClass::AudioVideo => match b_sub_class {
                0x01 => match b_protocol {
                    0x00 => Ok(()),
                    _ => error,
                },
                0x02 => match b_protocol {
                    0x00 => Ok(()),
                    _ => error,
                },
                0x03 => match b_protocol {
                    0x00 => Ok(()),
                    _ => error,
                },
                _ => error,
            },
            InterfaceDeviceClass::UsbTypeCBridge => match b_sub_class {
                0x00 => match b_protocol {
                    0x00 => Ok(()),
                    _ => error,
                },
                _ => error,
            },
            InterfaceDeviceClass::UsbBulkDisplay => match b_sub_class {
                0x00 => match b_protocol {
                    0x00 => Ok(()),
                    _ => error,
                },
                _ => error,
            },
            InterfaceDeviceClass::MctpOverUSB => match b_sub_class {
                0x00 => match b_protocol {
                    0x01 => Ok(()),
                    0x02 => Ok(()),
                    _ => error,
                },
                0x01 => match b_protocol {
                    0x01 => Ok(()),
                    0x02 => Ok(()),
                    _ => error,
                },
                _ => error,
            },
            InterfaceDeviceClass::I3c => match b_sub_class {
                0x00 => match b_protocol {
                    0x00 => Ok(()),
                    _ => error,
                },
                _ => error,
            },
            InterfaceDeviceClass::Diagnostic => match b_sub_class {
                0x01 => match b_protocol {
                    0x01 => Ok(()),
                    _ => error,
                },
                0x02 | 0x03 | 0x04 | 0x05 | 0x06 | 0x07 => match b_protocol {
                    0x00 | 0x01 => Ok(()),
                    _ => error,
                },
                0x08 => match b_protocol {
                    0x00 => Ok(()),
                    _ => error,
                },
                _ => error,
            },
            InterfaceDeviceClass::WirelessController => match b_sub_class {
                0x01 => match b_protocol {
                    0x01 | 0x02 | 0x03 | 0x04 => Ok(()),
                    _ => error,
                },
                0x02 => match b_protocol {
                    0x01 | 0x02 | 0x03 | 0x05 => Ok(()),
                    _ => error,
                },
                _ => error,
            },
            InterfaceDeviceClass::Miscellaneous => match b_sub_class {
                0x01 | 0x02 | 0x06 => match b_protocol {
                    0x01 => Ok(()),
                    0x02 => Ok(()),
                    _ => error,
                },
                0x03 => match b_protocol {
                    0x01 => Ok(()),
                    _ => error,
                },
                0x04 => match b_protocol {
                    0x01 | 0x02 | 0x03 | 0x04 | 0x05 | 0x06 | 0x07 => Ok(()),
                    _ => error,
                },
                0x05 | 0x07 => match b_protocol {
                    0x00 | 0x01 | 0x02 => Ok(()),
                    _ => error,
                },
                _ => error,
            },
            InterfaceDeviceClass::ApplicationSpecific => match b_sub_class {
                0x01 => match b_protocol {
                    0x01 => Ok(()),
                    _ => error,
                },
                0x02 => match b_protocol {
                    0x00 => Ok(()),
                    _ => error,
                },
                0x03 => match b_protocol {
                    0x00 | 0x01 => Ok(()),
                    _ => error,
                },
                _ => error,
            },
            InterfaceDeviceClass::VendorSpecific => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INTERFACE_AUDIO: u8 = 0x01;
    const INTERFACE_COMMUNICATION_AND_CDC_CONTROL: u8 = 0x02;
    const INTERFACE_HUMAN_INTERFACE_DEVICE: u8 = 0x03;
    const INTERFACE_PHYSICAL: u8 = 0x05;
    const INTERFACE_IMAGE: u8 = 0x06;
    const INTERFACE_PRINTER: u8 = 0x07;
    const INTERFACE_MASS_STORAGE: u8 = 0x08;
    const INTERFACE_CDC_DATA: u8 = 0x0A;
    const INTERFACE_SMART_CARD: u8 = 0x0B;
    const INTERFACE_CONTENT_SECURITY: u8 = 0x0D;
    const INTERFACE_VIDEO: u8 = 0x0E;
    const INTERFACE_PERSONAL_HEALTHCARE: u8 = 0x0F;
    const INTERFACE_AUDIO_VIDEO: u8 = 0x10;
    const INTERFACE_USB_TYPE_C_BRIDGE: u8 = 0x12;
    const INTERFACE_USB_BULK_DISPLAY: u8 = 0x13;
    const INTERFACE_MCTP_OVER_USB: u8 = 0x14;
    const INTERFACE_I3C: u8 = 0x15;
    const INTERFACE_DIAGNOSTIC: u8 = 0xDC;
    const INTERFACE_WIRELESS_CONTROLLER: u8 = 0xE0;
    const INTERFACE_MISCELLANEOUS: u8 = 0xEF;
    const INTERFACE_APPLICATION_SPECIFIC: u8 = 0xFE;
    const INTERFACE_VENDOR_SPECIFIC: u8 = 0xFF;

    #[test]
    fn test_interface_device_class() {
        assert_eq!(
            InterfaceDeviceClass::Audio.encode().unwrap(),
            INTERFACE_AUDIO
        );
        assert_eq!(
            InterfaceDeviceClass::CommunicationAndCDCControl
                .encode()
                .unwrap(),
            INTERFACE_COMMUNICATION_AND_CDC_CONTROL
        );
        assert_eq!(
            InterfaceDeviceClass::HumanInterfaceDevice.encode().unwrap(),
            INTERFACE_HUMAN_INTERFACE_DEVICE
        );
        assert_eq!(
            InterfaceDeviceClass::Physical.encode().unwrap(),
            INTERFACE_PHYSICAL
        );
        assert_eq!(
            InterfaceDeviceClass::Image.encode().unwrap(),
            INTERFACE_IMAGE
        );
        assert_eq!(
            InterfaceDeviceClass::Printer.encode().unwrap(),
            INTERFACE_PRINTER
        );
        assert_eq!(
            InterfaceDeviceClass::MassStorage.encode().unwrap(),
            INTERFACE_MASS_STORAGE
        );
        assert_eq!(
            InterfaceDeviceClass::CdcData.encode().unwrap(),
            INTERFACE_CDC_DATA
        );
        assert_eq!(
            InterfaceDeviceClass::SmartCard.encode().unwrap(),
            INTERFACE_SMART_CARD
        );
        assert_eq!(
            InterfaceDeviceClass::ContentSecurity.encode().unwrap(),
            INTERFACE_CONTENT_SECURITY
        );
        assert_eq!(
            InterfaceDeviceClass::Video.encode().unwrap(),
            INTERFACE_VIDEO
        );
        assert_eq!(
            InterfaceDeviceClass::PersonalHealthcare.encode().unwrap(),
            INTERFACE_PERSONAL_HEALTHCARE
        );
        assert_eq!(
            InterfaceDeviceClass::AudioVideo.encode().unwrap(),
            INTERFACE_AUDIO_VIDEO
        );
        assert_eq!(
            InterfaceDeviceClass::UsbTypeCBridge.encode().unwrap(),
            INTERFACE_USB_TYPE_C_BRIDGE
        );
        assert_eq!(
            InterfaceDeviceClass::UsbBulkDisplay.encode().unwrap(),
            INTERFACE_USB_BULK_DISPLAY
        );
        assert_eq!(
            InterfaceDeviceClass::MctpOverUSB.encode().unwrap(),
            INTERFACE_MCTP_OVER_USB
        );
        assert_eq!(InterfaceDeviceClass::I3c.encode().unwrap(), INTERFACE_I3C);
        assert_eq!(
            InterfaceDeviceClass::Diagnostic.encode().unwrap(),
            INTERFACE_DIAGNOSTIC
        );
        assert_eq!(
            InterfaceDeviceClass::WirelessController.encode().unwrap(),
            INTERFACE_WIRELESS_CONTROLLER
        );
        assert_eq!(
            InterfaceDeviceClass::Miscellaneous.encode().unwrap(),
            INTERFACE_MISCELLANEOUS
        );
        assert_eq!(
            InterfaceDeviceClass::ApplicationSpecific.encode().unwrap(),
            INTERFACE_APPLICATION_SPECIFIC
        );
        assert_eq!(
            InterfaceDeviceClass::VendorSpecific.encode().unwrap(),
            INTERFACE_VENDOR_SPECIFIC
        );
    }
}
