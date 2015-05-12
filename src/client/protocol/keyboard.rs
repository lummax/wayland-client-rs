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
    static wl_keyboard_interface: ffi::wayland::WLInterface;
}

/// This specifies the format of the keymap provided to the
/// client with the wl_keyboard.keymap event.
#[repr(C)]
#[derive(Debug)]
pub enum KeyboardKeymapFormat {
    /// no keymap; client must understand how to interpret the raw keycode
    NOKEYMAP = 0,
    /// libxkbcommon compatible; to determine the xkb keycode, clients must add 8 to the key event keycode
    XKBV1 = 1,
}

impl FromPrimitive for KeyboardKeymapFormat {
    fn from_u32(num: u32) -> Option<Self> {
        return match num {
            0 => Some(KeyboardKeymapFormat::NOKEYMAP),
            1 => Some(KeyboardKeymapFormat::XKBV1),
            _ => None
        }
    }
}

/// Describes the physical state of a key which provoked the key event.
#[repr(C)]
#[derive(Debug)]
pub enum KeyboardKeyState {
    /// key is not pressed
    RELEASED = 0,
    /// key is pressed
    PRESSED = 1,
}

impl FromPrimitive for KeyboardKeyState {
    fn from_u32(num: u32) -> Option<Self> {
        return match num {
            0 => Some(KeyboardKeyState::RELEASED),
            1 => Some(KeyboardKeyState::PRESSED),
            _ => None
        }
    }
}

#[repr(C)]
enum KeyboardEvent {
    Keymap = 0,
    Enter = 1,
    Leave = 2,
    Key = 3,
    Modifiers = 4,
    RepeatInfo = 5,
}

impl FromPrimitive for KeyboardEvent {
    fn from_u32(num: u32) -> Option<Self> {
        return match num {
            0 => Some(KeyboardEvent::Keymap),
            1 => Some(KeyboardEvent::Enter),
            2 => Some(KeyboardEvent::Leave),
            3 => Some(KeyboardEvent::Key),
            4 => Some(KeyboardEvent::Modifiers),
            5 => Some(KeyboardEvent::RepeatInfo),
            _ => None
        }
    }
}

#[repr(C)]
enum KeyboardRequest {
    Release = 0,
    _Dummy,
}

impl FromPrimitive for KeyboardRequest {
    fn from_u32(num: u32) -> Option<Self> {
        return match num {
            0 => Some(KeyboardRequest::Release),
            _ => None
        }
    }
}

/// The wl_keyboard interface represents one or more keyboards
/// associated with a seat.
#[derive(Debug)]
pub struct Keyboard {
    proxy: BaseProxy,
}

impl Keyboard {
    
