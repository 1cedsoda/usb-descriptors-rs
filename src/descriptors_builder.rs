use alloc::{boxed::Box, vec::Vec};

use crate::{
    binary::encode_w_value,
    configuration::{
        configuration_builder::ConfigurationBuilder,
        configuration_descriptor::CONFIGURATION_DESCRIPTOR_TYPE,
    },
    descriptor::Descriptor,
    device::{device_builder::DeviceBuilder, device_descriptor::DEVICE_DESCRIPTOR_TYPE},
    endpoint::{
        endpoint_builder::EndpointBuilder,
        endpoint_descriptor::{ENDPOINT_DESCRIPTOR_LENGTH, ENDPOINT_DESCRIPTOR_TYPE},
    },
    interface::{
        interface_builder::InterfaceBuilder,
        interface_descriptor::{INTERFACE_DESCRIPTOR_LENGTH, INTERFACE_DESCRIPTOR_TYPE},
    },
    string::{string_builder::StringBuidler, string_descriptor::STRING_DESCRIPTOR_TYPE},
};

pub struct DescriptorsBuilder {
    pub descriptors: Vec<Box<dyn Descriptor>>,
    pub w_values: Vec<u16>,
    configuration_total_length: u16,
    configuration_descriptor_index: u8, // counting up
    interface_descriptor_index: u8,     // counting up
    string_descriptor_index: u8,        // counting down
}

impl DescriptorsBuilder {
    fn add_endpoint_descriptor<'a>(&mut self, builder: &'a EndpointBuilder) -> Result<(), &'a str> {
        let descriptor = builder.build()?;
        let w_value = encode_w_value(
            &ENDPOINT_DESCRIPTOR_TYPE,
            self.configuration_descriptor_index,
        )
        .unwrap(); // TODO
        self.configuration_descriptor_index += 1;
        self.configuration_total_length += ENDPOINT_DESCRIPTOR_LENGTH as u16;
        self.descriptors.push(Box::new(descriptor));
        self.w_values.push(w_value);
        Ok(())
    }

    fn add_interface_descriptor<'a>(
        &mut self,
        builder: &'a InterfaceBuilder,
    ) -> Result<(), &'a str> {
        let num_endpoints = builder.endpoints.len() as u8;

        self.add_string_descriptor(&builder.interface)?;

        let descriptor = builder.build(
            self.interface_descriptor_index,
            self.string_descriptor_index,
            num_endpoints,
        )?;
        let w_value =
            encode_w_value(&INTERFACE_DESCRIPTOR_TYPE, self.interface_descriptor_index).unwrap(); // TODO
        self.configuration_descriptor_index += 1;
        self.configuration_total_length += INTERFACE_DESCRIPTOR_LENGTH as u16;
        self.descriptors.push(Box::new(descriptor));
        self.w_values.push(w_value);
        Ok(())
    }

    fn add_configuration_descriptor<'a>(
        &mut self,
        builder: &'a ConfigurationBuilder,
    ) -> Result<(), &'a str> {
        let num_interfaces = builder.interfaces.len() as u8;

        self.add_string_descriptor(&builder.configuration)?;

        let descriptor = builder.build(
            self.configuration_total_length,
            num_interfaces,
            self.configuration_descriptor_index,
            self.string_descriptor_index,
        )?;
        let w_value = encode_w_value(
            &CONFIGURATION_DESCRIPTOR_TYPE,
            self.interface_descriptor_index,
        )
        .unwrap(); // TODO
        self.configuration_descriptor_index += 1;
        self.configuration_total_length = 0;
        self.descriptors.push(Box::new(descriptor));
        self.w_values.push(w_value);
        Ok(())
    }

    fn add_device_descriptor<'a>(&mut self, builder: &'a DeviceBuilder) -> Result<(), &'a str> {
        let num_configurations = builder.configurations.len() as u8;

        self.add_string_descriptor(&builder.serial_number)?;
        self.add_string_descriptor(&builder.product)?;
        self.add_string_descriptor(&builder.manufacturer)?;

        let descriptor = builder.build(
            num_configurations,
            self.string_descriptor_index + 1,
            self.string_descriptor_index + 2,
            self.string_descriptor_index + 3,
        )?;
        let w_value =
            encode_w_value(&DEVICE_DESCRIPTOR_TYPE, self.interface_descriptor_index).unwrap(); // TODO
        self.configuration_descriptor_index += 1;
        self.configuration_total_length = 0;
        self.descriptors.push(Box::new(descriptor));
        self.w_values.push(w_value);
        Ok(())
    }

    fn add_string_descriptor<'a>(&mut self, builder: &'a StringBuidler) -> Result<(), &'a str> {
        let string_index = self.string_descriptor_index;
        if string_index > 0 {
            self.string_descriptor_index -= 1;
        }
        let descriptor = builder.build(string_index);
        let w_value = encode_w_value(&STRING_DESCRIPTOR_TYPE, string_index)?;
        self.descriptors.push(Box::new(descriptor));
        self.w_values.push(w_value);
        Ok(())
    }

    pub fn build<'a>(
        device_builder: &'a DeviceBuilder,
        language: &'a StringBuidler,
    ) -> Result<DescriptorsBuilder, &'a str> {
        let mut helper = DescriptorsBuilder::default();

        let mut string_descriptor_num = 4; // 1x language + 3x in device descriptor
        for configuration_builder in device_builder.configurations.iter() {
            for _ in configuration_builder.interfaces.iter() {
                string_descriptor_num += 1;
            }
            string_descriptor_num += 1;
        }
        helper.string_descriptor_index = string_descriptor_num as u8 - 1;

        for configuration_builder in device_builder.configurations.iter().rev() {
            for interface_builder in configuration_builder.interfaces.iter().rev() {
                for endpoint_builder in interface_builder.endpoints.iter().rev() {
                    helper.add_endpoint_descriptor(endpoint_builder)?;
                }
                helper.add_interface_descriptor(interface_builder)?;
            }
            helper.add_configuration_descriptor(configuration_builder)?;
        }
        helper.add_device_descriptor(&device_builder)?;
        helper.add_string_descriptor(&language)?;

        helper.descriptors.reverse();
        helper.w_values.reverse();

        Ok(helper)
    }
}

impl Default for DescriptorsBuilder {
    fn default() -> Self {
        DescriptorsBuilder {
            descriptors: Vec::<Box<dyn Descriptor>>::new(),
            w_values: Vec::<u16>::new(),
            configuration_total_length: 0,
            configuration_descriptor_index: 0,
            interface_descriptor_index: 0,
            string_descriptor_index: 0,
        }
    }
}
