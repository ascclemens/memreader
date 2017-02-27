extern crate kernel32;
extern crate winapi;

use self::kernel32::{ReadProcessMemory, OpenProcess};
use self::winapi::winnt::HANDLE;

use {ReadsMemory, ProvidesSlices};
use slice::MemorySlice;
use error::*;

pub struct MemReader {
  handle: HANDLE
}

impl MemReader {
  pub fn new(pid: u32) -> Result<MemReader> {
    let handle = unsafe { OpenProcess(0x0010, 0, pid) };
    if handle.is_null() {
      return Err(MemReaderError::Handle(None));
    }
    Ok(MemReader {
      handle: handle
    })
  }
}

impl ReadsMemory for MemReader {
  fn read_bytes(&self, address: usize, n: usize) -> Result<Vec<u8>> {
    let mut buffer: Vec<u8> = vec![0; n];
    let mut read: u64 = unsafe { ::std::mem::uninitialized() };
    let res = unsafe {
      ReadProcessMemory(self.handle,
        address as *const _,
        buffer.as_mut_ptr() as *mut _,
        n as u64,
        &mut read as *mut _)
    };
    if res != 1 {
      return Err(MemReaderError::UnsuccessfulRead(Some(res)));
    }
    if read != n as u64 {
      return Err(MemReaderError::FewerBytesRead(read as usize, buffer[..n].to_vec()));
    }
    Ok(buffer)
  }
}

impl ProvidesSlices for MemReader {
  fn address_slice(&self, start: usize, end: usize) -> MemorySlice {
    MemorySlice::new(self, start, end)
  }

  fn address_slice_len(&self, start: usize, n: usize) -> MemorySlice {
    MemorySlice::new(self, start, start + n)
  }
}
