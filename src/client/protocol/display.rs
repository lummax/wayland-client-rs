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
use client::base::Display as BaseDisplay;
use client::base::{FromRawPtr, AsRawPtr, EventQueue};

#[link(name="wayland-client")]
extern {
    static wl_display_interface: ffi::wayland::WLInterface;
}

/// These errors are global and can be emitted in response to any
/// server request.
#[repr(C)]
#[derive(Debug)]
pub enum DisplayError {
    /// server couldn't find object
    InvalidObject = 0,
    /// method doesn't exist on the specified interface
    InvalidMethod = 1,
    /// server is out of memory
    NoMemory = 2,
}

impl FromPrimitive for DisplayError {
    fn from_u32(num: u32) -> Option<Self> {
        return match num {
            0 => Some(DisplayError::InvalidObject),
            1 => Some(DisplayError::InvalidMethod),
            2 => Some(DisplayError::NoMemory),
            _ => None
        }
    }
}

#[repr(C)]
enum DisplayEvent {
    Error = 0,
    DeleteId = 1,
}

impl FromPrimitive for DisplayEvent {
    fn from_u32(num: u32) -> Option<Self> {
        return match num {
            0 => Some(DisplayEvent::Error),
            1 => Some(DisplayEvent::DeleteId),
            _ => None
        }
    }
}

#[repr(C)]
enum DisplayRequest {
    Sync = 0,
    GetRegistry = 1,
}

impl FromPrimitive for DisplayRequest {
    fn from_u32(num: u32) -> Option<Self> {
        return match num {
            0 => Some(DisplayRequest::Sync),
            1 => Some(DisplayRequest::GetRegistry),
            _ => None
        }
    }
}

/// The core global object.  This is a special singleton object.  It
/// is used for internal Wayland protocol features.
#[derive(Debug)]
pub struct Display {
    display: BaseDisplay,
}

impl Display {
    
    /// The sync request asks the server to emit the 'done' event
    /// on the returned wl_callback object.  Since requests are
    /// handled in-order and events are delivered in-order, this can
    /// be used as a barrier to ensure all previous requests and the
    /// resulting events have been handled.
    /// 
    /// The object returned by this request will be destroyed by the
    /// compositor after the callback is fired and as such the client must not
    /// attempt to use it after that point.
    /// 
    /// The callback_data passed in the callback is the event serial.
    pub fn sync(&mut self) -> Result<::client::protocol::Callback, &'static str> {
        let proxy = self.as_mut_ptr() as *mut ffi::wayland::WLProxy;
        let object = unsafe {
            ffi::wayland::wl_proxy_marshal_constructor(
                proxy, DisplayRequest::Sync as u32, ::client::protocol::Callback::get_interface(), ptr::null::<c_void>()
            )
        };
        return ::client::protocol::Callback::from_mut_ptr(object);
    }
    
    /// This request creates a registry object that allows the client
    /// to list and bind the global objects available from the
    /// compositor.
    pub fn get_registry(&mut self) -> Result<::client::protocol::Registry, &'static str> {
        let proxy = self.as_mut_ptr() as *mut ffi::wayland::WLProxy;
        let object = unsafe {
            ffi::wayland::wl_proxy_marshal_constructor(
                proxy, DisplayRequest::GetRegistry as u32, ::client::protocol::Registry::get_interface(), ptr::null::<c_void>()
            )
        };
        return ::client::protocol::Registry::from_mut_ptr(object);
    }

    pub fn connect(name: Option<&str>) -> Result<Self, &'static str> {
        return match BaseDisplay::connect(name) {
            Ok(display) => Ok(Display{
                display: display,
            }),
            Err(err) => Err(err),
        }
    }

    pub fn connect_to_fd(fd: RawFd) -> Result<Self, &'static str> {
        return match BaseDisplay::connect_to_fd(fd) {
            Ok(display) => Ok(Display{
                display: display,
            }),
            Err(err) => Err(err),
        }
    }

    pub fn get_id(&mut self) -> u32 {
        return self.display.get_id();
    }

    pub fn get_class(&mut self) -> String {
        return self.display.get_class();
    }

    pub fn set_queue(&mut self, queue: &mut EventQueue) {
        self.display.set_queue(queue);
    }

    pub fn get_fd(&mut self) -> RawFd {
        return self.display.get_fd();
    }

    pub fn dispatch(&mut self) -> Option<u32> {
        return self.display.dispatch();
    }

    pub fn dispatch_queue(&mut self, queue: &mut EventQueue) -> Option<u32> {
        return self.display.dispatch_queue(queue);
    }

    pub fn dispatch_queue_pending(&mut self, queue: &mut EventQueue) -> Option<u32> {
        return self.display.dispatch_queue_pending(queue);
    }

    pub fn dispatch_pending(&mut self) -> Option<u32> {
        return self.display.dispatch_pending();
    }

    pub fn get_error(&mut self) -> i32 {
        return self.display.get_error();
    }

    pub fn flush(&mut self) -> Option<u32> {
        return self.display.flush();
    }

    pub fn roundtrip_queue(&mut self, queue: &mut EventQueue) -> Option<u32> {
        return self.display.roundtrip_queue(queue);
    }

    pub fn roundtrip(&mut self) -> Option<u32> {
        return self.display.roundtrip();
    }

    pub fn create_queue(&mut self) -> Result<EventQueue, &'static str> {
        return self.display.create_queue();
    }

    pub fn prepare_read_queue(&mut self, queue: &mut EventQueue) -> bool {
        return self.display.prepare_read_queue(queue);
    }

    pub fn prepare_read(&mut self) -> bool {
        return self.display.prepare_read();
    }

    pub fn cancel_read(&mut self) {
        self.display.cancel_read();
    }

    pub fn read_events(&mut self) -> bool {
        return self.display.read_events();
    }
}

