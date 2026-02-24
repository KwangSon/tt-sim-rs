//! Shared memory interface for tt-kmd communication

/// Shared memory size (256 MB)
pub const SHARED_MEM_SIZE: usize = 256 * 1024 * 1024;

/// Device state in shared memory
pub struct DeviceState {
    /// Magic number
    pub magic: u32,
    /// Device ID
    pub device_id: u32,
    /// PCI Vendor ID
    pub vendor_id: u16,
    /// PCI Device ID
    pub device_id_pci: u16,
    /// Device ready flag
    pub ready: bool,
}

impl DeviceState {
    /// Create new device state
    pub fn new(device_id: u32, vendor_id: u16, device_id_pci: u16) -> Self {
        DeviceState {
            magic: 0x54_53_49_4D, // "TSIM"
            device_id,
            vendor_id,
            device_id_pci,
            ready: true,
        }
    }
}

/// Shared memory manager
pub struct SharedMemory {
    /// Device ID
    pub device_id: u32,
    /// Device state
    pub state: DeviceState,
}

impl SharedMemory {
    /// Create shared memory for a device
    pub fn new(device_id: u32, vendor_id: u16, device_id_pci: u16) -> Self {
        SharedMemory {
            device_id,
            state: DeviceState::new(device_id, vendor_id, device_id_pci),
        }
    }

    /// Get shared memory file path
    pub fn path(&self) -> String {
        format!("/tmp/tt-sim-rs-{}", self.device_id)
    }
}
