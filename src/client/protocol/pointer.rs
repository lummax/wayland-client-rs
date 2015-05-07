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
    static wl_pointer_interface: ffi::wayland::WLInterface;
}

#[repr(C)]
#[derive(Debug)]
pub enum PointerError {
    /// given wl_surface has another role
    Role = 0,
    
    _Dummy,
}

impl FromPrimitive for PointerError {
    fn from_u32(num: u32) -> Option<Self> {
        return match num {
            0 => Some(PointerError::Role),
            _ => None
        }
    }
}

/// Describes the physical state of a button which provoked the button
/// event.
#[repr(C)]
#[derive(Debug)]
pub enum PointerButtonState {
    /// The button is not pressed
    Released = 0,
    /// The button is pressed
    Pressed = 1,
}

impl FromPrimitive for PointerButtonState {
    fn from_u32(num: u32) -> Option<Self> {
        return match num {
            0 => Some(PointerButtonState::Released),
            1 => Some(PointerButtonState::Pressed),
            _ => None
        }
    }
}

/// Describes the axis types of scroll events.
#[repr(C)]
#[derive(Debug)]
pub enum PointerAxis {
    VerticalScroll = 0,
    HorizontalScroll = 1,
}

impl FromPrimitive for PointerAxis {
    fn from_u32(num: u32) -> Option<Self> {
        return match num {
            0 => Some(PointerAxis::VerticalScroll),
            1 => Some(PointerAxis::HorizontalScroll),
            _ => None
        }
    }
}

#[repr(C)]
enum PointerEvent {
    Enter = 0,
    Leave = 1,
    Motion = 2,
    Button = 3,
    Axis = 4,
}

impl FromPrimitive for PointerEvent {
    fn from_u32(num: u32) -> Option<Self> {
        return match num {
            0 => Some(PointerEvent::Enter),
            1 => Some(PointerEvent::Leave),
            2 => Some(PointerEvent::Motion),
            3 => Some(PointerEvent::Button),
            4 => Some(PointerEvent::Axis),
            _ => None
        }
    }
}

#[repr(C)]
enum PointerRequest {
    SetCursor = 0,
    Release = 1,
}

impl FromPrimitive for PointerRequest {
    fn from_u32(num: u32) -> Option<Self> {
        return match num {
            0 => Some(PointerRequest::SetCursor),
            1 => Some(PointerRequest::Release),
            _ => None
        }
    }
}

/// The wl_pointer interface represents one or more input devices,
/// such as mice, which control the pointer location and pointer_focus
/// of a seat.
/// 
/// The wl_pointer interface generates motion, enter and leave
/// events for the surfaces that the pointer is located over,
/// and button and axis events for button presses, button releases
/// and scrolling.
#[derive(Debug)]
pub struct Pointer {
    proxy: BaseProxy,
}

impl Pointer {
    
    /// Set the pointer surface, i.e., the surface that contains the
    /// pointer image (cursor). This request gives the surface the role
    /// of a cursor. If the surface already has another role, it raises
    /// a protocol error.
    /// 
    /// The cursor actually changes only if the pointer
    /// focus for this device is one of the requesting client's surfaces
    /// or the surface parameter is the current pointer surface. If
    /// there was a previous surface set with this request it is
    /// replaced. If surface is NULL, the pointer image is hidden.
    /// 
    /// The parameters hotspot_x and hotspot_y define the position of
    /// the pointer surface relative to the pointer location. Its
    /// top-left corner is always at (x, y) - (hotspot_x, hotspot_y),
    /// where (x, y) are the coordinates of the pointer location, in surface
    /// local coordinates.
    /// 
    /// On surface.attach requests to the pointer surface, hotspot_x
    /// and hotspot_y are decremented by the x and y parameters
    /// passed to the request. Attach must be confirmed by
    /// wl_surface.commit as usual.
    /// 
    /// The hotspot can also be updated by passing the currently set
    /// pointer surface to this request with new values for hotspot_x
    /// and hotspot_y.
    /// 
    /// The current and pending input regions of the wl_surface are
    /// cleared, and wl_surface.set_input_region is ignored until the
    /// wl_surface is no longer used as the cursor. When the use as a
    /// cursor ends, the current and pending input regions become
    /// undefined, and the wl_surface is unmapped.
    pub fn set_cursor(&mut self, serial: u32, surface: &mut ::client::protocol::Surface, hotspot_x: i32, hotspot_y: i32) {
        let proxy = self.as_mut_ptr() as *mut ffi::wayland::WLProxy;
        let surfacepointer = surface.as_mut_ptr() as *mut ffi::wayland::WLProxy;
        unsafe {
            ffi::wayland::wl_proxy_marshal(
                proxy, PointerRequest::SetCursor as u32, serial, surfacepointer, hotspot_x, hotspot_y
            );
        }
    }
    
