use crate::binary::EncodeByte;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum InterfaceClass {
    Audio,
    CommunicationAndCDCControl,
    HumanInterfaceDevice,
    Physical,
    Image,
    Printer,
    MassStorage,
    CdcData ,
    SmartCard,
    ContentSecurity,
    Video,
    PersonalHealthcare,
    AudioVideo,
    UsbTypeCBridge,
    UsbBulkDisplay,
    MctpOverUSB,
    I3c,
    Diagnostic,
    WirelessController,
    Miscellaneous,
    ApplicationSpecific,
    VendorSpecific,
}

impl EncodeByte for InterfaceClass {
    fn encode(&self) -> Result<u8, &str> {
        match self {
            InterfaceClass::Audio => Ok(0x01),
            InterfaceClass::CommunicationAndCDCControl => Ok(0x02),
            InterfaceClass::HumanInterfaceDevice => Ok(0x03),
            InterfaceClass::Physical => Ok(0x05),
            InterfaceClass::Image => Ok(0x06),
            InterfaceClass::Printer => Ok(0x07),
            InterfaceClass::MassStorage => Ok(0x08),
            InterfaceClass::CdcData => Ok(0x0A),
            InterfaceClass::SmartCard => Ok(0x0B),
            InterfaceClass::ContentSecurity => Ok(0x0D),
            InterfaceClass::Video => Ok(0x0E),
            InterfaceClass::PersonalHealthcare => Ok(0x0F),
            InterfaceClass::AudioVideo => Ok(0x10),
            InterfaceClass::UsbTypeCBridge => Ok(0x12),
            InterfaceClass::UsbBulkDisplay => Ok(0x13),
            InterfaceClass::MctpOverUSB => Ok(0x14),
            InterfaceClass::I3c => Ok(0x15),
            InterfaceClass::Diagnostic => Ok(0xDC),
            InterfaceClass::WirelessController => Ok(0xE0),
            InterfaceClass::Miscellaneous => Ok(0xEF),
            InterfaceClass::ApplicationSpecific => Ok(0xFE),
            InterfaceClass::VendorSpecific => Ok(0xFF),
        }
    }
}

