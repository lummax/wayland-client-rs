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
    static wl_registry_interface: ffi::wayland::WLInterface;
}

#[repr(C)]
enum RegistryEvent {
    Global = 0,
    GlobalRemove = 1,
}

impl FromPrimitive for RegistryEvent {
    fn from_u32(num: u32) -> Option<Self> {
        return match num {
            0 => Some(RegistryEvent::Global),
            1 => Some(RegistryEvent::GlobalRemove),
            _ => None
        }
    }
}

#[repr(C)]
enum RegistryRequest {
    Bind = 0,
    _Dummy,
}

impl FromPrimitive for RegistryRequest {
    fn from_u32(num: u32) -> Option<Self> {
        return match num {
            0 => Some(RegistryRequest::Bind),
            _ => None
        }
    }
}

/// The global registry object.  The server has a number of global
/// objects that are available to all clients.  These objects
/// typically represent an actual object in the server (for example,
/// an input device) or they are singleton objects that provide
/// extension functionality.
/// 
/// When a client creates a registry object, the registry object
/// will emit a global event for each global currently in the
/// registry.  Globals come and go as a result of device or
/// monitor hotplugs, reconfiguration or other events, and the
/// registry will send out global and global_remove events to
/// keep the client up to date with the changes.  To mark the end
/// of the initial burst of events, the client can use the
/// wl_display.sync request immediately after calling
/// wl_display.get_registry.
/// 
/// A client can bind to a global object by using the bind
/// request.  This creates a client-side handle that lets the object
/// emit events to the client and lets the client invoke requests on
/// the object.
#[derive(Debug)]
pub struct Registry {
    proxy: BaseProxy,
}

impl Registry {
    
    /// Binds a new, client-created object to the server using the
    /// specified name as the identifier.
    pub fn bind<T: FromRawPtr<ffi::wayland::WLProxy> + GetInterface>(&mut self, name: u32, version: u32) -> Result<T, &'static str> {
        let proxy = self.as_mut_ptr() as *mut ffi::wayland::WLProxy;
        let object = unsafe {
            ffi::wayland::wl_proxy_marshal_constructor(
                proxy, RegistryRequest::Bind as u32, T::get_interface(), name, (*T::get_interface()).name, version, ptr::null::<c_void>()
            )
        };
        return T::from_mut_ptr(object);
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


impl FromRawPtr<ffi::wayland::WLProxy> for Registry {
    fn from_mut_ptr(ptr: *mut ffi::wayland::WLProxy) -> Result<Self, &'static str> {
        return match FromRawPtr::from_mut_ptr(ptr) {
            Ok(proxy) => Ok(Registry {
                proxy: proxy,
            }),
            Err(str) => Err(str),
        }
    }
}

impl AsRawPtr<ffi::wayland::WLProxy> for Registry {
    fn as_mut_ptr(&mut self) -> *mut ffi::wayland::WLProxy {
        return self.proxy.as_mut_ptr();
    }
}

impl GetInterface for Registry {
    fn get_interface() -> *const ffi::wayland::WLInterface {
        return &wl_registry_interface as *const ffi::wayland::WLInterface;
    }
}

#[allow(unused_variables)]
extern fn registry_event_dispatcher<T: RegistryEventHandler>(
        user_data: *mut c_void,
        _target: *mut c_void,
        opcode: uint32_t,
        _message: *const ffi::wayland::WLMessage,
        arguments: *mut ffi::wayland::WLArgument) -> c_int {
    let object = user_data as *mut T;
    return match RegistryEvent::from_u32(opcode) {
        Some(event) => {
            match event {
                RegistryEvent::Global => {
                    let name = unsafe { *(*arguments.offset(0)).uint() };
                    let interface_raw = unsafe { *(*arguments.offset(1)).string() };
                    let interface_buffer = unsafe { CStr::from_ptr(interface_raw).to_bytes() };
                    let interface = String::from_utf8(interface_buffer.to_vec()).unwrap();
                    let version = unsafe { *(*arguments.offset(2)).uint() };
                    unsafe { (*object).on_global(name, interface, version); }
                },
                RegistryEvent::GlobalRemove => {
                    let name = unsafe { *(*arguments.offset(0)).uint() };
                    unsafe { (*object).on_global_remove(name); }
                },
            }
            0
        },
        _ => -1,
    }
}

pub trait RegistryEventHandler: Sized {
    fn connect_dispatcher(&mut self) {
        unsafe {
            ffi::wayland::wl_proxy_add_dispatcher(
                self.get_registry().as_mut_ptr(),
                registry_event_dispatcher::<Self>,
                self as *mut Self as *mut c_void,
                ptr::null_mut());
        }
    }

    fn get_registry(&mut self) -> &mut Registry;
    
    /// Notify the client of global objects.
    /// 
    /// The event notifies the client that a global object with
    /// the given name is now available, and it implements the
    /// given version of the given interface.
    #[allow(unused_variables)]
    fn on_global(&mut self, name: u32, interface: String, version: u32) {}
    
    /// Notify the client of removed global objects.
    /// 
    /// This event notifies the client that the global identified
    /// by name is no longer available.  If the client bound to
    /// the global using the bind request, the client should now
    /// destroy that object.
    /// 
    /// The object remains valid and requests to the object will be
    /// ignored until the client destroys it, to avoid races between
    /// the global going away and a client sending a request to it.
    #[allow(unused_variables)]
    fn on_global_remove(&mut self, name: u32) {}
}
