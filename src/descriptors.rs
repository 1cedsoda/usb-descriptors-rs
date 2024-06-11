use alloc::vec::Vec;

use crate::descriptors_builder::DescriptorsBuilder;

pub struct Descriptors {
    pub descriptors: Vec<Vec<u8>>,
    pub w_values: Vec<u16>,
}

impl Descriptors {
    pub fn encode<'a>(builder: &DescriptorsBuilder) -> Result<Descriptors, &'a str> {
        let descriptors: Vec<Vec<u8>> = builder
            .descriptors
            .iter()
            .map(|x| x.encode().unwrap()) // TODO: no unwrap
            .collect();
        Ok(Descriptors {
            descriptors,
            w_values: builder.w_values.clone(),
        })
    }

    pub fn get_descriptor(&self, w_value: u16) -> Option<&Vec<u8>> {
        for i in 0..self.w_values.len() {
            let current_w_value = self.w_values.get(i)?;
            if *current_w_value == w_value {
                return self.descriptors.get(i);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec::Vec;

    use crate::{
        configuration::{
            configuration_attributes::ConfigurationAttributes,
            configuration_builder::ConfigurationBuilder, milliamperes::Milliamperes,
        },
        descriptors::{Descriptors, DescriptorsBuilder},
        device::{device_builder::DeviceBuilder, device_class::DeviceClass},
        endpoint::{
            direction::Direction, endpoint_address::EndpointAddress,
            endpoint_attributes::EndpointAttributes, endpoint_builder::EndpointBuilder,
            sync_type::SyncType, transfer_type::TransferType, usage_type::UsageType,
        },
        interface::{interface_builder::InterfaceBuilder, interface_class::InterfaceClass},
        string::{language_code::EN_US, string_builder::StringBuidler},
        version::USB2_0,
    };

    #[test]
    fn test_descriptor_store() {
        let builder = DescriptorsBuilder::build(
            &DeviceBuilder {
                usb: USB2_0,
                device_class: DeviceClass::Device,
                device_suclass: 0x00,
                device_protocol: 0x00,
                max_packet_size_0: 0x40,
                id_vendor: 0x1234,
                id_product: 0x1234,
                device: USB2_0,
                manufacturer: StringBuidler::text("MA"),
                product: StringBuidler::text("PR"),
                serial_number: StringBuidler::text("SE"),
                configurations: vec![ConfigurationBuilder {
                    configuration: StringBuidler::text("C0"),
                    configuration_value: 0x01,
                    attributes: ConfigurationAttributes {
                        self_powered: false,
                        remote_wakeup: false,
                    },
                    max_power: Milliamperes(500),
                    interfaces: vec![InterfaceBuilder {
                        interface: StringBuidler::text("I0"),
                        alternate_setting: 0x00,
                        interface_class: InterfaceClass::HumanInterfaceDevice,
                        interface_suclass: 0x00,
                        interface_protocol: 0x00,
                        endpoints: vec![EndpointBuilder {
                            endpoint_address: EndpointAddress {
                                endpoint_number: 0x01,
                                direction: Direction::In,
                            },
                            attributes: EndpointAttributes {
                                transfer_type: TransferType::Interrupt,
                                sync_type: SyncType::NoSync,
                                usage_type: UsageType::Data,
                            },
                            max_packet_size: 0x40,
                            interval: 0x01,
                        }],
                    }],
                }],
            },
            &StringBuidler::languages(vec![EN_US]),
        )
        .unwrap();

        let store = Descriptors::encode(&builder).unwrap();

        for i in 0..store.w_values.len() {
            let value = builder.w_values[i];
            let descriptor = &builder.descriptors[i];
            println!("0x{:x} {}", value, descriptor.get_descriptor_type())
        }

        // descriptos list and descriptor_values have to be equal
        assert_eq!(store.w_values.len(), store.descriptors.len());

        // check descriptor_values for dubplicates
        let mut found_descriptor_values = Vec::<u16>::new();
        for value in store.w_values.iter() {
            assert_eq!(
                found_descriptor_values.contains(value),
                false,
                "Duplicate descriptor_value found: 0x{:x}",
                value
            );
            found_descriptor_values.push(*value);

            // try to get descriptor by value
            let descriptor = store.get_descriptor(*value);
            assert_eq!(
                descriptor.is_some(),
                true,
                "Descriptor not found for value: 0x{:x}",
                value
            );

            // check if descriptor type matches forst 8 bits of value
            assert_eq!(
                *descriptor.unwrap().get(1).unwrap(),
                (*value >> 8) as u8,
                "Descriptor type 0x{:x} does not match value: 0x{:x}",
                *descriptor.unwrap().get(1).unwrap(),
                value
            );
        }

        let expected = vec![
            // String Descriptor - Language
            vec![
                0x04, // bLength = 4
                0x03, // bDescriptorType = String
                0x09, 0x04, // wLANGID[0] = 0x0409 (English - United States)
            ],
            // Device descriptor
            vec![
                0x12, // bLength = 18
                0x01, // bDescriptorType = Device
                0x00, 0x02, // bcdUSB = 2.0
                0x00, // bDeviceClass = Device
                0x00, // bDeviceSubClass = 0
                0x00, // bDeviceProtocol = 0
                0x40, // bMaxPacketSize0 = 64
                0x34, 0x12, // idVendor = 0x1234
                0x34, 0x12, // idProduct = 0x1234
                0x00, 0x02, // bcdDevice = 2.0
                0x01, // iManufacturer = 1
                0x02, // iProduct = 2
                0x03, // iSerialNumber = 3
                0x01, // bNumConfigurations = 1
            ],
            // String Descriptor - Manufacturer
            vec![
                0x06, // bLength = 4
                0x03, // bDescriptorType = String
                77, 0x00, // M
                65, 0x00, // A
            ],
            // String Descriptor - Product
            vec![
                0x06, // bLength = 20
                0x03, // bDescriptorType = String
                0x50, 0x00, // P
                0x52, 0x00, // R
            ],
            // String Descriptor - Serial Number
            vec![
                0x06, // bLength = 12
                0x03, // bDescriptorType = String
                0x53, 0x00, // S
                0x45, 0x00, // E
            ],
            // Configuration descriptor
            vec![
                0x09, // bLength = 9
                0x02, // bDescriptorType = Configuration
                16, 0x00, // wTotalLength = 16
                0x01, // bNumInterfaces = 1
                0x02, // bConfigurationValue = 2
                0x03, // iConfiguration = 3
                0x80, // bmAttributes = 0x80
                250,  // bMaxPower = 500mA
            ],
            // String Descriptor - Configuration
            vec![
                0x06, // bLength = 6
                0x03, // bDescriptorType = String
                0x43, 0x00, // C
                0x30, 0x00, // 0
            ],
            // Interface descriptor
            vec![
                0x09, // bLength = 9
                0x04, // bDescriptorType = Interface
                0x00, // bInterfaceNumber = 0
                0x00, // bAlternateSetting = 0
                0x01, // bNumEndpoints = 1
                0x03, // bInterfaceClass = HID
                0x00, // bInterfaceSubClass = 0
                0x00, // bInterfaceProtocol = 0
                0x04, // iInterface = 0
            ],
            // String Descriptor - Interface
            vec![
                0x06, // bLength = 6
                0x03, // bDescriptorType = String
                0x49, 0x00, // I
                0x30, 0x00, // 0
            ],
            // Endpoint descriptor
            vec![
                0x07, // bLength = 7
                0x05, // bDescriptorType = Endpoint
                0x81, // bEndpointAddress = IN 1
                0x03, // bmAttributes = Interrupt
                0x40, 0x00, // wMaxPacketSize = 64
                0x01, // bInterval = 1
            ],
        ];
        for i in 0..store.w_values.len() {
            let w_value = store.w_values[i];
            let descriptor = &store.descriptors[i];
            let expected = &expected[i];
            assert_eq!(
                descriptor, expected,
                "Descriptor 0x{:x} at position {} does not match expected",
                w_value, i
            );
        }
    }
}