impl InterfaceClass {
    pub fn validate(&self, suclass: u8, protocol: u8) -> Result<(), &str> {
        let error = Err("The interface base class is not compatible with the interface subclass and protocol. Pease check https://www.usb.org/defined-class-codes");
        match self {
            InterfaceClass::Audio => Ok(()),
            InterfaceClass::CommunicationAndCDCControl => Ok(()),
            InterfaceClass::HumanInterfaceDevice => Ok(()),
            InterfaceClass::Physical => Ok(()),
            InterfaceClass::Image => match suclass {
                0x01 => match protocol {
                    0x01 => Ok(()),
                    _ => error,
                },

                _ => error,
            },
            InterfaceClass::Printer => Ok(()),
            InterfaceClass::MassStorage => Ok(()),
            InterfaceClass::CdcData => Ok(()),
            InterfaceClass::SmartCard => Ok(()),
            InterfaceClass::ContentSecurity => match suclass {
                0x00 => match protocol {
                    0x00 => Ok(()),
                    _ => error,
                },
                _ => error,
            },
            InterfaceClass::Video => Ok(()),
            InterfaceClass::PersonalHealthcare => Ok(()),
            InterfaceClass::AudioVideo => match suclass {
                0x01 | 0x02 | 0x03 => match protocol {
                    0x00 => Ok(()),
                    _ => error,
                },
                _ => error,
            },
            InterfaceClass::UsbTypeCBridge => match suclass {
                0x00 => match protocol {
                    0x00 => Ok(()),
                    _ => error,
                },
                _ => error,
            },
            InterfaceClass::UsbBulkDisplay => match suclass {
                0x00 => match protocol {
                    0x00 => Ok(()),
                    _ => error,
                },
                _ => error,
            },
            InterfaceClass::MctpOverUSB => match suclass {
                0x00 | 0x01 => match protocol {
                    0x01 | 0x02 => Ok(()),
                    _ => error,
                },
                _ => error,
            },
            InterfaceClass::I3c => match suclass {
                0x00 => match protocol {
                    0x00 => Ok(()),
                    _ => error,
                },
                _ => error,
            },
            InterfaceClass::Diagnostic => match suclass {
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
            InterfaceClass::WirelessController => match suclass {
                0x01 => match protocol {
                    0x01 | 0x02 | 0x03 | 0x04 => Ok(()),
                    _ => error,
                },
                0x02 => match protocol {
                    0x01 | 0x02 | 0x03 => Ok(()),
                    _ => error,
                },
                _ => error,
            },
            InterfaceClass::Miscellaneous => match suclass {
                0x01 | 0x02 | 0x06 => match protocol {
                    0x01 | 0x02 => Ok(()),
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
            InterfaceClass::ApplicationSpecific => match suclass {
                0x01 => match protocol {
                    0x01 => Ok(()),
                    _ => error,
                },
                0x02 => match protocol {
                    0x00 => Ok(()),
                    _ => error,
                },
                0x03 => match protocol {
                    0x00 | 0x01 => Ok(()),
                    _ => error,
                },
                _ => error,
            },
            InterfaceClass::VendorSpecific => Ok(()),
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
    const INTERFACE_USTYPE_C_BRIDGE: u8 = 0x12;
    const INTERFACE_USBULK_DISPLAY: u8 = 0x13;
    const INTERFACE_MCTP_OVER_USB: u8 = 0x14;
    const INTERFACE_I3C: u8 = 0x15;
    const INTERFACE_DIAGNOSTIC: u8 = 0xDC;
    const INTERFACE_WIRELESS_CONTROLLER: u8 = 0xE0;
    const INTERFACE_MISCELLANEOUS: u8 = 0xEF;
    const INTERFACE_APPLICATION_SPECIFIC: u8 = 0xFE;
    const INTERFACE_VENDOR_SPECIFIC: u8 = 0xFF;

    #[test]
    fn test_interface_device_class() {
        assert_eq!(InterfaceClass::Audio.encode().unwrap(), INTERFACE_AUDIO);
        assert_eq!(
            InterfaceClass::CommunicationAndCDCControl.encode().unwrap(),
            INTERFACE_COMMUNICATION_AND_CDC_CONTROL
        );
        assert_eq!(
            InterfaceClass::HumanInterfaceDevice.encode().unwrap(),
            INTERFACE_HUMAN_INTERFACE_DEVICE
        );
        assert_eq!(
            InterfaceClass::Physical.encode().unwrap(),
            INTERFACE_PHYSICAL
        );
        assert_eq!(InterfaceClass::Image.encode().unwrap(), INTERFACE_IMAGE);
        assert_eq!(InterfaceClass::Printer.encode().unwrap(), INTERFACE_PRINTER);
        assert_eq!(
            InterfaceClass::MassStorage.encode().unwrap(),
            INTERFACE_MASS_STORAGE
        );
        assert_eq!(
            InterfaceClass::CdcData.encode().unwrap(),
            INTERFACE_CDC_DATA
        );
        assert_eq!(
            InterfaceClass::SmartCard.encode().unwrap(),
            INTERFACE_SMART_CARD
        );
        assert_eq!(
            InterfaceClass::ContentSecurity.encode().unwrap(),
            INTERFACE_CONTENT_SECURITY
        );
        assert_eq!(InterfaceClass::Video.encode().unwrap(), INTERFACE_VIDEO);
        assert_eq!(
            InterfaceClass::PersonalHealthcare.encode().unwrap(),
            INTERFACE_PERSONAL_HEALTHCARE
        );
        assert_eq!(
            InterfaceClass::AudioVideo.encode().unwrap(),
            INTERFACE_AUDIO_VIDEO
        );
        assert_eq!(
            InterfaceClass::UsbTypeCBridge.encode().unwrap(),
            INTERFACE_USTYPE_C_BRIDGE
        );
        assert_eq!(
            InterfaceClass::UsbBulkDisplay.encode().unwrap(),
            INTERFACE_USBULK_DISPLAY
        );
        assert_eq!(
            InterfaceClass::MctpOverUSB.encode().unwrap(),
            INTERFACE_MCTP_OVER_USB
        );
        assert_eq!(InterfaceClass::I3c.encode().unwrap(), INTERFACE_I3C);
        assert_eq!(
            InterfaceClass::Diagnostic.encode().unwrap(),
            INTERFACE_DIAGNOSTIC
        );
        assert_eq!(
            InterfaceClass::WirelessController.encode().unwrap(),
            INTERFACE_WIRELESS_CONTROLLER
        );
        assert_eq!(
            InterfaceClass::Miscellaneous.encode().unwrap(),
            INTERFACE_MISCELLANEOUS
        );
        assert_eq!(
            InterfaceClass::ApplicationSpecific.encode().unwrap(),
            INTERFACE_APPLICATION_SPECIFIC
        );
        assert_eq!(
            InterfaceClass::VendorSpecific.encode().unwrap(),
            INTERFACE_VENDOR_SPECIFIC
        );
    }
}
