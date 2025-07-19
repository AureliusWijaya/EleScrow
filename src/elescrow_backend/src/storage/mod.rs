pub mod memory;
pub mod stable_storage;

pub use memory::{get_memory, MemoryRegion, MemoryStats};
pub use stable_storage::{StableStorage, IndexedStorage, TimeSeriesStorage};