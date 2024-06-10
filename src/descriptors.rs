use alloc::vec::Vec;

use crate::descriptors_builder::DescriptorsBuilder;

pub struct Descriptors {
    pub descriptors: Vec<Vec<u8>>,
    pub descriptor_w_values: Vec<u16>,
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
            descriptor_w_values: builder.w_values.clone(),
        })
    }

    pub fn get_descriptor(&self, w_value: u16) -> Option<&Vec<u8>> {
        for i in 0..self.descriptor_w_values.len() {
            let current_w_value = self.descriptor_w_values.get(i)?;
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
                manufacturer: StringBuidler::text("Manufacturer"),
                product: StringBuidler::text("Product"),
                serial_number: StringBuidler::text("12345"),
                configurations: vec![ConfigurationBuilder {
                    configuration: StringBuidler::text("Configuration 0"),
                    configuration_value: 0x01,
                    attributes: ConfigurationAttributes {
                        self_powered: false,
                        remote_wakeup: false,
                    },
                    max_power: Milliamperes(500),
                    interfaces: vec![InterfaceBuilder {
                        interface: StringBuidler::text("Interface 0"),
                        alternate_setting: 0x00,
                        interface_class: InterfaceClass::HumanInterfaceDevice,
                        interface_suclass: 0x00,
                        interface_protocol: 0x00,
                        endpoints: vec![EndpointBuilder {
                            endpoint_address: EndpointAddress {
                                endpoint_number: 0x01,
                                direction: Direction::Out,
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
            StringBuidler::languages(vec![EN_US]),
        )
        .unwrap();

        let store = Descriptors::encode(&builder).unwrap();

        for i in 0..store.descriptor_w_values.len() {
            let value = builder.w_values[i];
            let descriptor = &builder.descriptors[i];
            println!("0x{:x} {}", value, descriptor.get_descriptor_type())
        }

        // descriptos list and descriptor_values have to be equal
        assert_eq!(store.descriptor_w_values.len(), store.descriptors.len());

        // check descriptor_values for dubplicates
        let mut found_descriptor_values = Vec::<u16>::new();
        for value in store.descriptor_w_values.iter() {
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
    }
}
