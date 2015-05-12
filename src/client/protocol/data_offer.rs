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
    static wl_data_offer_interface: ffi::wayland::WLInterface;
}

#[repr(C)]
enum DataOfferEvent {
    Offer = 0,
    _Dummy,
}

impl FromPrimitive for DataOfferEvent {
    fn from_u32(num: u32) -> Option<Self> {
        return match num {
            0 => Some(DataOfferEvent::Offer),
            _ => None
        }
    }

    fn from_i32(num: i32) -> Option<Self> {
        return Self::from_u32(num as u32);
    }
}

#[repr(C)]
enum DataOfferRequest {
    Accept = 0,
    Receive = 1,
    Destroy = 2,
}

impl FromPrimitive for DataOfferRequest {
    fn from_u32(num: u32) -> Option<Self> {
        return match num {
            0 => Some(DataOfferRequest::Accept),
            1 => Some(DataOfferRequest::Receive),
            2 => Some(DataOfferRequest::Destroy),
            _ => None
        }
    }

    fn from_i32(num: i32) -> Option<Self> {
        return Self::from_u32(num as u32);
    }
}

/// A wl_data_offer represents a piece of data offered for transfer
/// by another client (the source client).  It is used by the
/// copy-and-paste and drag-and-drop mechanisms.  The offer
/// describes the different mime types that the data can be
/// converted to and provides the mechanism for transferring the
/// data directly from the source client.
#[derive(Debug)]
pub struct DataOffer {
    proxy: BaseProxy,
}

impl DataOffer {
    
    /// Indicate that the client can accept the given mime type, or
    /// NULL for not accepted.
    /// 
    /// Used for feedback during drag-and-drop.
    pub fn accept(&mut self, serial: u32, mime_type: Option<&str>) {
        let proxy = self.as_mut_ptr() as *mut ffi::wayland::WLProxy;
        let mime_typepointer = mime_type.map(|o| CString::new(o).unwrap().as_ptr()).unwrap_or(ptr::null());
        unsafe {
            ffi::wayland::wl_proxy_marshal(
                proxy, DataOfferRequest::Accept as u32, serial, mime_typepointer
            );
        }
    }
    
    /// To transfer the offered data, the client issues this request
    /// and indicates the mime type it wants to receive.  The transfer
    /// happens through the passed file descriptor (typically created
    /// with the pipe system call).  The source client writes the data
    /// in the mime type representation requested and then closes the
    /// file descriptor.
    /// 
    /// The receiving client reads from the read end of the pipe until
    /// EOF and then closes its end, at which point the transfer is
    /// complete.
    pub fn receive(&mut self, mime_type: &str, fd: RawFd) {
        let proxy = self.as_mut_ptr() as *mut ffi::wayland::WLProxy;
        let mime_typepointer = CString::new(mime_type).unwrap().as_ptr();
        unsafe {
            ffi::wayland::wl_proxy_marshal(
                proxy, DataOfferRequest::Receive as u32, mime_typepointer, fd
            );
        }
    }
    
    /// Destroy the data offer.
    pub fn destroy(mut self) {
        let proxy = self.as_mut_ptr() as *mut ffi::wayland::WLProxy;
        unsafe {
            ffi::wayland::wl_proxy_marshal(
                proxy, DataOfferRequest::Destroy as u32
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


impl FromRawPtr<ffi::wayland::WLProxy> for DataOffer {
    fn from_mut_ptr(ptr: *mut ffi::wayland::WLProxy) -> Result<Self, &'static str> {
        return match FromRawPtr::from_mut_ptr(ptr) {
            Ok(proxy) => Ok(DataOffer {
                proxy: proxy,
            }),
            Err(str) => Err(str),
        }
    }
}

impl AsRawPtr<ffi::wayland::WLProxy> for DataOffer {
    fn as_mut_ptr(&mut self) -> *mut ffi::wayland::WLProxy {
        return self.proxy.as_mut_ptr();
    }
}

impl GetInterface for DataOffer {
    fn get_interface() -> *const ffi::wayland::WLInterface {
        return &wl_data_offer_interface as *const ffi::wayland::WLInterface;
    }
}

#[allow(unused_variables)]
extern fn data_offer_event_dispatcher<T: DataOfferEventHandler>(
        user_data: *mut c_void,
        _target: *mut c_void,
        opcode: uint32_t,
        _message: *const ffi::wayland::WLMessage,
        arguments: *mut ffi::wayland::WLArgument) -> c_int {
    let object = user_data as *mut T;
    return match DataOfferEvent::from_u32(opcode) {
        Some(event) => {
            match event {
                DataOfferEvent::Offer => {
                    let mime_type_raw = unsafe { *(*arguments.offset(0)).string() };
                    let mime_type_buffer = unsafe { CStr::from_ptr(mime_type_raw).to_bytes() };
                    let mime_type = String::from_utf8(mime_type_buffer.to_vec()).unwrap();
                    unsafe { (*object).on_offer(mime_type); }
                },
                _ => {
                },
            }
            0
        },
        _ => -1,
    }
}

pub trait DataOfferEventHandler: Sized {
    fn connect_dispatcher(&mut self) {
        unsafe {
            ffi::wayland::wl_proxy_add_dispatcher(
                self.get_data_offer().as_mut_ptr(),
                data_offer_event_dispatcher::<Self>,
                self as *mut Self as *mut c_void,
                ptr::null_mut());
        }
    }

    fn get_data_offer(&mut self) -> &mut DataOffer;
    
    /// Sent immediately after creating the wl_data_offer object.  One
    /// event per offered mime type.
    #[allow(unused_variables)]
    fn on_offer(&mut self, mime_type: String) {}
}