impl AsRawPtr<ffi::wayland::WLDisplay> for Display {
    fn as_mut_ptr(&mut self) -> *mut ffi::wayland::WLDisplay {
        return self.display.as_mut_ptr();
    }
}


impl GetInterface for Display {
    fn get_interface() -> *const ffi::wayland::WLInterface {
        return &wl_display_interface as *const ffi::wayland::WLInterface;
    }
}

#[allow(unused_variables)]
extern fn display_event_dispatcher<T: DisplayEventHandler>(
        user_data: *mut c_void,
        _target: *mut c_void,
        opcode: uint32_t,
        _message: *const ffi::wayland::WLMessage,
        arguments: *mut ffi::wayland::WLArgument) -> c_int {
    let object = user_data as *mut T;
    return match DisplayEvent::from_u32(opcode) {
        Some(event) => {
            match event {
                DisplayEvent::Error => {
                    let object_id = unsafe { *(*arguments.offset(0)).object() };
                    let code = unsafe { *(*arguments.offset(1)).uint() };
                    let message_raw = unsafe { *(*arguments.offset(2)).string() };
                    let message_buffer = unsafe { CStr::from_ptr(message_raw).to_bytes() };
                    let message = String::from_utf8(message_buffer.to_vec()).unwrap();
                    unsafe { (*object).on_error(object_id, code, message); }
                },
                DisplayEvent::DeleteId => {
                    let id = unsafe { *(*arguments.offset(0)).uint() };
                    unsafe { (*object).on_delete_id(id); }
                },
            }
            0
        },
        _ => -1,
    }
}

pub trait DisplayEventHandler: Sized {
    fn connect_dispatcher(&mut self) {
        unsafe {
            ffi::wayland::wl_proxy_add_dispatcher(
                self.get_display().as_mut_ptr() as *mut ffi::wayland::WLProxy,
                display_event_dispatcher::<Self>,
                self as *mut Self as *mut c_void,
                ptr::null_mut());
        }
    }

    fn get_display(&mut self) -> &mut Display;
    
    /// The error event is sent out when a fatal (non-recoverable)
    /// error has occurred.  The object_id argument is the object
    /// where the error occurred, most often in response to a request
    /// to that object.  The code identifies the error and is defined
    /// by the object interface.  As such, each interface defines its
    /// own set of error codes.  The message is an brief description
    /// of the error, for (debugging) convenience.
    #[allow(unused_variables)]
    fn on_error(&mut self, object_id: *mut ffi::wayland::WLObject, code: u32, message: String) {}
    
    /// This event is used internally by the object ID management
    /// logic.  When a client deletes an object, the server will send
    /// this event to acknowledge that it has seen the delete request.
    /// When the client receive this event, it will know that it can
    /// safely reuse the object ID.
    #[allow(unused_variables)]
    fn on_delete_id(&mut self, id: u32) {}
}
