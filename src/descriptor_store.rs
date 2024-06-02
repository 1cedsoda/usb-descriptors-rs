use alloc::{boxed::Box, vec::Vec};

use crate::{
    descriptor::Descriptor,
    descriptor_type::DescriptorType,
    device::device_node::DeviceNode,
    string::{string::String, string_content::StringContent},
};

/// Stores descriptors for USB protocol
pub struct DescriptorStore {
    /// A list of stored descriptors
    descriptors: Vec<Box<dyn Descriptor>>,
    /// Acts as a lookup table for the descriptors (same length and order)
    descriptor_w_values: Vec<u16>,
}

impl DescriptorStore {
    /// Fill the descriptor store by providing a tree of descriptor nodes
    ///
    /// `new` converts the nodes into descriptors by calculating contextual values like indices and lengths
    pub fn new(
        device_node: DeviceNode,
        languages: String,
    ) -> Result<DescriptorStore, &'static str> {
        match &languages.b_string {
            StringContent::Languages(languages) => match languages.len() {
                0 => return Err("At least one language is required"),
                _ => {}
            },
            _ => return Err("Only languages are supported"),
        }

        let mut descriptors = Vec::<Box<dyn Descriptor>>::new();
        let mut descriptor_w_values = Vec::<u16>::new();
        let mut configuration_descriptors_num: u8 = 0;
        let mut interface_descriptors_num: u8 = 0;
        let mut string_descriptors_num: u8 = 0;

        // String Descriptor (Language)
        let string_descriptor = languages.build(string_descriptors_num);
        descriptor_w_values.push(string_descriptor.get_w_value());
        descriptors.push(Box::new(string_descriptor));
        string_descriptors_num += 1;

        // Iterate Nodes
        let b_num_configurations = device_node.configurations.len() as u8;
        for configuration_node in &device_node.configurations {
            let b_num_interfaces = configuration_node.interfaces.len() as u8;
            let mut w_total_length = 0;
            for interface_node in &configuration_node.interfaces {
                let b_num_endpoints = interface_node.endpoints.len() as u8;
                for endpoint in &interface_node.endpoints {
                    // Endpoint Descriptor
                    let endpoint_descriptor = endpoint.build().unwrap();
                    w_total_length += endpoint_descriptor.b_length as u16;
                    descriptor_w_values.push(endpoint_descriptor.get_w_value());
                    descriptors.push(Box::new(endpoint_descriptor));
                }

                // Interface String Descriptor
                let string_descriptor = interface_node.interface.build(string_descriptors_num);
                descriptor_w_values.push(string_descriptor.get_w_value());
                descriptors.push(Box::new(string_descriptor));
                string_descriptors_num += 1;

                // Interface Descriptor
                let interface_descriptor = interface_node
                    .get_interface()
                    .build(
                        interface_descriptors_num,
                        string_descriptors_num,
                        b_num_endpoints,
                    )
                    .unwrap();
                w_total_length += interface_descriptor.b_length as u16;
                descriptor_w_values.push(interface_descriptor.get_w_value());
                descriptors.push(Box::new(interface_descriptor));
                interface_descriptors_num += 1;
            }

            // Configuration String Descriptor
            let string_descriptor = configuration_node
                .configuration
                .build(string_descriptors_num);
            descriptor_w_values.push(string_descriptor.get_w_value());
            descriptors.push(Box::new(string_descriptor));
            string_descriptors_num += 1;

            // Configuration Descriptor
            let configuration_descriptor = configuration_node
                .get_configuration()
                .build(
                    w_total_length,
                    b_num_interfaces,
                    configuration_descriptors_num,
                    string_descriptors_num,
                )
                .unwrap();
            descriptor_w_values.push(configuration_descriptor.get_w_value());
            descriptors.push(Box::new(configuration_descriptor));
            configuration_descriptors_num += 1;
        }

        // Device Strings
        let string_descriptor = device_node.manufacturer.build(string_descriptors_num);
        descriptor_w_values.push(string_descriptor.get_w_value());
        descriptors.push(Box::new(string_descriptor));
        let i_manufacturer = string_descriptors_num;
        string_descriptors_num += 1;

        let string_descriptor = device_node.product.build(string_descriptors_num);
        descriptor_w_values.push(string_descriptor.get_w_value());
        descriptors.push(Box::new(string_descriptor));
        let i_product = string_descriptors_num;
        string_descriptors_num += 1;

        let string_descriptor = device_node.serial_number.build(string_descriptors_num);
        descriptor_w_values.push(string_descriptor.get_w_value());
        descriptors.push(Box::new(string_descriptor));
        let i_serial_number = string_descriptors_num;
        // string_descriptors_num += 1;

        // Device Descriptor
        let device_descriptor = device_node
            .get_device()
            .build(
                b_num_configurations,
                i_manufacturer,
                i_product,
                i_serial_number,
            )
            .unwrap();
        descriptor_w_values.push(device_descriptor.get_w_value());
        descriptors.push(Box::new(device_descriptor));

