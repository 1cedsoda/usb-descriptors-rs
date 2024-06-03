use alloc::{boxed::Box, vec::Vec};

use crate::{
    descriptor::Descriptor,
    descriptor_type::DescriptorType,
    device::device_node::DeviceNode,
    string::{string_content::StringContent, string_intrinsics::StringIntrinsics},
};

/// Stores descriptors for USB protocol
pub struct DescriptorStore {
    /// A list of stored descriptors
    descriptors: Vec<Box<dyn Descriptor>>,
    /// Acts as a lookup table for the descriptors (same length and order)
    descriptor_values: Vec<u16>,
}

impl DescriptorStore {
    /// Fill the descriptor store by providing a tree of descriptor nodes
    ///
    /// `new` converts the nodes into descriptors by calculating contextual values like indices and lengths
    pub fn new(
        device_node: DeviceNode,
        languages: StringIntrinsics,
    ) -> Result<DescriptorStore, &'static str> {
        match &languages.string {
            StringContent::Languages(languages) => match languages.len() {
                0 => return Err("At least one language is required"),
                _ => {}
            },
            _ => return Err("Only languages are supported"),
        }

        let mut descriptors = Vec::<Box<dyn Descriptor>>::new();
        let mut descriptor_values = Vec::<u16>::new();
        let mut configuration_descriptors_num: u8 = 0;
        let mut interface_descriptors_num: u8 = 0;
        let mut string_descriptors_num: u8 = 0;

        // String Descriptor (Language)
        let string_descriptor = languages.build(string_descriptors_num);
        descriptor_values.push(string_descriptor.get_w_value());
        descriptors.push(Box::new(string_descriptor));
        string_descriptors_num += 1;

        // Iterate Nodes
        let num_configurations = device_node.configurations.len() as u8;
        for configuration_node in &device_node.configurations {
            let num_interfaces = configuration_node.interfaces.len() as u8;
            let mut total_length = 0;
            for interface_node in &configuration_node.interfaces {
                let num_endpoints = interface_node.endpoints.len() as u8;
                for endpoint in &interface_node.endpoints {
                    // Endpoint Descriptor
                    let endpoint_descriptor = endpoint.build().unwrap();
                    total_length += endpoint_descriptor.length as u16;
                    descriptor_values.push(endpoint_descriptor.get_w_value());
                    descriptors.push(Box::new(endpoint_descriptor));
                }

                // Interface String Descriptor
                let string_descriptor = interface_node.interface.build(string_descriptors_num);
                descriptor_values.push(string_descriptor.get_w_value());
                descriptors.push(Box::new(string_descriptor));
                string_descriptors_num += 1;

                // Interface Descriptor
                let interface_descriptor = interface_node
                    .get_interface()
                    .build(
                        interface_descriptors_num,
                        string_descriptors_num,
                        num_endpoints,
                    )
                    .unwrap();
                total_length += interface_descriptor.length as u16;
                descriptor_values.push(interface_descriptor.get_w_value());
                descriptors.push(Box::new(interface_descriptor));
                interface_descriptors_num += 1;
            }

            // Configuration String Descriptor
            let string_descriptor = configuration_node
                .configuration
                .build(string_descriptors_num);
            descriptor_values.push(string_descriptor.get_w_value());
            descriptors.push(Box::new(string_descriptor));
            string_descriptors_num += 1;

            // Configuration Descriptor
            let configuration_descriptor = configuration_node
                .get_configuration()
                .build(
                    total_length,
                    num_interfaces,
                    configuration_descriptors_num,
                    string_descriptors_num,
                )
                .unwrap();
            descriptor_values.push(configuration_descriptor.get_w_value());
            descriptors.push(Box::new(configuration_descriptor));
            configuration_descriptors_num += 1;
        }

        // Device Strings
        let string_descriptor = device_node.manufacturer.build(string_descriptors_num);
        descriptor_values.push(string_descriptor.get_w_value());
        descriptors.push(Box::new(string_descriptor));
        let manufacturer = string_descriptors_num;
        string_descriptors_num += 1;

        let string_descriptor = device_node.product.build(string_descriptors_num);
        descriptor_values.push(string_descriptor.get_w_value());
        descriptors.push(Box::new(string_descriptor));
        let product = string_descriptors_num;
        string_descriptors_num += 1;

        let string_descriptor = device_node.serial_number.build(string_descriptors_num);
        descriptor_values.push(string_descriptor.get_w_value());
        descriptors.push(Box::new(string_descriptor));
        let serial_number = string_descriptors_num;
        // string_descriptors_num += 1;

        // Device Descriptor
        let device_descriptor = device_node
            .get_device()
            .build(num_configurations, manufacturer, product, serial_number)
            .unwrap();
        descriptor_values.push(device_descriptor.get_w_value());
        descriptors.push(Box::new(device_descriptor));

        // reverse the order of the descriptors
        descriptors.reverse();
        descriptor_values.reverse();

