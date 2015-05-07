// Copyright (c) <2015> <lummax>
// Licensed under MIT (http://opensource.org/licenses/MIT)

use std::ffi::CStr;

use ffi;
use client::base::{FromRawPtr, AsRawPtr, EventQueue};

#[derive(Debug)]
#[allow(raw_pointer_derive)]
pub struct Proxy {
    wl_object: *mut ffi::wayland::WLProxy,
}

impl Proxy {
    pub fn get_id(&mut self) -> u32 {
        return unsafe { ffi::wayland::wl_proxy_get_id(self.wl_object) };
    }

    pub fn get_class(&mut self) -> String {
        let string_ptr =  unsafe {
            ffi::wayland::wl_proxy_get_class(self.wl_object)
        };
        let buffer = unsafe { CStr::from_ptr(string_ptr).to_bytes() };
        return String::from_utf8(buffer.to_vec()).unwrap();
    }

    pub fn set_queue(&mut self, queue: &mut EventQueue) {
        return unsafe {
            ffi::wayland::wl_proxy_set_queue(self.wl_object, queue.as_mut_ptr())
        };
    }
}

impl Drop for Proxy {
    fn drop(&mut self) {
        unsafe {
            ffi::wayland::wl_proxy_destroy(self.wl_object);
        }
    }
}

impl FromRawPtr<ffi::wayland::WLProxy> for Proxy {
    fn from_mut_ptr(ptr: *mut ffi::wayland::WLProxy) -> Result<Self, &'static str> {
        if ptr.is_null() {
            return Err("pointer for Proxy is null");
        }
        return Ok(Proxy {
            wl_object: ptr,
        });
    }
}

impl AsRawPtr<ffi::wayland::WLProxy> for Proxy {
    fn as_mut_ptr(&mut self) -> *mut ffi::wayland::WLProxy {
        return self.wl_object;
    }
}
