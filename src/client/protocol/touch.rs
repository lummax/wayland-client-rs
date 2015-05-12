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
    static wl_touch_interface: ffi::wayland::WLInterface;
}

#[repr(C)]
enum TouchEvent {
    Down = 0,
    Up = 1,
    Motion = 2,
    Frame = 3,
    Cancel = 4,
}

impl FromPrimitive for TouchEvent {
    fn from_u32(num: u32) -> Option<Self> {
        return match num {
            0 => Some(TouchEvent::Down),
            1 => Some(TouchEvent::Up),
            2 => Some(TouchEvent::Motion),
            3 => Some(TouchEvent::Frame),
            4 => Some(TouchEvent::Cancel),
            _ => None
        }
    }

    fn from_i32(num: i32) -> Option<Self> {
        return Self::from_u32(num as u32);
    }
}

#[repr(C)]
enum TouchRequest {
    Release = 0,
    _Dummy,
}

impl FromPrimitive for TouchRequest {
    fn from_u32(num: u32) -> Option<Self> {
        return match num {
            0 => Some(TouchRequest::Release),
            _ => None
        }
    }

    fn from_i32(num: i32) -> Option<Self> {
        return Self::from_u32(num as u32);
    }
}

/// The wl_touch interface represents a touchscreen
/// associated with a seat.
/// 
/// Touch interactions can consist of one or more contacts.
/// For each contact, a series of events is generated, starting
/// with a down event, followed by zero or more motion events,
/// and ending with an up event. Events relating to the same
/// contact point can be identified by the ID of the sequence.
#[derive(Debug)]
pub struct Touch {
    proxy: BaseProxy,
}

impl Touch {
    
    pub fn release(mut self) {
        let proxy = self.as_mut_ptr() as *mut ffi::wayland::WLProxy;
        unsafe {
            ffi::wayland::wl_proxy_marshal(
                proxy, TouchRequest::Release as u32
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


impl FromRawPtr<ffi::wayland::WLProxy> for Touch {
    fn from_mut_ptr(ptr: *mut ffi::wayland::WLProxy) -> Result<Self, &'static str> {
        return match FromRawPtr::from_mut_ptr(ptr) {
            Ok(proxy) => Ok(Touch {
                proxy: proxy,
            }),
            Err(str) => Err(str),
        }
    }
}

impl AsRawPtr<ffi::wayland::WLProxy> for Touch {
    fn as_mut_ptr(&mut self) -> *mut ffi::wayland::WLProxy {
        return self.proxy.as_mut_ptr();
    }
}

impl GetInterface for Touch {
    fn get_interface() -> *const ffi::wayland::WLInterface {
        return &wl_touch_interface as *const ffi::wayland::WLInterface;
    }
}

#[allow(unused_variables)]
extern fn touch_event_dispatcher<T: TouchEventHandler>(
        user_data: *mut c_void,
        _target: *mut c_void,
        opcode: uint32_t,
        _message: *const ffi::wayland::WLMessage,
        arguments: *mut ffi::wayland::WLArgument) -> c_int {
    let object = user_data as *mut T;
    return match TouchEvent::from_u32(opcode) {
        Some(event) => {
            match event {
                TouchEvent::Down => {
                    let serial = unsafe { *(*arguments.offset(0)).uint() };
                    let time = unsafe { *(*arguments.offset(1)).uint() };
                    let surface = unsafe { *(*arguments.offset(2)).object() };
                    let id = unsafe { *(*arguments.offset(3)).int() };
                    let x = unsafe { *(*arguments.offset(4)).fixed_point() };
                    let y = unsafe { *(*arguments.offset(5)).fixed_point() };
                    unsafe { (*object).on_down(serial, time, surface, id, x, y); }
                },
                TouchEvent::Up => {
                    let serial = unsafe { *(*arguments.offset(0)).uint() };
                    let time = unsafe { *(*arguments.offset(1)).uint() };
                    let id = unsafe { *(*arguments.offset(2)).int() };
                    unsafe { (*object).on_up(serial, time, id); }
                },
                TouchEvent::Motion => {
                    let time = unsafe { *(*arguments.offset(0)).uint() };
                    let id = unsafe { *(*arguments.offset(1)).int() };
                    let x = unsafe { *(*arguments.offset(2)).fixed_point() };
                    let y = unsafe { *(*arguments.offset(3)).fixed_point() };
                    unsafe { (*object).on_motion(time, id, x, y); }
                },
                TouchEvent::Frame => {
                    unsafe { (*object).on_frame(); }
                },
                TouchEvent::Cancel => {
                    unsafe { (*object).on_cancel(); }
                },
            }
            0
        },
        _ => -1,
    }
}

pub trait TouchEventHandler: Sized {
    fn connect_dispatcher(&mut self) {
        unsafe {
            ffi::wayland::wl_proxy_add_dispatcher(
                self.get_touch().as_mut_ptr(),
                touch_event_dispatcher::<Self>,
                self as *mut Self as *mut c_void,
                ptr::null_mut());
        }
    }

    fn get_touch(&mut self) -> &mut Touch;
    
    /// A new touch point has appeared on the surface. This touch point is
    /// assigned a unique @id. Future events from this touchpoint reference
    /// this ID. The ID ceases to be valid after a touch up event and may be
    /// re-used in the future.
    #[allow(unused_variables)]
    fn on_down(&mut self, serial: u32, time: u32, surface: *mut ffi::wayland::WLObject, id: i32, x: i32, y: i32) {}
    
    /// The touch point has disappeared. No further events will be sent for
    /// this touchpoint and the touch point's ID is released and may be
    /// re-used in a future touch down event.
    #[allow(unused_variables)]
    fn on_up(&mut self, serial: u32, time: u32, id: i32) {}
    
    /// A touchpoint has changed coordinates.
    #[allow(unused_variables)]
    fn on_motion(&mut self, time: u32, id: i32, x: i32, y: i32) {}
    
    /// Indicates the end of a contact point list.
    #[allow(unused_variables)]
    fn on_frame(&mut self) {}
    
    /// Sent if the compositor decides the touch stream is a global
    /// gesture. No further events are sent to the clients from that
    /// particular gesture. Touch cancellation applies to all touch points
    /// currently active on this client's surface. The client is
    /// responsible for finalizing the touch points, future touch points on
    /// this surface may re-use the touch point ID.
    #[allow(unused_variables)]
    fn on_cancel(&mut self) {}
}
