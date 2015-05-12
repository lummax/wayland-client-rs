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
    static wl_data_source_interface: ffi::wayland::WLInterface;
}

#[repr(C)]
enum DataSourceEvent {
    Target = 0,
    Send = 1,
    Cancelled = 2,
}

impl FromPrimitive for DataSourceEvent {
    fn from_u32(num: u32) -> Option<Self> {
        return match num {
            0 => Some(DataSourceEvent::Target),
            1 => Some(DataSourceEvent::Send),
            2 => Some(DataSourceEvent::Cancelled),
            _ => None
        }
    }

    fn from_i32(num: i32) -> Option<Self> {
        return Self::from_u32(num as u32);
    }
}

#[repr(C)]
enum DataSourceRequest {
    Offer = 0,
    Destroy = 1,
}

impl FromPrimitive for DataSourceRequest {
    fn from_u32(num: u32) -> Option<Self> {
        return match num {
            0 => Some(DataSourceRequest::Offer),
            1 => Some(DataSourceRequest::Destroy),
            _ => None
        }
    }

    fn from_i32(num: i32) -> Option<Self> {
        return Self::from_u32(num as u32);
    }
}

/// The wl_data_source object is the source side of a wl_data_offer.
/// It is created by the source client in a data transfer and
/// provides a way to describe the offered data and a way to respond
/// to requests to transfer the data.
#[derive(Debug)]
pub struct DataSource {
    proxy: BaseProxy,
}

impl DataSource {
    
    /// This request adds a mime type to the set of mime types
    /// advertised to targets.  Can be called several times to offer
    /// multiple types.
    pub fn offer(&mut self, mime_type: &str) {
        let proxy = self.as_mut_ptr() as *mut ffi::wayland::WLProxy;
        let mime_typepointer = CString::new(mime_type).unwrap().as_ptr();
        unsafe {
            ffi::wayland::wl_proxy_marshal(
                proxy, DataSourceRequest::Offer as u32, mime_typepointer
            );
        }
    }
    
    /// Destroy the data source.
    pub fn destroy(mut self) {
        let proxy = self.as_mut_ptr() as *mut ffi::wayland::WLProxy;
        unsafe {
            ffi::wayland::wl_proxy_marshal(
                proxy, DataSourceRequest::Destroy as u32
            );
        }
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


impl FromRawPtr<ffi::wayland::WLProxy> for DataSource {
    fn from_mut_ptr(ptr: *mut ffi::wayland::WLProxy) -> Result<Self, &'static str> {
        return match FromRawPtr::from_mut_ptr(ptr) {
            Ok(proxy) => Ok(DataSource {
                proxy: proxy,
            }),
            Err(str) => Err(str),
        }
    }
}

impl AsRawPtr<ffi::wayland::WLProxy> for DataSource {
    fn as_mut_ptr(&mut self) -> *mut ffi::wayland::WLProxy {
        return self.proxy.as_mut_ptr();
    }
}

impl GetInterface for DataSource {
    fn get_interface() -> *const ffi::wayland::WLInterface {
        return &wl_data_source_interface as *const ffi::wayland::WLInterface;
    }
}

#[allow(unused_variables)]
extern fn data_source_event_dispatcher<T: DataSourceEventHandler>(
        user_data: *mut c_void,
        _target: *mut c_void,
        opcode: uint32_t,
        _message: *const ffi::wayland::WLMessage,
        arguments: *mut ffi::wayland::WLArgument) -> c_int {
    let object = user_data as *mut T;
    return match DataSourceEvent::from_u32(opcode) {
        Some(event) => {
            match event {
                DataSourceEvent::Target => {
                    let mime_type_raw = unsafe { *(*arguments.offset(0)).string() };
                    let mime_type_buffer = unsafe { CStr::from_ptr(mime_type_raw).to_bytes() };
                    let mime_type = String::from_utf8(mime_type_buffer.to_vec()).unwrap();
                    unsafe { (*object).on_target(mime_type); }
                },
                DataSourceEvent::Send => {
                    let mime_type_raw = unsafe { *(*arguments.offset(0)).string() };
                    let mime_type_buffer = unsafe { CStr::from_ptr(mime_type_raw).to_bytes() };
                    let mime_type = String::from_utf8(mime_type_buffer.to_vec()).unwrap();
                    let fd = unsafe { *(*arguments.offset(1)).file_descriptor() };
                    unsafe { (*object).on_send(mime_type, fd); }
                },
                DataSourceEvent::Cancelled => {
                    unsafe { (*object).on_cancelled(); }
                },
            }
            0
        },
        _ => -1,
    }
}

pub trait DataSourceEventHandler: Sized {
    fn connect_dispatcher(&mut self) {
        unsafe {
            ffi::wayland::wl_proxy_add_dispatcher(
                self.get_data_source().as_mut_ptr(),
                data_source_event_dispatcher::<Self>,
                self as *mut Self as *mut c_void,
                ptr::null_mut());
        }
    }

    fn get_data_source(&mut self) -> &mut DataSource;
    
    /// Sent when a target accepts pointer_focus or motion events.  If
    /// a target does not accept any of the offered types, type is NULL.
    /// 
    /// Used for feedback during drag-and-drop.
    #[allow(unused_variables)]
    fn on_target(&mut self, mime_type: String) {}
    
    /// Request for data from the client.  Send the data as the
    /// specified mime type over the passed file descriptor, then
    /// close it.
    #[allow(unused_variables)]
    fn on_send(&mut self, mime_type: String, fd: RawFd) {}
    
    /// This data source has been replaced by another data source.
    /// The client should clean up and destroy this data source.
    #[allow(unused_variables)]
    fn on_cancelled(&mut self) {}
}
