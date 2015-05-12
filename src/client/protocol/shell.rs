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
    static wl_shell_interface: ffi::wayland::WLInterface;
}

#[repr(C)]
#[derive(Debug)]
pub enum ShellError {
    /// given wl_surface has another role
    Role = 0,
    
    _Dummy,
}

impl FromPrimitive for ShellError {
    fn from_u32(num: u32) -> Option<Self> {
        return match num {
            0 => Some(ShellError::Role),
            _ => None
        }
    }

    fn from_i32(num: i32) -> Option<Self> {
        return Self::from_u32(num as u32);
    }
}

pub trait ShellErrorSet {
    fn has_role(&self) -> bool;
    fn has_(&self) -> bool;
}

impl ShellErrorSet for u32 {
    fn is_role(&self) -> bool {
        return self & (ShellError::Role as u32) != 0;
    }
    fn is_(&self) -> bool {
        return self & (ShellError::_Dummy as u32) != 0;
    }
}

impl ShellErrorSet for i32 {
    fn is_role(&self) -> bool {
        return self & (ShellError::Role as i32) != 0;
    }
    fn is_(&self) -> bool {
        return self & (ShellError::_Dummy as i32) != 0;
    }
}



#[repr(C)]
enum ShellRequest {
    GetShellSurface = 0,
    _Dummy,
}

impl FromPrimitive for ShellRequest {
    fn from_u32(num: u32) -> Option<Self> {
        return match num {
            0 => Some(ShellRequest::GetShellSurface),
            _ => None
        }
    }

    fn from_i32(num: i32) -> Option<Self> {
        return Self::from_u32(num as u32);
    }
}



/// This interface is implemented by servers that provide
/// desktop-style user interfaces.
/// 
/// It allows clients to associate a wl_shell_surface with
/// a basic surface.
#[derive(Debug)]
pub struct Shell {
    proxy: BaseProxy,
}

impl Shell {
    
    /// Create a shell surface for an existing surface. This gives
    /// the wl_surface the role of a shell surface. If the wl_surface
    /// already has another role, it raises a protocol error.
    /// 
    /// Only one shell surface can be associated with a given surface.
    pub fn get_shell_surface(&mut self, surface: &mut ::client::protocol::Surface) -> Result<::client::protocol::ShellSurface, &'static str> {
        let proxy = self.as_mut_ptr() as *mut ffi::wayland::WLProxy;
        let surfacepointer = surface.as_mut_ptr() as *mut ffi::wayland::WLProxy;
        let object = unsafe {
            ffi::wayland::wl_proxy_marshal_constructor(
                proxy, ShellRequest::GetShellSurface as u32, ::client::protocol::ShellSurface::get_interface(), ptr::null::<c_void>(), surfacepointer
            )
        };
        return ::client::protocol::ShellSurface::from_mut_ptr(object);
    }

    pub fn get_id(&mut self) -> u32 {
        return self.proxy.get_id();
    }

    pub fn get_class(&mut self) -> String {
        return self.proxy.get_class();
    }

    pub fn set_queue(&mut self, queue: Option<&mut EventQueue>) {
        self.proxy.set_queue(queue);
    }
}


impl FromRawPtr<ffi::wayland::WLProxy> for Shell {
    fn from_mut_ptr(ptr: *mut ffi::wayland::WLProxy) -> Result<Self, &'static str> {
        return match FromRawPtr::from_mut_ptr(ptr) {
            Ok(proxy) => Ok(Shell {
                proxy: proxy,
            }),
            Err(str) => Err(str),
        }
    }
}

impl AsRawPtr<ffi::wayland::WLProxy> for Shell {
    fn as_mut_ptr(&mut self) -> *mut ffi::wayland::WLProxy {
        return self.proxy.as_mut_ptr();
    }
}

impl GetInterface for Shell {
    fn get_interface() -> *const ffi::wayland::WLInterface {
        return &wl_shell_interface as *const ffi::wayland::WLInterface;
    }
}


