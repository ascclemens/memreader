#![allow(non_camel_case_types)]

extern crate libc;

use libc::{c_int, c_uint, c_void, size_t, memcpy};

pub type natural_t = c_uint;
pub type kern_return_t = c_int;
pub type mach_port_name_t = natural_t;
pub type mach_port_t = natural_t;
pub type vm_map_t = mach_port_t;
pub type vm_address_t = vm_offset_t;
pub type vm_offset_t = libc::uintptr_t;
pub type vm_size_t = libc::uintptr_t;
pub type mach_msg_type_number_t = natural_t;

extern {
  // /usr/include/mach/mach_traps.h
  pub fn task_for_pid(target_tport: mach_port_name_t, pid: c_int, t: *mut mach_port_name_t) -> kern_return_t;

  // /usr/include/mach/mach_init.h
  pub fn mach_task_self() -> mach_port_t;

  // /usr/include/mach/vm_map.h
  pub fn vm_read(target_task: vm_map_t,
                  address: vm_address_t,
                  size: vm_size_t,
                  data: *mut vm_offset_t,
                  dataCnt: *mut mach_msg_type_number_t) -> kern_return_t;
}

pub struct MemReader {
  port: mach_port_name_t
}

impl MemReader {
  pub fn new(pid: u32) -> Result<MemReader, c_int> {
    let port = unsafe {
      let mut recv_port: mach_port_name_t = std::mem::uninitialized();
      let res = task_for_pid(mach_task_self(), pid as c_int, &mut recv_port as *mut mach_port_name_t);
      if res != 0 {
        return Err(res);
      }
      recv_port
    };
    Ok(MemReader {
      port: port
    })
  }

  /// Request `n` bytes from the memory at `address`. Returns a `Vec<u8>` containing the bytes
  /// received, which may or may not be equal to `n`.
  pub fn read_bytes(&self, address: usize, n: usize) -> Result<Vec<u8>, c_int> {
    let mut ptr: libc::uintptr_t = unsafe { std::mem::uninitialized() };
    let mut read: libc::c_uint = unsafe { std::mem::uninitialized() };
    let res = unsafe {
      vm_read(self.port,
        address as vm_address_t,
        n as vm_size_t,
        &mut ptr as *mut libc::uintptr_t,
        &mut read as *mut libc::c_uint)
    };
    if res != 0 {
      return Err(res);
    }
    let ptr: *const c_void = unsafe { std::mem::transmute(ptr) };
    let mut buf: Vec<u8> = Vec::with_capacity(read as usize);
    unsafe { memcpy(buf.as_mut_ptr() as *mut _ as *mut c_void, ptr, read as size_t); }
    unsafe { buf.set_len(read as usize); }
    Ok(buf.to_vec())
  }
}

pub struct FileReader {
  file: std::path::PathBuf,
  data: Option<Vec<u8>>
}

impl FileReader {
  pub fn new<P>(path: P) -> Self
    where P: AsRef<std::path::Path>
  {
    FileReader {
      file: path.as_ref().to_owned(),
      data: None
    }
  }

  fn read_data(&mut self) -> Result<(), c_int> {
    if self.data.is_some() {
      return Err(-2);
    }
    let mut file = try!(std::fs::File::open(&self.file).map_err(|_| -1));
    let mut data = Vec::new();
    use std::io::Read;
    try!(file.read_to_end(&mut data).map_err(|_| -3));
    self.data = Some(data);
    Ok(())
  }

  pub fn read_bytes(&mut self, address: usize, n: usize) -> Result<Vec<u8>, c_int> {
    if self.data.is_none() {
      try!(self.read_data());
    }
    let data = self.data.clone().unwrap();
    let slice = &data[address..address + n];
    Ok(slice.to_vec())
  }
}
