use libc::c_int;

use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::ptr::null;

use ReadsMemory;

pub struct MemReader {
  pid: u32
}

impl MemReader {
  pub fn new(pid: u32) -> Result<MemReader, c_int> {
    Ok(MemReader {
      pid: pid
    })
  }

  fn get_memory_file(&self) -> Result<File, c_int> {
    File::open(&format!("/proc/{}/mem", self.pid)).map_err(|_| -1)
  }
}

impl ReadsMemory for MemReader {
  fn read_bytes(&self, address: usize, n: usize) -> Result<Vec<u8>, c_int> {
    let mut file = self.get_memory_file()?;
    file.seek(SeekFrom::Start(address as u64)).map_err(|_| -2)?;
    let mut bytes: Vec<u8> = Vec::with_capacity(n);
    unsafe { bytes.set_len(n) };
    file.read_exact(&mut bytes).map_err(|_| -3)?;
    Ok(bytes)
  }
}
