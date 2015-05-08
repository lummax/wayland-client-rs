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
    static wl_shm_interface: ffi::wayland::WLInterface;
}

/// These errors can be emitted in response to wl_shm requests.
#[repr(C)]
#[derive(Debug)]
pub enum ShmError {
    /// buffer format is not known
    InvalidFormat = 0,
    /// invalid size or stride during pool or buffer creation
    InvalidStride = 1,
    /// mmapping the file descriptor failed
    InvalidFd = 2,
}

impl FromPrimitive for ShmError {
    fn from_u32(num: u32) -> Option<Self> {
        return match num {
            0 => Some(ShmError::InvalidFormat),
            1 => Some(ShmError::InvalidStride),
            2 => Some(ShmError::InvalidFd),
            _ => None
        }
    }
}

/// This describes the memory layout of an individual pixel.
/// 
/// All renderers should support argb8888 and xrgb8888 but any other
/// formats are optional and may not be supported by the particular
/// renderer in use.
#[repr(C)]
#[derive(Debug)]
pub enum ShmFormat {
    /// 32-bit ARGB format
    Argb8888 = 0,
    /// 32-bit RGB format
    Xrgb8888 = 1,
    C8 = 0x20203843,
    Rgb332 = 0x38424752,
    Bgr233 = 0x38524742,
    Xrgb4444 = 0x32315258,
    Xbgr4444 = 0x32314258,
    Rgbx4444 = 0x32315852,
    Bgrx4444 = 0x32315842,
    Argb4444 = 0x32315241,
    Abgr4444 = 0x32314241,
    Rgba4444 = 0x32314152,
    Bgra4444 = 0x32314142,
    Xrgb1555 = 0x35315258,
    Xbgr1555 = 0x35314258,
    Rgbx5551 = 0x35315852,
    Bgrx5551 = 0x35315842,
    Argb1555 = 0x35315241,
    Abgr1555 = 0x35314241,
    Rgba5551 = 0x35314152,
    Bgra5551 = 0x35314142,
    Rgb565 = 0x36314752,
    Bgr565 = 0x36314742,
    Rgb888 = 0x34324752,
    Bgr888 = 0x34324742,
    Xbgr8888 = 0x34324258,
    Rgbx8888 = 0x34325852,
    Bgrx8888 = 0x34325842,
    Abgr8888 = 0x34324241,
    Rgba8888 = 0x34324152,
    Bgra8888 = 0x34324142,
    Xrgb2101010 = 0x30335258,
    Xbgr2101010 = 0x30334258,
    Rgbx1010102 = 0x30335852,
    Bgrx1010102 = 0x30335842,
    Argb2101010 = 0x30335241,
    Abgr2101010 = 0x30334241,
    Rgba1010102 = 0x30334152,
    Bgra1010102 = 0x30334142,
    Yuyv = 0x56595559,
    Yvyu = 0x55595659,
    Uyvy = 0x59565955,
    Vyuy = 0x59555956,
    Ayuv = 0x56555941,
    Nv12 = 0x3231564e,
    Nv21 = 0x3132564e,
    Nv16 = 0x3631564e,
    Nv61 = 0x3136564e,
    Yuv410 = 0x39565559,
    Yvu410 = 0x39555659,
    Yuv411 = 0x31315559,
    Yvu411 = 0x31315659,
    Yuv420 = 0x32315559,
    Yvu420 = 0x32315659,
    Yuv422 = 0x36315559,
    Yvu422 = 0x36315659,
    Yuv444 = 0x34325559,
    Yvu444 = 0x34325659,
}

impl FromPrimitive for ShmFormat {
    fn from_u32(num: u32) -> Option<Self> {
        return match num {
            0 => Some(ShmFormat::Argb8888),
            1 => Some(ShmFormat::Xrgb8888),
            0x20203843 => Some(ShmFormat::C8),
            0x38424752 => Some(ShmFormat::Rgb332),
            0x38524742 => Some(ShmFormat::Bgr233),
            0x32315258 => Some(ShmFormat::Xrgb4444),
            0x32314258 => Some(ShmFormat::Xbgr4444),
            0x32315852 => Some(ShmFormat::Rgbx4444),
            0x32315842 => Some(ShmFormat::Bgrx4444),
            0x32315241 => Some(ShmFormat::Argb4444),
            0x32314241 => Some(ShmFormat::Abgr4444),
            0x32314152 => Some(ShmFormat::Rgba4444),
            0x32314142 => Some(ShmFormat::Bgra4444),
            0x35315258 => Some(ShmFormat::Xrgb1555),
            0x35314258 => Some(ShmFormat::Xbgr1555),
            0x35315852 => Some(ShmFormat::Rgbx5551),
            0x35315842 => Some(ShmFormat::Bgrx5551),
            0x35315241 => Some(ShmFormat::Argb1555),
            0x35314241 => Some(ShmFormat::Abgr1555),
            0x35314152 => Some(ShmFormat::Rgba5551),
            0x35314142 => Some(ShmFormat::Bgra5551),
            0x36314752 => Some(ShmFormat::Rgb565),
            0x36314742 => Some(ShmFormat::Bgr565),
            0x34324752 => Some(ShmFormat::Rgb888),
            0x34324742 => Some(ShmFormat::Bgr888),
            0x34324258 => Some(ShmFormat::Xbgr8888),
            0x34325852 => Some(ShmFormat::Rgbx8888),
            0x34325842 => Some(ShmFormat::Bgrx8888),
            0x34324241 => Some(ShmFormat::Abgr8888),
            0x34324152 => Some(ShmFormat::Rgba8888),
            0x34324142 => Some(ShmFormat::Bgra8888),
            0x30335258 => Some(ShmFormat::Xrgb2101010),
            0x30334258 => Some(ShmFormat::Xbgr2101010),
            0x30335852 => Some(ShmFormat::Rgbx1010102),
            0x30335842 => Some(ShmFormat::Bgrx1010102),
            0x30335241 => Some(ShmFormat::Argb2101010),
            0x30334241 => Some(ShmFormat::Abgr2101010),
            0x30334152 => Some(ShmFormat::Rgba1010102),
            0x30334142 => Some(ShmFormat::Bgra1010102),
            0x56595559 => Some(ShmFormat::Yuyv),
            0x55595659 => Some(ShmFormat::Yvyu),
            0x59565955 => Some(ShmFormat::Uyvy),
            0x59555956 => Some(ShmFormat::Vyuy),
            0x56555941 => Some(ShmFormat::Ayuv),
            0x3231564e => Some(ShmFormat::Nv12),
            0x3132564e => Some(ShmFormat::Nv21),
            0x3631564e => Some(ShmFormat::Nv16),
            0x3136564e => Some(ShmFormat::Nv61),
            0x39565559 => Some(ShmFormat::Yuv410),
            0x39555659 => Some(ShmFormat::Yvu410),
            0x31315559 => Some(ShmFormat::Yuv411),
            0x31315659 => Some(ShmFormat::Yvu411),
            0x32315559 => Some(ShmFormat::Yuv420),
            0x32315659 => Some(ShmFormat::Yvu420),
            0x36315559 => Some(ShmFormat::Yuv422),
            0x36315659 => Some(ShmFormat::Yvu422),
            0x34325559 => Some(ShmFormat::Yuv444),
            0x34325659 => Some(ShmFormat::Yvu444),
            _ => None
        }
    }
}

