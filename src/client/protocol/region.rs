// Copyright © 2008-2011 Kristian Høgsberg
// Copyright © 2010-2011 Intel Corporation
// Copyright © 2012-2013 Collabora, Ltd.
// 
// Permission to use, copy, modify, distribute, and sell this
// software and its documentation for any purpose is hereby granted
// without fee, provided that the above copyright notice appear in
// all copies and that both that copyright notice and this permission
// notice appear in supporting documentation, and that the name of
// the copyright holders not be used in advertising or publicity
// pertaining to distribution of the software without specific,
// written prior permission.  The copyright holders make no
// representations about the suitability of this software for any
// purpose.  It is provided "as is" without express or implied
// warranty.
// 
// THE COPYRIGHT HOLDERS DISCLAIM ALL WARRANTIES WITH REGARD TO THIS
// SOFTWARE, INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND
// FITNESS, IN NO EVENT SHALL THE COPYRIGHT HOLDERS BE LIABLE FOR ANY
// SPECIAL, INDIRECT OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN
// AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION,
// ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF
// THIS SOFTWARE.

// Generated with version 1.7.0

#![allow(unused_imports)]

use std::{ptr, mem};
use std::ffi::{CStr, CString};
use std::os::unix::io::RawFd;
use libc::{c_void, c_int, uint32_t};

use ffi;
use client::protocol::{FromPrimitive, GetInterface};
use client::base::Proxy as BaseProxy;
use client::base::{FromRawPtr, AsRawPtr, EventQueue};

#[link(name="wayland-client")]
extern {
    static wl_region_interface: ffi::wayland::WLInterface;
}


#[repr(C)]
enum RegionRequest {
    Destroy = 0,
    Add = 1,
    Subtract = 2,
}

impl FromPrimitive for RegionRequest {
    fn from_u32(num: u32) -> Option<Self> {
        return match num {
            0 => Some(RegionRequest::Destroy),
            1 => Some(RegionRequest::Add),
            2 => Some(RegionRequest::Subtract),
            _ => None
        }
    }
}

/// A region object describes an area.
/// 
/// Region objects are used to describe the opaque and input
/// regions of a surface.
#[derive(Debug)]
pub struct Region {
    proxy: BaseProxy,
}

impl Region {
    
    /// Destroy the region.  This will invalidate the object ID.
    pub fn destroy(mut self) {
        let proxy = self.as_mut_ptr() as *mut ffi::wayland::WLProxy;
        unsafe {
            ffi::wayland::wl_proxy_marshal(
                proxy, RegionRequest::Destroy as u32
            );
        }
    }
    
    /// Add the specified rectangle to the region.
    pub fn add(&mut self, x: i32, y: i32, width: i32, height: i32) {
        let proxy = self.as_mut_ptr() as *mut ffi::wayland::WLProxy;
        unsafe {
            ffi::wayland::wl_proxy_marshal(
                proxy, RegionRequest::Add as u32, x, y, width, height
            );
        }
    }
    
    /// Subtract the specified rectangle from the region.
    pub fn subtract(&mut self, x: i32, y: i32, width: i32, height: i32) {
        let proxy = self.as_mut_ptr() as *mut ffi::wayland::WLProxy;
        unsafe {
            ffi::wayland::wl_proxy_marshal(
                proxy, RegionRequest::Subtract as u32, x, y, width, height
            );
        }
    }

    pub fn get_id(&mut self) -> u32 {
        return self.proxy.get_id();
    }

    pub fn get_class(&mut self) -> String {
        return self.proxy.get_class();
    }

    pub fn set_queue(&mut self, queue: &mut EventQueue) {
        self.proxy.set_queue(queue);
    }
}


impl FromRawPtr<ffi::wayland::WLProxy> for Region {
    fn from_mut_ptr(ptr: *mut ffi::wayland::WLProxy) -> Result<Self, &'static str> {
        return match FromRawPtr::from_mut_ptr(ptr) {
            Ok(proxy) => Ok(Region {
                proxy: proxy,
            }),
            Err(str) => Err(str),
        }
    }
}

impl AsRawPtr<ffi::wayland::WLProxy> for Region {
    fn as_mut_ptr(&mut self) -> *mut ffi::wayland::WLProxy {
        return self.proxy.as_mut_ptr();
    }
}

impl GetInterface for Region {
    fn get_interface() -> *const ffi::wayland::WLInterface {
        return &wl_region_interface as *const ffi::wayland::WLInterface;
    }
}


