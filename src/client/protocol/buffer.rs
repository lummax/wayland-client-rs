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
    static wl_buffer_interface: ffi::wayland::WLInterface;
}

#[repr(C)]
enum BufferEvent {
    Release = 0,
    _Dummy,
}

impl FromPrimitive for BufferEvent {
    fn from_u32(num: u32) -> Option<Self> {
        return match num {
            0 => Some(BufferEvent::Release),
            _ => None
        }
    }
}

#[repr(C)]
enum BufferRequest {
    Destroy = 0,
    _Dummy,
}

impl FromPrimitive for BufferRequest {
    fn from_u32(num: u32) -> Option<Self> {
        return match num {
            0 => Some(BufferRequest::Destroy),
            _ => None
        }
    }
}

/// A buffer provides the content for a wl_surface. Buffers are
/// created through factory interfaces such as wl_drm, wl_shm or
/// similar. It has a width and a height and can be attached to a
/// wl_surface, but the mechanism by which a client provides and
/// updates the contents is defined by the buffer factory interface.
#[derive(Debug)]
pub struct Buffer {
    proxy: BaseProxy,
}

impl Buffer {
    
    /// Destroy a buffer. If and how you need to release the backing
    /// storage is defined by the buffer factory interface.
    /// 
    /// For possible side-effects to a surface, see wl_surface.attach.
    pub fn destroy(mut self) {
        let proxy = self.as_mut_ptr() as *mut ffi::wayland::WLProxy;
        unsafe {
            ffi::wayland::wl_proxy_marshal(
                proxy, BufferRequest::Destroy as u32
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


impl FromRawPtr<ffi::wayland::WLProxy> for Buffer {
    fn from_mut_ptr(ptr: *mut ffi::wayland::WLProxy) -> Result<Self, &'static str> {
        return match FromRawPtr::from_mut_ptr(ptr) {
            Ok(proxy) => Ok(Buffer {
                proxy: proxy,
            }),
            Err(str) => Err(str),
        }
    }
}

impl AsRawPtr<ffi::wayland::WLProxy> for Buffer {
    fn as_mut_ptr(&mut self) -> *mut ffi::wayland::WLProxy {
        return self.proxy.as_mut_ptr();
    }
}

impl GetInterface for Buffer {
    fn get_interface() -> *const ffi::wayland::WLInterface {
        return &wl_buffer_interface as *const ffi::wayland::WLInterface;
    }
}

#[allow(unused_variables)]
extern fn buffer_event_dispatcher<T: BufferEventHandler>(
        user_data: *mut c_void,
        _target: *mut c_void,
        opcode: uint32_t,
        _message: *const ffi::wayland::WLMessage,
        arguments: *mut ffi::wayland::WLArgument) -> c_int {
    let object = user_data as *mut T;
    return match BufferEvent::from_u32(opcode) {
        Some(event) => {
            match event {
                BufferEvent::Release => {
                    unsafe { (*object).on_release(); }
                },
                _ => {
                },
            }
            0
        },
        _ => -1,
    }
}

pub trait BufferEventHandler: Sized {
    fn connect_dispatcher(&mut self) {
        unsafe {
            ffi::wayland::wl_proxy_add_dispatcher(
                self.get_buffer().as_mut_ptr(),
                buffer_event_dispatcher::<Self>,
                self as *mut Self as *mut c_void,
                ptr::null_mut());
        }
    }

    fn get_buffer(&mut self) -> &mut Buffer;
    
    /// Sent when this wl_buffer is no longer used by the compositor.
    /// The client is now free to re-use or destroy this buffer and its
    /// backing storage.
    /// 
    /// If a client receives a release event before the frame callback
    /// requested in the same wl_surface.commit that attaches this
    /// wl_buffer to a surface, then the client is immediately free to
    /// re-use the buffer and its backing storage, and does not need a
    /// second buffer for the next surface content update. Typically
    /// this is possible, when the compositor maintains a copy of the
    /// wl_surface contents, e.g. as a GL texture. This is an important
    /// optimization for GL(ES) compositors with wl_shm clients.
    #[allow(unused_variables)]
    fn on_release(&mut self) {}
}
