#![no_std]

extern crate alloc;

#[cfg(test)]
#[macro_use]
extern crate std;

pub mod configuration;
pub mod device;
pub mod endpoint;
pub mod interface;
pub mod string;

pub mod binary;
pub mod descriptor;
pub mod descriptors;
pub mod descriptors_builder;
pub mod descriptor_type;
pub mod version;
