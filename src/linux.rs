use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

use {ReadsMemory, ProvidesSlices};
use slice::MemorySlice;
use error::*;

pub struct MemReader {
  pid: u32
}

impl MemReader {
  pub fn new(pid: u32) -> Result<MemReader> {
    Ok(MemReader {
      pid: pid
    })
  }

  fn get_memory_file(&self) -> Result<File> {
    File::open(&format!("/proc/{}/mem", self.pid)).map_err(|e| MemReaderError::Io(e))
  }
}

impl ReadsMemory for MemReader {
  fn read_bytes(&self, address: usize, n: usize) -> Result<Vec<u8>> {
    let mut file = self.get_memory_file()?;
    file.seek(SeekFrom::Start(address as u64)).map_err(|e| MemReaderError::Io(e))?;
    let mut bytes: Vec<u8> = vec![0; n];
    file.read_exact(&mut bytes).map_err(|e| MemReaderError::Io(e))?;
    Ok(bytes)
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
