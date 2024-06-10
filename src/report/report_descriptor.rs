use alloc::vec::Vec;

use crate::descriptor_type::DescriptorType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReportDescriptor {
    /// Turns into `bLength`
    pub length: u8,
    /// Turns into `bDescriptorType`
    pub descriptor_type: DescriptorType,
    /// The actual report descriptor items
    pub items: Vec<ApplicationCollection>,
}

pub enum Collections {
    Application(ApplicationCollection),
    Physical(PhysicalCollection),
    
}

pub struct UsagePage {
    pub usage: Usage,
    pub usage_type: UsageType,
    pub usage_minimum: u16,
    pub usage_maximum: u16,
    pub logical_minimum: i16,
    pub logical_maximum: i16,
    pub report_count: u16,
    pub report_size: u16,
}

pub enum Usage {
    GenericDesktop = 0x01,
    Simulation = 0x02,
    Vr = 0x03,
    Sport = 0x04,
    Game = 0x05,
    GenericDevice = 0x06,
    Keyboard = 0x07,
    Led = 0x08,
    Button = 0x09,
    Ordinal = 0x0A,
    Telephony = 0x0B,
    Consumer = 0x0C,
    Digitizer = 0x0D,
    Pid = 0x0F,
    Unicode = 0x10,
    AlphanumericDisplay = 0x14,
    MedicalInstruments = 0x40,
    Monitor = 0x80,
    MonitorEnumerated = 0x81,
    MonitorVirtual = 0x82,
    MonitorReserved = 0x83,
    Power = 0x84,
    PowerReserved = 0x85,
    PowerBattery = 0x86,
    PowerBatterySystem = 0x87,
    PowerSystem = 0x88,
    PowerDevice = 0x89,
    PowerCharger = 0x8A,
    PowerChargerReserved = 0x8B,
    BarCodeScanner = 0x8C,
    Scale = 0x8D,
    MagneticStripeReader = 0x8E,
    ReservedPointOfSale = 0x8F,
    CameraControl = 0x90,
    Arcade = 0x91,
    VendorDefined = 0xFF,
    Undefined = 0x00,
}