        Ok(DescriptorStore {
            descriptors,
            descriptor_values,
        })
    }

    /// Encode the selected descriptors into a continuous byte array
    pub fn encode(
        &self,
        include_device_descriptor: bool,
        include_configuration_descriptors: bool,
        include_interface_descriptors: bool,
        include_endpoint_descriptors: bool,
        include_string_descriptors: bool,
    ) -> Result<Vec<u8>, &str> {
        let mut bytes = Vec::<u8>::new();
        if include_device_descriptor {
            bytes.append(&mut self.descriptors[0].encode()?);
        }
        if include_configuration_descriptors {
            for value in self.descriptor_values.iter() {
                if DescriptorType::from_value(*value)? == DescriptorType::Configuration {
                    bytes.append(&mut self.descriptors[0].encode()?);
                }
            }
        }
        if include_interface_descriptors {
            for value in self.descriptor_values.iter() {
                if DescriptorType::from_value(*value)? == DescriptorType::Interface {
                    bytes.append(&mut self.descriptors[0].encode()?);
                }
            }
        }
        if include_endpoint_descriptors {
            for value in self.descriptor_values.iter() {
                if DescriptorType::from_value(*value)? == DescriptorType::Endpoint {
                    bytes.append(&mut self.descriptors[0].encode()?);
                }
            }
        }
        if include_string_descriptors {
            for value in self.descriptor_values.iter() {
                if DescriptorType::from_value(*value)? == DescriptorType::String {
                    bytes.append(&mut self.descriptors[0].encode()?);
                }
            }
        }

        Ok(bytes)
    }

    /// Find a descriptor by its wValue (8b descriptor type + 8b descriptor indentifier)
    pub fn get_descriptor(&self, value: u16) -> Result<&Box<dyn Descriptor>, &str> {
        for descriptor in self.descriptors.iter() {
            if descriptor.get_w_value() == value {
                return Ok(descriptor);
            }
        }
        Err("Descriptor not found")
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec::Vec;

    use crate::{
        binary::EncodeByte,
        configuration::{
            configuration_attributes::ConfigurationAttributes,
            configuration_node::ConfigurationNode, milliamperes::Milliamperes,
        },
        device::{
            bcd_version::US2_0, device_device_class::DeviceDeviceClass, device_node::DeviceNode,
        },
        endpoint::{
            direction::Direction, endpoint_address::EndpointAddress,
            endpoint_attributes::EndpointAttributes, endpoint_intrinsics::EndpointIntrinsics,
            sync_type::SyncType, transfer_type::TransferType, usage_type::UsageType,
        },
        interface::{interface_device_class::InterfaceDeviceClass, interface_node::InterfaceNode},
        string::{language_code::EN_US, string_intrinsics::StringIntrinsics},
    };

    use super::DescriptorStore;

    #[test]
    fn test_descriptor_store() {
        let store = DescriptorStore::new(
            DeviceNode {
                bcd_usb: US2_0,
                device_class: DeviceDeviceClass::Device,
                device_suclass: 0x00,
                device_protocol: 0x00,
                max_packet_size_0: 0x40,
                id_vendor: 0x1234,
                id_product: 0x1234,
                bcd_device: US2_0,
                manufacturer: StringIntrinsics::text("Manufacturer"),
                product: StringIntrinsics::text("Product"),
                serial_number: StringIntrinsics::text("12345"),
                configurations: vec![ConfigurationNode {
                    configuration: StringIntrinsics::text("Configuration 0"),
                    configuration_value: 0x01,
                    attributes: ConfigurationAttributes {
                        self_powered: false,
                        remote_wakeup: false,
                    },
                    max_power: Milliamperes(500),
                    interfaces: vec![InterfaceNode {
                        interface: StringIntrinsics::text("Interface 0"),
                        alternate_setting: 0x00,
                        interface_class: InterfaceDeviceClass::HumanInterfaceDevice,
                        interface_suclass: 0x00,
                        interface_protocol: 0x00,
                        endpoints: vec![EndpointIntrinsics {
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
            StringIntrinsics::languages(vec![EN_US]),
        )
        .unwrap();

        for i in 0..store.descriptor_values.len() {
            let value = store.descriptor_values[i];
            let descriptor = &store.descriptors[i];
            println!("{} {}", value, descriptor.get_descriptor_type())
        }

        // descriptos list and descriptor_values have to be equal
        assert_eq!(store.descriptor_values.len(), store.descriptors.len());

        // check descriptor_values for dubplicates
        let mut found_descriptor_values = Vec::<u16>::new();
        for value in store.descriptor_values.iter() {
            assert_eq!(
                found_descriptor_values.contains(value),
                false,
                "Duplicate descriptor_value found: {}",
                value
            );
            found_descriptor_values.push(*value);

            // try to get descriptor by value
            let descriptor = store.get_descriptor(*value);
            assert_eq!(
                descriptor.is_ok(),
                true,
                "Descriptor not found for value: {}",
                value
            );

            // check if descriptor type matches forst 8 bits of value
            let descriptor_type = descriptor.unwrap().get_descriptor_type();
            assert_eq!(
                descriptor_type.encode().unwrap(),
                (*value >> 8) as u8,
                "Descriptor type does not match value: {}",
                value
            );
        }
    }
}
