extern crate kernel32;
extern crate winapi;

use self::kernel32::{ReadProcessMemory, OpenProcess, K32EnumProcessModulesEx, K32GetModuleBaseNameA, GetLastError};
use self::winapi::minwindef::{HMODULE, DWORD, MAX_PATH};
use self::winapi::psapi::{LIST_MODULES_32BIT, LIST_MODULES_64BIT};
use self::winapi::winnt::{PROCESS_VM_READ, PROCESS_QUERY_INFORMATION};

use std::sync::atomic::{AtomicPtr, Ordering};
use std::mem::{size_of, size_of_val, uninitialized};
use std::ffi::CStr;

use {ReadsMemory, ProvidesSlices};
use slice::MemorySlice;
use error::*;

pub struct MemReader {
  handle: AtomicPtr<::std::os::raw::c_void>
}

impl MemReader {
  pub fn new(pid: u32) -> Result<MemReader> {
    let handle = unsafe { OpenProcess(PROCESS_VM_READ | PROCESS_QUERY_INFORMATION, 0, pid) };
    if handle.is_null() {
      return Err(MemReaderError::Handle(None));
    }
    Ok(MemReader {
      handle: AtomicPtr::new(handle)
    })
  }

  pub fn base_address(&self, process_name: &str) -> Option<usize> {
    let mut hmod: HMODULE = unsafe { uninitialized() };
    let mut cb_needed: DWORD = unsafe { uninitialized() };
    let res = unsafe {
      K32EnumProcessModulesEx(self.handle.load(Ordering::Relaxed),
        &mut hmod as *mut HMODULE,
        size_of_val(&hmod) as u32,
        &mut cb_needed as *mut DWORD,
        LIST_MODULES_32BIT | LIST_MODULES_64BIT
      )
    };
    if res == 0 {
      return None;
    }
    let mut base_name: [::std::os::raw::c_char; MAX_PATH] = [0; MAX_PATH];
    unsafe {
      K32GetModuleBaseNameA(self.handle.load(Ordering::Relaxed),
        hmod,
        &mut base_name[0] as *mut _,
        base_name.len() as u32 / size_of::<::std::os::raw::c_char>() as u32
      );
    }
    let base_name = unsafe { CStr::from_ptr(&base_name[0] as *const _) };
    let base_name = match base_name.to_str() {
      Ok(n) => n,
      Err(_) => return None
    };
    if base_name.to_lowercase() == process_name.to_lowercase() {
      Some(hmod as usize)
    } else {
      None
    }
  }
}

impl ReadsMemory for MemReader {
  fn read_bytes(&self, address: usize, n: usize) -> Result<Vec<u8>> {
    let mut buffer: Vec<u8> = vec![0; n];
    let mut read: u64 = unsafe { ::std::mem::uninitialized() };
    let res = unsafe {
      ReadProcessMemory(self.handle.load(Ordering::Relaxed),
        address as *const _,
        buffer.as_mut_ptr() as *mut _,
        n as u64,
        &mut read as *mut _)
    };
    if res != 1 {
      return Err(MemReaderError::UnsuccessfulRead(Some(res as isize)));
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
