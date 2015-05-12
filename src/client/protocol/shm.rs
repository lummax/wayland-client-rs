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
    INVALIDFORMAT = 0,
    /// invalid size or stride during pool or buffer creation
    INVALIDSTRIDE = 1,
    /// mmapping the file descriptor failed
    INVALIDFD = 2,
}

impl FromPrimitive for ShmError {
    fn from_u32(num: u32) -> Option<Self> {
        return match num {
            0 => Some(ShmError::INVALIDFORMAT),
            1 => Some(ShmError::INVALIDSTRIDE),
            2 => Some(ShmError::INVALIDFD),
            _ => None
        }
    }

    fn from_i32(num: i32) -> Option<Self> {
        return Self::from_u32(num as u32);
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
    ARGB8888 = 0,
    /// 32-bit RGB format
    XRGB8888 = 1,
    C8 = 0x20203843,
    RGB332 = 0x38424752,
    BGR233 = 0x38524742,
    XRGB4444 = 0x32315258,
    XBGR4444 = 0x32314258,
    RGBX4444 = 0x32315852,
    BGRX4444 = 0x32315842,
    ARGB4444 = 0x32315241,
    ABGR4444 = 0x32314241,
    RGBA4444 = 0x32314152,
    BGRA4444 = 0x32314142,
    XRGB1555 = 0x35315258,
    XBGR1555 = 0x35314258,
    RGBX5551 = 0x35315852,
    BGRX5551 = 0x35315842,
    ARGB1555 = 0x35315241,
    ABGR1555 = 0x35314241,
    RGBA5551 = 0x35314152,
    BGRA5551 = 0x35314142,
    RGB565 = 0x36314752,
    BGR565 = 0x36314742,
    RGB888 = 0x34324752,
    BGR888 = 0x34324742,
    XBGR8888 = 0x34324258,
    RGBX8888 = 0x34325852,
    BGRX8888 = 0x34325842,
    ABGR8888 = 0x34324241,
    RGBA8888 = 0x34324152,
    BGRA8888 = 0x34324142,
    XRGB2101010 = 0x30335258,
    XBGR2101010 = 0x30334258,
    RGBX1010102 = 0x30335852,
    BGRX1010102 = 0x30335842,
    ARGB2101010 = 0x30335241,
    ABGR2101010 = 0x30334241,
    RGBA1010102 = 0x30334152,
    BGRA1010102 = 0x30334142,
    YUYV = 0x56595559,
    YVYU = 0x55595659,
    UYVY = 0x59565955,
    VYUY = 0x59555956,
    AYUV = 0x56555941,
    NV12 = 0x3231564e,
    NV21 = 0x3132564e,
    NV16 = 0x3631564e,
    NV61 = 0x3136564e,
    YUV410 = 0x39565559,
    YVU410 = 0x39555659,
    YUV411 = 0x31315559,
    YVU411 = 0x31315659,
    YUV420 = 0x32315559,
    YVU420 = 0x32315659,
    YUV422 = 0x36315559,
    YVU422 = 0x36315659,
    YUV444 = 0x34325559,
    YVU444 = 0x34325659,
}

impl FromPrimitive for ShmFormat {
    fn from_u32(num: u32) -> Option<Self> {
        return match num {
            0 => Some(ShmFormat::ARGB8888),
            1 => Some(ShmFormat::XRGB8888),
            0x20203843 => Some(ShmFormat::C8),
            0x38424752 => Some(ShmFormat::RGB332),
            0x38524742 => Some(ShmFormat::BGR233),
            0x32315258 => Some(ShmFormat::XRGB4444),
            0x32314258 => Some(ShmFormat::XBGR4444),
            0x32315852 => Some(ShmFormat::RGBX4444),
            0x32315842 => Some(ShmFormat::BGRX4444),
            0x32315241 => Some(ShmFormat::ARGB4444),
            0x32314241 => Some(ShmFormat::ABGR4444),
            0x32314152 => Some(ShmFormat::RGBA4444),
            0x32314142 => Some(ShmFormat::BGRA4444),
            0x35315258 => Some(ShmFormat::XRGB1555),
            0x35314258 => Some(ShmFormat::XBGR1555),
            0x35315852 => Some(ShmFormat::RGBX5551),
            0x35315842 => Some(ShmFormat::BGRX5551),
            0x35315241 => Some(ShmFormat::ARGB1555),
            0x35314241 => Some(ShmFormat::ABGR1555),
            0x35314152 => Some(ShmFormat::RGBA5551),
            0x35314142 => Some(ShmFormat::BGRA5551),
            0x36314752 => Some(ShmFormat::RGB565),
            0x36314742 => Some(ShmFormat::BGR565),
            0x34324752 => Some(ShmFormat::RGB888),
            0x34324742 => Some(ShmFormat::BGR888),
            0x34324258 => Some(ShmFormat::XBGR8888),
            0x34325852 => Some(ShmFormat::RGBX8888),
            0x34325842 => Some(ShmFormat::BGRX8888),
            0x34324241 => Some(ShmFormat::ABGR8888),
            0x34324152 => Some(ShmFormat::RGBA8888),
            0x34324142 => Some(ShmFormat::BGRA8888),
            0x30335258 => Some(ShmFormat::XRGB2101010),
            0x30334258 => Some(ShmFormat::XBGR2101010),
            0x30335852 => Some(ShmFormat::RGBX1010102),
            0x30335842 => Some(ShmFormat::BGRX1010102),
            0x30335241 => Some(ShmFormat::ARGB2101010),
            0x30334241 => Some(ShmFormat::ABGR2101010),
            0x30334152 => Some(ShmFormat::RGBA1010102),
            0x30334142 => Some(ShmFormat::BGRA1010102),
            0x56595559 => Some(ShmFormat::YUYV),
            0x55595659 => Some(ShmFormat::YVYU),
            0x59565955 => Some(ShmFormat::UYVY),
            0x59555956 => Some(ShmFormat::VYUY),
            0x56555941 => Some(ShmFormat::AYUV),
            0x3231564e => Some(ShmFormat::NV12),
            0x3132564e => Some(ShmFormat::NV21),
            0x3631564e => Some(ShmFormat::NV16),
            0x3136564e => Some(ShmFormat::NV61),
            0x39565559 => Some(ShmFormat::YUV410),
            0x39555659 => Some(ShmFormat::YVU410),
            0x31315559 => Some(ShmFormat::YUV411),
            0x31315659 => Some(ShmFormat::YVU411),
            0x32315559 => Some(ShmFormat::YUV420),
            0x32315659 => Some(ShmFormat::YVU420),
            0x36315559 => Some(ShmFormat::YUV422),
            0x36315659 => Some(ShmFormat::YVU422),
            0x34325559 => Some(ShmFormat::YUV444),
            0x34325659 => Some(ShmFormat::YVU444),
            _ => None
        }
    }

    fn from_i32(num: i32) -> Option<Self> {
        return Self::from_u32(num as u32);
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

    fn from_i32(num: i32) -> Option<Self> {
        return Self::from_u32(num as u32);
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

    fn from_i32(num: i32) -> Option<Self> {
        return Self::from_u32(num as u32);
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
