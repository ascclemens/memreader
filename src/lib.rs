#[macro_use]
extern crate cfg_if;
extern crate libc;

pub mod slice;
pub mod error;

cfg_if! {
  if #[cfg(target_os = "macos")] {
    pub mod mac;
    pub use mac::MemReader;
  } else if #[cfg(target_os = "linux")] {
    pub mod linux;
    pub use linux::MemReader;
  } else if #[cfg(target_os = "windows")] {
    pub mod windows;
    pub use windows::MemReader;
  } else {
    panic!("Unsupported system");
  }
}

use slice::MemorySlice;
use error::*;

/// Internal trait for reading memory at an address.
trait ReadsMemory {
  /// Request `n` bytes from the memory at `address`. Returns a `Vec<u8>` containing the bytes
  /// received, which may or may not be equal to `n`.
  fn read_bytes(&self, address: usize, n: usize) -> Result<Vec<u8>>;
}

/// This trait allows for providing `MemorySlice`s that can read memory in that slice.
pub trait ProvidesSlices {
  /// Create a slice representing the memory between the `start` and `end` addresses.
  fn address_slice(&self, start: usize, end: usize) -> MemorySlice;

  /// Create a slice representing the memory between the `start` address and the address at
  /// `start + n`.
  fn address_slice_len(&self, start: usize, n: usize) -> MemorySlice;
}