        // reverse the order of the descriptors
        descriptors.reverse();
        descriptor_w_values.reverse();

        Ok(DescriptorStore {
            descriptors,
            descriptor_w_values,
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
            for w_value in self.descriptor_w_values.iter() {
                if DescriptorType::from_w_value(*w_value)? == DescriptorType::Configuration {
                    bytes.append(&mut self.descriptors[0].encode()?);
                }
            }
        }
        if include_interface_descriptors {
            for w_value in self.descriptor_w_values.iter() {
                if DescriptorType::from_w_value(*w_value)? == DescriptorType::Interface {
                    bytes.append(&mut self.descriptors[0].encode()?);
                }
            }
        }
        if include_endpoint_descriptors {
            for w_value in self.descriptor_w_values.iter() {
                if DescriptorType::from_w_value(*w_value)? == DescriptorType::Endpoint {
                    bytes.append(&mut self.descriptors[0].encode()?);
                }
            }
        }
        if include_string_descriptors {
            for w_value in self.descriptor_w_values.iter() {
                if DescriptorType::from_w_value(*w_value)? == DescriptorType::String {
                    bytes.append(&mut self.descriptors[0].encode()?);
                }
            }
        }

        Ok(bytes)
    }

    /// Find a descriptor by its wValue (8b descriptor type + 8b descriptor indentifier)
    pub fn get_descriptor(&self, w_value: u16) -> Result<&Box<dyn Descriptor>, &str> {
        for descriptor in self.descriptors.iter() {
            if descriptor.get_w_value() == w_value {
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
            bcd_version::USB_2_0, device_device_class::DeviceDeviceClass, device_node::DeviceNode,
        },
        endpoint::{
            direction::Direction, endpoint::Endpoint, endpoint_address::EndpointAddress,
            endpoint_attributes::EndpointAttributes, sync_type::SyncType,
            transfer_type::TransferType, usage_type::UsageType,
        },
        interface::{interface_device_class::InterfaceDeviceClass, interface_node::InterfaceNode},
        string::{language_code::EN_US, string::String},
    };

    use super::DescriptorStore;

    #[test]
    fn test_descriptor_store() {
        let store = DescriptorStore::new(
            DeviceNode {
                bcd_usb: USB_2_0,
                b_device_class: DeviceDeviceClass::Device,
                b_device_sub_class: 0x00,
                b_device_protocol: 0x00,
                b_max_packet_size_0: 0x40,
                id_vendor: 0x1234,
                id_product: 0x1234,
                bcd_device: USB_2_0,
                manufacturer: String::text("Manufacturer"),
                product: String::text("Product"),
                serial_number: String::text("12345"),
                configurations: vec![ConfigurationNode {
                    configuration: String::text("Configuration 0"),
                    b_configuration_value: 0x01,
                    bm_attributes: ConfigurationAttributes {
                        self_powered: false,
                        remote_wakeup: false,
                    },
                    b_max_power: Milliamperes(500),
                    interfaces: vec![InterfaceNode {
                        interface: String::text("Interface 0"),
                        b_alternate_setting: 0x00,
                        b_interface_class: InterfaceDeviceClass::HumanInterfaceDevice,
                        b_interface_sub_class: 0x00,
                        b_interface_protocol: 0x00,
                        endpoints: vec![Endpoint {
                            b_endpoint_address: EndpointAddress {
                                endpoint_number: 0x01,
                                direction: Direction::Out,
                            },
                            bm_attributes: EndpointAttributes {
                                transfer_type: TransferType::Interrupt,
                                sync_type: SyncType::NoSync,
                                usage_type: UsageType::Data,
                            },
                            w_max_packet_size: 0x40,
                            b_interval: 0x01,
                        }],
                    }],
                }],
            },
            String::languages(vec![EN_US]),
        )
        .unwrap();

        for i in 0..store.descriptor_w_values.len() {
            let w_value = store.descriptor_w_values[i];
            let descriptor = &store.descriptors[i];
            println!("{} {}", w_value, descriptor.get_descriptor_type())
        }

        // descriptos list and descriptor_w_values have to be equal
        assert_eq!(store.descriptor_w_values.len(), store.descriptors.len());

        // check descriptor_w_values for dubplicates
        let mut found_descriptor_w_values = Vec::<u16>::new();
        for w_value in store.descriptor_w_values.iter() {
            assert_eq!(
                found_descriptor_w_values.contains(w_value),
                false,
                "Duplicate descriptor_w_value found: {}",
                w_value
            );
            found_descriptor_w_values.push(*w_value);

            // try to get descriptor by w_value
            let descriptor = store.get_descriptor(*w_value);
            assert_eq!(
                descriptor.is_ok(),
                true,
                "Descriptor not found for w_value: {}",
                w_value
            );

            // check if descriptor type matches forst 8 bits of w_value
            let descriptor_type = descriptor.unwrap().get_descriptor_type();
            assert_eq!(
                descriptor_type.encode().unwrap(),
                (*w_value >> 8) as u8,
                "Descriptor type does not match w_value: {}",
                w_value
            );
        }
    }
}