#[repr(C)]
enum ShmEvent {
    Format = 0,
    _Dummy,
}

impl FromPrimitive for ShmEvent {
    fn from_u32(num: u32) -> Option<Self> {
        return match num {
            0 => Some(ShmEvent::Format),
            _ => None
        }
    }
}

#[repr(C)]
enum ShmRequest {
    CreatePool = 0,
    _Dummy,
}

impl FromPrimitive for ShmRequest {
    fn from_u32(num: u32) -> Option<Self> {
        return match num {
            0 => Some(ShmRequest::CreatePool),
            _ => None
        }
    }
}

/// A global singleton object that provides support for shared
/// memory.
/// 
/// Clients can create wl_shm_pool objects using the create_pool
/// request.
/// 
/// At connection setup time, the wl_shm object emits one or more
/// format events to inform clients about the valid pixel formats
/// that can be used for buffers.
#[derive(Debug)]
pub struct Shm {
    proxy: BaseProxy,
}

impl Shm {
    
    /// Create a new wl_shm_pool object.
    /// 
    /// The pool can be used to create shared memory based buffer
    /// objects.  The server will mmap size bytes of the passed file
    /// descriptor, to use as backing memory for the pool.
    pub fn create_pool(&mut self, fd: RawFd, size: i32) -> Result<::client::protocol::ShmPool, &'static str> {
        let proxy = self.as_mut_ptr() as *mut ffi::wayland::WLProxy;
        let object = unsafe {
            ffi::wayland::wl_proxy_marshal_constructor(
                proxy, ShmRequest::CreatePool as u32, ::client::protocol::ShmPool::get_interface(), ptr::null::<c_void>(), fd, size
            )
        };
        return ::client::protocol::ShmPool::from_mut_ptr(object);
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


impl FromRawPtr<ffi::wayland::WLProxy> for Shm {
    fn from_mut_ptr(ptr: *mut ffi::wayland::WLProxy) -> Result<Self, &'static str> {
        return match FromRawPtr::from_mut_ptr(ptr) {
            Ok(proxy) => Ok(Shm {
                proxy: proxy,
            }),
            Err(str) => Err(str),
        }
    }
}

impl AsRawPtr<ffi::wayland::WLProxy> for Shm {
    fn as_mut_ptr(&mut self) -> *mut ffi::wayland::WLProxy {
        return self.proxy.as_mut_ptr();
    }
}

impl GetInterface for Shm {
    fn get_interface() -> *const ffi::wayland::WLInterface {
        return &wl_shm_interface as *const ffi::wayland::WLInterface;
    }
}

#[allow(unused_variables)]
extern fn shm_event_dispatcher<T: ShmEventHandler>(
        user_data: *mut c_void,
        _target: *mut c_void,
        opcode: uint32_t,
        _message: *const ffi::wayland::WLMessage,
        arguments: *mut ffi::wayland::WLArgument) -> c_int {
    let object = user_data as *mut T;
    return match ShmEvent::from_u32(opcode) {
        Some(event) => {
            match event {
                ShmEvent::Format => {
                    let format = unsafe { *(*arguments.offset(0)).uint() };
                    unsafe { (*object).on_format(format); }
                },
                _ => {
                },
            }
            0
        },
        _ => -1,
    }
}

pub trait ShmEventHandler: Sized {
    fn connect_dispatcher(&mut self) {
        unsafe {
            ffi::wayland::wl_proxy_add_dispatcher(
                self.get_shm().as_mut_ptr(),
                shm_event_dispatcher::<Self>,
                self as *mut Self as *mut c_void,
                ptr::null_mut());
        }
    }

    fn get_shm(&mut self) -> &mut Shm;
    
    /// Informs the client about a valid pixel format that
    /// can be used for buffers. Known formats include
    /// argb8888 and xrgb8888.
    #[allow(unused_variables)]
    fn on_format(&mut self, format: u32) {}
}
