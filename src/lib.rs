//! tt-sim-rs: Virtual Tenstorrent Device Simulator

pub mod device;
pub mod memory;
pub mod registers;
pub mod shared_memory;

pub use device::{Device, DeviceType};
pub use memory::MemoryModel;
pub use shared_memory::SharedMemory;
