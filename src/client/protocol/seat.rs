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
    static wl_seat_interface: ffi::wayland::WLInterface;
}

/// This is a bitmask of capabilities this seat has; if a member is
/// set, then it is present on the seat.
#[repr(C)]
#[derive(Debug)]
pub enum SeatCapability {
    /// The seat has pointer devices
    Pointer = 1,
    /// The seat has one or more keyboards
    Keyboard = 2,
    /// The seat has touch devices
    Touch = 4,
}

impl FromPrimitive for SeatCapability {
    fn from_u32(num: u32) -> Option<Self> {
        return match num {
            1 => Some(SeatCapability::Pointer),
            2 => Some(SeatCapability::Keyboard),
            4 => Some(SeatCapability::Touch),
            _ => None
        }
    }

    fn from_i32(num: i32) -> Option<Self> {
        return Self::from_u32(num as u32);
    }
}

pub trait SeatCapabilitySet {
    fn has_pointer(&self) -> bool;
    fn has_keyboard(&self) -> bool;
    fn has_touch(&self) -> bool;
}

impl SeatCapabilitySet for u32 {
    fn is_pointer(&self) -> bool {
        return self & (SeatCapability::Pointer as u32) != 0;
    }
    fn is_keyboard(&self) -> bool {
        return self & (SeatCapability::Keyboard as u32) != 0;
    }
    fn is_touch(&self) -> bool {
        return self & (SeatCapability::Touch as u32) != 0;
    }
}

impl SeatCapabilitySet for i32 {
    fn is_pointer(&self) -> bool {
        return self & (SeatCapability::Pointer as i32) != 0;
    }
    fn is_keyboard(&self) -> bool {
        return self & (SeatCapability::Keyboard as i32) != 0;
    }
    fn is_touch(&self) -> bool {
        return self & (SeatCapability::Touch as i32) != 0;
    }
}


#[repr(C)]
enum SeatEvent {
    Capabilities = 0,
    Name = 1,
}

impl FromPrimitive for SeatEvent {
    fn from_u32(num: u32) -> Option<Self> {
        return match num {
            0 => Some(SeatEvent::Capabilities),
            1 => Some(SeatEvent::Name),
            _ => None
        }
    }

    fn from_i32(num: i32) -> Option<Self> {
        return Self::from_u32(num as u32);
    }
}



#[repr(C)]
enum SeatRequest {
    GetPointer = 0,
    GetKeyboard = 1,
    GetTouch = 2,
}

impl FromPrimitive for SeatRequest {
    fn from_u32(num: u32) -> Option<Self> {
        return match num {
            0 => Some(SeatRequest::GetPointer),
            1 => Some(SeatRequest::GetKeyboard),
            2 => Some(SeatRequest::GetTouch),
            _ => None
        }
    }

    fn from_i32(num: i32) -> Option<Self> {
        return Self::from_u32(num as u32);
    }
}



/// A seat is a group of keyboards, pointer and touch devices. This
/// object is published as a global during start up, or when such a
/// device is hot plugged.  A seat typically has a pointer and
/// maintains a keyboard focus and a pointer focus.
#[derive(Debug)]
pub struct Seat {
    proxy: BaseProxy,
}

impl Seat {
    
