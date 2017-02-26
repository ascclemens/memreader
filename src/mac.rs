#![allow(non_camel_case_types)]

use libc::{c_int, c_uint, uintptr_t};
use std::mem::uninitialized;

use {ReadsMemory, ProvidesSlices};
use slice::MemorySlice;

pub type natural_t = c_uint;
pub type kern_return_t = c_int;
pub type mach_port_name_t = natural_t;
pub type mach_port_t = natural_t;
pub type vm_map_t = mach_port_t;
pub type vm_address_t = vm_offset_t;
pub type vm_offset_t = uintptr_t;
pub type vm_size_t = uintptr_t;
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

  // /usr/include/mach/vm_map.h
  pub fn vm_read_overwrite(target_task: vm_map_t,
                            address: vm_address_t,
                            size: vm_size_t,
                            data: vm_address_t,
                            outsize: *mut vm_size_t) -> kern_return_t;

  // /usr/include/mach/vm_map.h
  pub fn vm_deallocate(target_task: vm_map_t,
                        address: vm_address_t,
                        size: vm_size_t) -> kern_return_t;
}

pub struct MemReader {
  port: mach_port_name_t
}

impl MemReader {
  pub fn new(pid: u32) -> Result<MemReader, c_int> {
    let port = unsafe {
      let mut recv_port: mach_port_name_t = uninitialized();
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
}

impl ReadsMemory for MemReader {
  fn read_bytes(&self, address: usize, n: usize) -> Result<Vec<u8>, c_int> {
    let mut buf: Vec<u8> = vec![0; n];
    let mut read: vm_size_t = unsafe { uninitialized() };
    let res = unsafe {
      vm_read_overwrite(self.port,
        address as vm_address_t,
        n as vm_size_t,
        buf.as_mut_ptr() as usize,
        &mut read as *mut vm_size_t)
    };
    if res != 0 {
      return Err(res);
    }
    let copy = buf.to_vec();
    Ok(copy)
  }
}

impl ProvidesSlices for MemReader {
  fn address_slice<'a>(&'a self, start: usize, end: usize) -> MemorySlice<'a> {
    MemorySlice::new(self, start, end)
  }

  fn address_slice_len<'a>(&'a self, start: usize, n: usize) -> MemorySlice<'a> {
    MemorySlice::new(self, start, start + n)
  }
}