    pub fn release(mut self) {
        let proxy = self.as_mut_ptr() as *mut ffi::wayland::WLProxy;
        unsafe {
            ffi::wayland::wl_proxy_marshal(
                proxy, PointerRequest::Release as u32
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


impl FromRawPtr<ffi::wayland::WLProxy> for Pointer {
    fn from_mut_ptr(ptr: *mut ffi::wayland::WLProxy) -> Result<Self, &'static str> {
        return match FromRawPtr::from_mut_ptr(ptr) {
            Ok(proxy) => Ok(Pointer {
                proxy: proxy,
            }),
            Err(str) => Err(str),
        }
    }
}

impl AsRawPtr<ffi::wayland::WLProxy> for Pointer {
    fn as_mut_ptr(&mut self) -> *mut ffi::wayland::WLProxy {
        return self.proxy.as_mut_ptr();
    }
}

impl GetInterface for Pointer {
    fn get_interface() -> *const ffi::wayland::WLInterface {
        return &wl_pointer_interface as *const ffi::wayland::WLInterface;
    }
}

#[allow(unused_variables)]
extern fn pointer_event_dispatcher<T: PointerEventHandler>(
        user_data: *mut c_void,
        _target: *mut c_void,
        opcode: uint32_t,
        _message: *const ffi::wayland::WLMessage,
        arguments: *mut ffi::wayland::WLArgument) -> c_int {
    let object = user_data as *mut T;
    return match PointerEvent::from_u32(opcode) {
        Some(event) => {
            match event {
                PointerEvent::Enter => {
                    let serial = unsafe { *(*arguments.offset(0)).uint() };
                    let surface = unsafe { *(*arguments.offset(1)).object() };
                    let surface_x = unsafe { *(*arguments.offset(2)).fixed_point() };
                    let surface_y = unsafe { *(*arguments.offset(3)).fixed_point() };
                    unsafe { (*object).on_enter(serial, surface, surface_x, surface_y); }
                },
                PointerEvent::Leave => {
                    let serial = unsafe { *(*arguments.offset(0)).uint() };
                    let surface = unsafe { *(*arguments.offset(1)).object() };
                    unsafe { (*object).on_leave(serial, surface); }
                },
                PointerEvent::Motion => {
                    let time = unsafe { *(*arguments.offset(0)).uint() };
                    let surface_x = unsafe { *(*arguments.offset(1)).fixed_point() };
                    let surface_y = unsafe { *(*arguments.offset(2)).fixed_point() };
                    unsafe { (*object).on_motion(time, surface_x, surface_y); }
                },
                PointerEvent::Button => {
                    let serial = unsafe { *(*arguments.offset(0)).uint() };
                    let time = unsafe { *(*arguments.offset(1)).uint() };
                    let button = unsafe { *(*arguments.offset(2)).uint() };
                    let state = unsafe { *(*arguments.offset(3)).uint() };
                    unsafe { (*object).on_button(serial, time, button, state); }
                },
                PointerEvent::Axis => {
                    let time = unsafe { *(*arguments.offset(0)).uint() };
                    let axis = unsafe { *(*arguments.offset(1)).uint() };
                    let value = unsafe { *(*arguments.offset(2)).fixed_point() };
                    unsafe { (*object).on_axis(time, axis, value); }
                },
            }
            0
        },
        _ => -1,
    }
}

pub trait PointerEventHandler: Sized {
    fn connect_dispatcher(&mut self) {
        unsafe {
            ffi::wayland::wl_proxy_add_dispatcher(
                self.get_pointer().as_mut_ptr(),
                pointer_event_dispatcher::<Self>,
                self as *mut Self as *mut c_void,
                ptr::null_mut());
        }
    }

    fn get_pointer(&mut self) -> &mut Pointer;
    
    /// Notification that this seat's pointer is focused on a certain
    /// surface.
    /// 
    /// When an seat's focus enters a surface, the pointer image
    /// is undefined and a client should respond to this event by setting
    /// an appropriate pointer image with the set_cursor request.
    #[allow(unused_variables)]
    fn on_enter(&mut self, serial: u32, surface: *mut ffi::wayland::WLObject, surface_x: i32, surface_y: i32) {}
    
    /// Notification that this seat's pointer is no longer focused on
    /// a certain surface.
    /// 
    /// The leave notification is sent before the enter notification
    /// for the new focus.
    #[allow(unused_variables)]
    fn on_leave(&mut self, serial: u32, surface: *mut ffi::wayland::WLObject) {}
    
    /// Notification of pointer location change. The arguments
    /// surface_x and surface_y are the location relative to the
    /// focused surface.
    #[allow(unused_variables)]
    fn on_motion(&mut self, time: u32, surface_x: i32, surface_y: i32) {}
    
    /// Mouse button click and release notifications.
    /// 
    /// The location of the click is given by the last motion or
    /// enter event.
    /// The time argument is a timestamp with millisecond
    /// granularity, with an undefined base.
    #[allow(unused_variables)]
    fn on_button(&mut self, serial: u32, time: u32, button: u32, state: u32) {}
    
    /// Scroll and other axis notifications.
    /// 
    /// For scroll events (vertical and horizontal scroll axes), the
    /// value parameter is the length of a vector along the specified
    /// axis in a coordinate space identical to those of motion events,
    /// representing a relative movement along the specified axis.
    /// 
    /// For devices that support movements non-parallel to axes multiple
    /// axis events will be emitted.
    /// 
    /// When applicable, for example for touch pads, the server can
    /// choose to emit scroll events where the motion vector is
    /// equivalent to a motion event vector.
    /// 
    /// When applicable, clients can transform its view relative to the
    /// scroll distance.
    #[allow(unused_variables)]
    fn on_axis(&mut self, time: u32, axis: u32, value: i32) {}
}