    /// The ID provided will be initialized to the wl_pointer interface
    /// for this seat.
    /// 
    /// This request only takes effect if the seat has the pointer
    /// capability.
    pub fn get_pointer(&mut self) -> Result<::client::protocol::Pointer, &'static str> {
        let proxy = self.as_mut_ptr() as *mut ffi::wayland::WLProxy;
        let object = unsafe {
            ffi::wayland::wl_proxy_marshal_constructor(
                proxy, SeatRequest::GetPointer as u32, ::client::protocol::Pointer::get_interface(), ptr::null::<c_void>()
            )
        };
        return ::client::protocol::Pointer::from_mut_ptr(object);
    }
    
    /// The ID provided will be initialized to the wl_keyboard interface
    /// for this seat.
    /// 
    /// This request only takes effect if the seat has the keyboard
    /// capability.
    pub fn get_keyboard(&mut self) -> Result<::client::protocol::Keyboard, &'static str> {
        let proxy = self.as_mut_ptr() as *mut ffi::wayland::WLProxy;
        let object = unsafe {
            ffi::wayland::wl_proxy_marshal_constructor(
                proxy, SeatRequest::GetKeyboard as u32, ::client::protocol::Keyboard::get_interface(), ptr::null::<c_void>()
            )
        };
        return ::client::protocol::Keyboard::from_mut_ptr(object);
    }
    
    /// The ID provided will be initialized to the wl_touch interface
    /// for this seat.
    /// 
    /// This request only takes effect if the seat has the touch
    /// capability.
    pub fn get_touch(&mut self) -> Result<::client::protocol::Touch, &'static str> {
        let proxy = self.as_mut_ptr() as *mut ffi::wayland::WLProxy;
        let object = unsafe {
            ffi::wayland::wl_proxy_marshal_constructor(
                proxy, SeatRequest::GetTouch as u32, ::client::protocol::Touch::get_interface(), ptr::null::<c_void>()
            )
        };
        return ::client::protocol::Touch::from_mut_ptr(object);
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


impl FromRawPtr<ffi::wayland::WLProxy> for Seat {
    fn from_mut_ptr(ptr: *mut ffi::wayland::WLProxy) -> Result<Self, &'static str> {
        return match FromRawPtr::from_mut_ptr(ptr) {
            Ok(proxy) => Ok(Seat {
                proxy: proxy,
            }),
            Err(str) => Err(str),
        }
    }
}

impl AsRawPtr<ffi::wayland::WLProxy> for Seat {
    fn as_mut_ptr(&mut self) -> *mut ffi::wayland::WLProxy {
        return self.proxy.as_mut_ptr();
    }
}

impl GetInterface for Seat {
    fn get_interface() -> *const ffi::wayland::WLInterface {
        return &wl_seat_interface as *const ffi::wayland::WLInterface;
    }
}

#[allow(unused_variables)]
extern fn seat_event_dispatcher<T: SeatEventHandler>(
        user_data: *mut c_void,
        _target: *mut c_void,
        opcode: uint32_t,
        _message: *const ffi::wayland::WLMessage,
        arguments: *mut ffi::wayland::WLArgument) -> c_int {
    let object = user_data as *mut T;
    return match SeatEvent::from_u32(opcode) {
        Some(event) => {
            match event {
                SeatEvent::Capabilities => {
                    let capabilities = unsafe { *(*arguments.offset(0)).uint() };
                    unsafe { (*object).on_capabilities(capabilities); }
                },
                SeatEvent::Name => {
                    let name_raw = unsafe { *(*arguments.offset(0)).string() };
                    let name_buffer = unsafe { CStr::from_ptr(name_raw).to_bytes() };
                    let name = String::from_utf8(name_buffer.to_vec()).unwrap();
                    unsafe { (*object).on_name(name); }
                },
            }
            0
        },
        _ => -1,
    }
}

pub trait SeatEventHandler: Sized {
    fn connect_dispatcher(&mut self) {
        unsafe {
            ffi::wayland::wl_proxy_add_dispatcher(
                self.get_seat().as_mut_ptr(),
                seat_event_dispatcher::<Self>,
                self as *mut Self as *mut c_void,
                ptr::null_mut());
        }
    }

    fn get_seat(&mut self) -> &mut Seat;
    
    /// This is emitted whenever a seat gains or loses the pointer,
    /// keyboard or touch capabilities.  The argument is a capability
    /// enum containing the complete set of capabilities this seat has.
    #[allow(unused_variables)]
    fn on_capabilities(&mut self, capabilities: u32) {}
    
    /// In a multiseat configuration this can be used by the client to help
    /// identify which physical devices the seat represents. Based on
    /// the seat configuration used by the compositor.
    #[allow(unused_variables)]
    fn on_name(&mut self, name: String) {}
}
