//! Memory model for virtual devices

/// BAR (Base Address Register) configuration
pub struct Bar {
    /// BAR index
    pub index: u8,
    /// Base address
    pub base: u64,
    /// Size
    pub size: u64,
    /// Is I/O space
    pub is_io: bool,
}

/// Memory model
pub struct MemoryModel {
    /// DRAM size
    pub dram_size: u64,
}

impl MemoryModel {
    /// Create new memory model
    pub fn new() -> Self {
        MemoryModel {
            dram_size: 8 * 1024 * 1024 * 1024, // 8GB
        }
    }
}

impl Default for MemoryModel {
    fn default() -> Self {
        Self::new()
    }
}
