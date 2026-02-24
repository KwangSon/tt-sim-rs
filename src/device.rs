/// Device type enumeration
#[derive(Debug, Clone, Copy)]
pub enum DeviceType {
    /// Wormhole device
    Wormhole,
    /// Blackhole device
    Blackhole,
    /// Grayskull device (deprecated)
    Grayskull,
}
impl DeviceType {
    /// Get PCI device ID
    pub fn pci_device_id(&self) -> u16 {
        match self {
            DeviceType::Wormhole => 0x401E,
            DeviceType::Blackhole => 0xB140,
            DeviceType::Grayskull => 0xFACA,
        }
    }

    /// Get device name
    pub fn name(&self) -> &'static str {
        match self {
            DeviceType::Wormhole => "Wormhole",
            DeviceType::Blackhole => "Blackhole",
            DeviceType::Grayskull => "Grayskull",
        }
    }
}

/// Virtual Tenstorrent device
#[derive(Debug)]
pub struct Device {
    /// Device type
    pub device_type: DeviceType,
    /// Device ID
    pub device_id: u32,
    /// PCI Vendor ID
    pub vendor_id: u16,
}

impl Device {
    /// Create a new device
    pub fn new(device_type: DeviceType, device_id: u32) -> Self {
        Device {
            device_type,
            device_id,
            vendor_id: 0x1E52, // Tenstorrent vendor ID
        }
    }
}
