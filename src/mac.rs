#![allow(non_camel_case_types)]

use libc::{c_int, c_uint, c_void, c_char, uintptr_t};
use std::mem::uninitialized;

use {ConstructsMemReader, ReadsMemory, ProvidesSlices, ProvidesBaseAddresses};
use slice::MemorySlice;
use error::*;

pub type natural_t = c_uint;
pub type kern_return_t = c_int;
pub type mach_port_name_t = natural_t;
pub type mach_port_t = natural_t;
pub type vm_map_t = mach_port_t;
pub type vm_address_t = vm_offset_t;
pub type vm_offset_t = uintptr_t;
pub type vm_size_t = uintptr_t;
pub type mach_msg_type_number_t = natural_t;
pub type mach_vm_address_t = u64;
pub type mach_vm_size_t = u64;
pub type vm_region_recurse_info_t = *mut ::std::os::raw::c_int;
pub type vm_region_submap_info_t = *mut vm_region_submap_info_64;
pub type vm_prot_t = ::std::os::raw::c_int;
pub type vm_inherit_t = ::std::os::raw::c_uint;
pub type boolean_t = ::std::os::raw::c_uint;
pub type vm_behavior_t = ::std::os::raw::c_int;
pub type vm32_object_id_t = u32;
pub type memory_object_offset_t = ::std::os::raw::c_ulonglong;

#[derive(Debug)]
#[repr(C)]
pub struct vm_region_submap_info_64 {
    pub protection: vm_prot_t,
    pub max_protection: vm_prot_t,
    pub inheritance: vm_inherit_t,
    pub offset: memory_object_offset_t,
    pub user_tag: ::std::os::raw::c_uint,
    pub pages_resident: ::std::os::raw::c_uint,
    pub pages_shared_now_private: ::std::os::raw::c_uint,
    pub pages_swapped_out: ::std::os::raw::c_uint,
    pub pages_dirtied: ::std::os::raw::c_uint,
    pub ref_count: ::std::os::raw::c_uint,
    pub shadow_depth: ::std::os::raw::c_ushort,
    pub external_pager: ::std::os::raw::c_uchar,
    pub share_mode: ::std::os::raw::c_uchar,
    pub is_submap: boolean_t,
    pub behavior: vm_behavior_t,
    pub object_id: vm32_object_id_t,
    pub user_wired_count: ::std::os::raw::c_ushort,
    pub pages_reusable: ::std::os::raw::c_uint,
}

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

  // /usr/include/libproc.h
  pub fn proc_regionfilename(pid: c_int,
                             address: u64,
                             buffer: *mut c_void,
                             buffersize: u32) -> c_int;

  // /usr/include/mach/mach_vm.h
  pub fn mach_vm_region_recurse(target_task: vm_map_t,
                                address: *mut mach_vm_address_t,
                                size: *mut mach_vm_size_t,
                                nesting_depth: *mut natural_t,
                                info: vm_region_recurse_info_t,
                                infoCnt: *mut mach_msg_type_number_t) -> kern_return_t;
}

pub struct MemReader {
  pid: u32,
  port: mach_port_name_t
}

impl ConstructsMemReader for MemReader {
  fn new(pid: u32) -> Result<MemReader> {
    let port = unsafe {
      let mut recv_port: mach_port_name_t = uninitialized();
      let res = task_for_pid(mach_task_self(), pid as c_int, &mut recv_port as *mut mach_port_name_t);
      if res != 0 {
        return Err(MemReaderError::Handle(Some(res as isize)));
      }
      recv_port
    };
    Ok(MemReader {
      pid: pid,
      port: port
    })
  }
}

impl ProvidesBaseAddresses for MemReader {
  fn base_address(&self, process_name: &str) -> Result<usize> {
    let mut depth: natural_t = 1;
    let mut address: mach_vm_address_t = 0;
    let mut size: mach_vm_size_t = 0;
    unsafe {
      loop {
        let mut info: vm_region_submap_info_64 = ::std::mem::zeroed();
        let mut count: mach_msg_type_number_t = 17;
        let res = mach_vm_region_recurse(self.port,
          &mut address as *mut _,
          &mut size as *mut _,
          &mut depth as *mut _,
          &mut info as *mut vm_region_submap_info_64 as *mut _,
          &mut count as *mut _);
        if res != 0 {
          return Err(MemReaderError::UnsuccessfulRead(Some(res as isize)));
        }
        if info.is_submap > 0 {
          depth += 1;
        } else {
          address += size;
          let mut name: [c_char; 1024] = [0; 1024];
          proc_regionfilename(self.pid as i32, address, &mut name[0] as *mut c_char as *mut _, ::std::mem::size_of_val(&name) as u32);
          let name = match ::std::ffi::CStr::from_ptr(&name[0] as *const _).to_str() {
            Err(_) => return Err(MemReaderError::CString),
            Ok(n) => n
          };
          if name.to_lowercase().contains(&process_name.to_lowercase()) {
            return Ok(address as usize);
          }
        }
      }
    }
  }
}

impl ReadsMemory for MemReader {
  fn read_bytes(&self, address: usize, n: usize) -> Result<Vec<u8>> {
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
      return Err(MemReaderError::UnsuccessfulRead(Some(res as isize)));
    }
    let copy = buf.to_vec();
    if read != n {
      return Err(MemReaderError::FewerBytesRead(read, copy[..read].to_vec()));
    }
    Ok(copy)
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
