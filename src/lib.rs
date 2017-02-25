#[macro_use]
extern crate cfg_if;
extern crate libc;

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

pub trait ReadsMemory {
  /// Request `n` bytes from the memory at `address`. Returns a `Vec<u8>` containing the bytes
  /// received, which may or may not be equal to `n`.
  fn read_bytes(&self, address: usize, n: usize) -> Result<Vec<u8>, libc::c_int>;
}