    pub fn release(mut self) {
        let proxy = self.as_mut_ptr() as *mut ffi::wayland::WLProxy;
        unsafe {
            ffi::wayland::wl_proxy_marshal(
                proxy, KeyboardRequest::Release as u32
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


impl FromRawPtr<ffi::wayland::WLProxy> for Keyboard {
    fn from_mut_ptr(ptr: *mut ffi::wayland::WLProxy) -> Result<Self, &'static str> {
        return match FromRawPtr::from_mut_ptr(ptr) {
            Ok(proxy) => Ok(Keyboard {
                proxy: proxy,
            }),
            Err(str) => Err(str),
        }
    }
}

impl AsRawPtr<ffi::wayland::WLProxy> for Keyboard {
    fn as_mut_ptr(&mut self) -> *mut ffi::wayland::WLProxy {
        return self.proxy.as_mut_ptr();
    }
}

impl GetInterface for Keyboard {
    fn get_interface() -> *const ffi::wayland::WLInterface {
        return &wl_keyboard_interface as *const ffi::wayland::WLInterface;
    }
}

#[allow(unused_variables)]
extern fn keyboard_event_dispatcher<T: KeyboardEventHandler>(
        user_data: *mut c_void,
        _target: *mut c_void,
        opcode: uint32_t,
        _message: *const ffi::wayland::WLMessage,
        arguments: *mut ffi::wayland::WLArgument) -> c_int {
    let object = user_data as *mut T;
    return match KeyboardEvent::from_u32(opcode) {
        Some(event) => {
            match event {
                KeyboardEvent::Keymap => {
                    let format = unsafe { *(*arguments.offset(0)).uint() };
                    let fd = unsafe { *(*arguments.offset(1)).file_descriptor() };
                    let size = unsafe { *(*arguments.offset(2)).uint() };
                    unsafe { (*object).on_keymap(format, fd, size); }
                },
                KeyboardEvent::Enter => {
                    let serial = unsafe { *(*arguments.offset(0)).uint() };
                    let surface = unsafe { *(*arguments.offset(1)).object() };
                    let keys = unsafe { *(*arguments.offset(2)).array() };
                    unsafe { (*object).on_enter(serial, surface, keys); }
                },
                KeyboardEvent::Leave => {
                    let serial = unsafe { *(*arguments.offset(0)).uint() };
                    let surface = unsafe { *(*arguments.offset(1)).object() };
                    unsafe { (*object).on_leave(serial, surface); }
                },
                KeyboardEvent::Key => {
                    let serial = unsafe { *(*arguments.offset(0)).uint() };
                    let time = unsafe { *(*arguments.offset(1)).uint() };
                    let key = unsafe { *(*arguments.offset(2)).uint() };
                    let state = unsafe { *(*arguments.offset(3)).uint() };
                    unsafe { (*object).on_key(serial, time, key, state); }
                },
                KeyboardEvent::Modifiers => {
                    let serial = unsafe { *(*arguments.offset(0)).uint() };
                    let mods_depressed = unsafe { *(*arguments.offset(1)).uint() };
                    let mods_latched = unsafe { *(*arguments.offset(2)).uint() };
                    let mods_locked = unsafe { *(*arguments.offset(3)).uint() };
                    let group = unsafe { *(*arguments.offset(4)).uint() };
                    unsafe { (*object).on_modifiers(serial, mods_depressed, mods_latched, mods_locked, group); }
                },
                KeyboardEvent::RepeatInfo => {
                    let rate = unsafe { *(*arguments.offset(0)).int() };
                    let delay = unsafe { *(*arguments.offset(1)).int() };
                    unsafe { (*object).on_repeat_info(rate, delay); }
                },
            }
            0
        },
        _ => -1,
    }
}

pub trait KeyboardEventHandler: Sized {
    fn connect_dispatcher(&mut self) {
        unsafe {
            ffi::wayland::wl_proxy_add_dispatcher(
                self.get_keyboard().as_mut_ptr(),
                keyboard_event_dispatcher::<Self>,
                self as *mut Self as *mut c_void,
                ptr::null_mut());
        }
    }

    fn get_keyboard(&mut self) -> &mut Keyboard;
    
    /// This event provides a file descriptor to the client which can be
    /// memory-mapped to provide a keyboard mapping description.
    #[allow(unused_variables)]
    fn on_keymap(&mut self, format: u32, fd: RawFd, size: u32) {}
    
    /// Notification that this seat's keyboard focus is on a certain
    /// surface.
    #[allow(unused_variables)]
    fn on_enter(&mut self, serial: u32, surface: *mut ffi::wayland::WLObject, keys: *mut ffi::wayland::WLArray) {}
    
    /// Notification that this seat's keyboard focus is no longer on
    /// a certain surface.
    /// 
    /// The leave notification is sent before the enter notification
    /// for the new focus.
    #[allow(unused_variables)]
    fn on_leave(&mut self, serial: u32, surface: *mut ffi::wayland::WLObject) {}
    
    /// A key was pressed or released.
    /// The time argument is a timestamp with millisecond
    /// granularity, with an undefined base.
    #[allow(unused_variables)]
    fn on_key(&mut self, serial: u32, time: u32, key: u32, state: u32) {}
    
    /// Notifies clients that the modifier and/or group state has
    /// changed, and it should update its local state.
    #[allow(unused_variables)]
    fn on_modifiers(&mut self, serial: u32, mods_depressed: u32, mods_latched: u32, mods_locked: u32, group: u32) {}
    
    /// Informs the client about the keyboard's repeat rate and delay.
    /// 
    /// This event is sent as soon as the wl_keyboard object has been created,
    /// and is guaranteed to be received by the client before any key press
    /// event.
    /// 
    /// Negative values for either rate or delay are illegal. A rate of zero
    /// will disable any repeating (regardless of the value of delay).
    /// 
    /// This event can be sent later on as well with a new value if necessary,
    /// so clients should continue listening for the event past the creation
    /// of wl_keyboard.
    #[allow(unused_variables)]
    fn on_repeat_info(&mut self, rate: i32, delay: i32) {}
}
