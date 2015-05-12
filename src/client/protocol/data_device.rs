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
    static wl_data_device_interface: ffi::wayland::WLInterface;
}

#[repr(C)]
#[derive(Debug)]
pub enum DataDeviceError {
    /// given wl_surface has another role
    ROLE = 0,
    
    _Dummy,
}

impl FromPrimitive for DataDeviceError {
    fn from_u32(num: u32) -> Option<Self> {
        return match num {
            0 => Some(DataDeviceError::ROLE),
            _ => None
        }
    }

    fn from_i32(num: i32) -> Option<Self> {
        return Self::from_u32(num as u32);
    }
}

#[repr(C)]
enum DataDeviceEvent {
    DataOffer = 0,
    Enter = 1,
    Leave = 2,
    Motion = 3,
    Drop = 4,
    Selection = 5,
}

impl FromPrimitive for DataDeviceEvent {
    fn from_u32(num: u32) -> Option<Self> {
        return match num {
            0 => Some(DataDeviceEvent::DataOffer),
            1 => Some(DataDeviceEvent::Enter),
            2 => Some(DataDeviceEvent::Leave),
            3 => Some(DataDeviceEvent::Motion),
            4 => Some(DataDeviceEvent::Drop),
            5 => Some(DataDeviceEvent::Selection),
            _ => None
        }
    }

    fn from_i32(num: i32) -> Option<Self> {
        return Self::from_u32(num as u32);
    }
}

#[repr(C)]
enum DataDeviceRequest {
    StartDrag = 0,
    SetSelection = 1,
    Release = 2,
}

impl FromPrimitive for DataDeviceRequest {
    fn from_u32(num: u32) -> Option<Self> {
        return match num {
            0 => Some(DataDeviceRequest::StartDrag),
            1 => Some(DataDeviceRequest::SetSelection),
            2 => Some(DataDeviceRequest::Release),
            _ => None
        }
    }

    fn from_i32(num: i32) -> Option<Self> {
        return Self::from_u32(num as u32);
    }
}

/// There is one wl_data_device per seat which can be obtained
/// from the global wl_data_device_manager singleton.
/// 
/// A wl_data_device provides access to inter-client data transfer
/// mechanisms such as copy-and-paste and drag-and-drop.
#[derive(Debug)]
pub struct DataDevice {
    proxy: BaseProxy,
}

impl DataDevice {
    
    /// This request asks the compositor to start a drag-and-drop
    /// operation on behalf of the client.
    /// 
    /// The source argument is the data source that provides the data
    /// for the eventual data transfer. If source is NULL, enter, leave
    /// and motion events are sent only to the client that initiated the
    /// drag and the client is expected to handle the data passing
    /// internally.
    /// 
    /// The origin surface is the surface where the drag originates and
    /// the client must have an active implicit grab that matches the
    /// serial.
    /// 
    /// The icon surface is an optional (can be NULL) surface that
    /// provides an icon to be moved around with the cursor.  Initially,
    /// the top-left corner of the icon surface is placed at the cursor
    /// hotspot, but subsequent wl_surface.attach request can move the
    /// relative position. Attach requests must be confirmed with
    /// wl_surface.commit as usual. The icon surface is given the role of
    /// a drag-and-drop icon. If the icon surface already has another role,
    /// it raises a protocol error.
    /// 
    /// The current and pending input regions of the icon wl_surface are
    /// cleared, and wl_surface.set_input_region is ignored until the
    /// wl_surface is no longer used as the icon surface. When the use
    /// as an icon ends, the current and pending input regions become
    /// undefined, and the wl_surface is unmapped.
    pub fn start_drag(&mut self, source: Option<&mut ::client::protocol::DataSource>, origin: &mut ::client::protocol::Surface, icon: Option<&mut ::client::protocol::Surface>, serial: u32) {
        let proxy = self.as_mut_ptr() as *mut ffi::wayland::WLProxy;
        let sourcepointer = source.map(|o| o.as_mut_ptr() as *mut ffi::wayland::WLProxy).unwrap_or(ptr::null_mut());
        let originpointer = origin.as_mut_ptr() as *mut ffi::wayland::WLProxy;
        let iconpointer = icon.map(|o| o.as_mut_ptr() as *mut ffi::wayland::WLProxy).unwrap_or(ptr::null_mut());
        unsafe {
            ffi::wayland::wl_proxy_marshal(
                proxy, DataDeviceRequest::StartDrag as u32, sourcepointer, originpointer, iconpointer, serial
            );
        }
    }
    
    /// This request asks the compositor to set the selection
    /// to the data from the source on behalf of the client.
    /// 
    /// To unset the selection, set the source to NULL.
    pub fn set_selection(&mut self, source: Option<&mut ::client::protocol::DataSource>, serial: u32) {
        let proxy = self.as_mut_ptr() as *mut ffi::wayland::WLProxy;
        let sourcepointer = source.map(|o| o.as_mut_ptr() as *mut ffi::wayland::WLProxy).unwrap_or(ptr::null_mut());
        unsafe {
            ffi::wayland::wl_proxy_marshal(
                proxy, DataDeviceRequest::SetSelection as u32, sourcepointer, serial
            );
        }
    }
    
    /// This request destroys the data device.
    pub fn release(mut self) {
        let proxy = self.as_mut_ptr() as *mut ffi::wayland::WLProxy;
        unsafe {
            ffi::wayland::wl_proxy_marshal(
                proxy, DataDeviceRequest::Release as u32
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


impl FromRawPtr<ffi::wayland::WLProxy> for DataDevice {
    fn from_mut_ptr(ptr: *mut ffi::wayland::WLProxy) -> Result<Self, &'static str> {
        return match FromRawPtr::from_mut_ptr(ptr) {
            Ok(proxy) => Ok(DataDevice {
                proxy: proxy,
            }),
            Err(str) => Err(str),
        }
    }
}

impl AsRawPtr<ffi::wayland::WLProxy> for DataDevice {
    fn as_mut_ptr(&mut self) -> *mut ffi::wayland::WLProxy {
        return self.proxy.as_mut_ptr();
    }
}

impl GetInterface for DataDevice {
    fn get_interface() -> *const ffi::wayland::WLInterface {
        return &wl_data_device_interface as *const ffi::wayland::WLInterface;
    }
}

#[allow(unused_variables)]
extern fn data_device_event_dispatcher<T: DataDeviceEventHandler>(
        user_data: *mut c_void,
        _target: *mut c_void,
        opcode: uint32_t,
        _message: *const ffi::wayland::WLMessage,
        arguments: *mut ffi::wayland::WLArgument) -> c_int {
    let object = user_data as *mut T;
    return match DataDeviceEvent::from_u32(opcode) {
        Some(event) => {
            match event {
                DataDeviceEvent::DataOffer => {
                    let id_ptr = unsafe { *(*arguments.offset(0)).new_id() };
                    let id = FromRawPtr::from_mut_ptr(id_ptr).unwrap();
                    unsafe { (*object).on_data_offer(id); }
                },
                DataDeviceEvent::Enter => {
                    let serial = unsafe { *(*arguments.offset(0)).uint() };
                    let surface = unsafe { *(*arguments.offset(1)).object() };
                    let x = unsafe { *(*arguments.offset(2)).fixed_point() };
                    let y = unsafe { *(*arguments.offset(3)).fixed_point() };
                    let id = unsafe { *(*arguments.offset(4)).object() };
                    unsafe { (*object).on_enter(serial, surface, x, y, id); }
                },
                DataDeviceEvent::Leave => {
                    unsafe { (*object).on_leave(); }
                },
                DataDeviceEvent::Motion => {
                    let time = unsafe { *(*arguments.offset(0)).uint() };
                    let x = unsafe { *(*arguments.offset(1)).fixed_point() };
                    let y = unsafe { *(*arguments.offset(2)).fixed_point() };
                    unsafe { (*object).on_motion(time, x, y); }
                },
                DataDeviceEvent::Drop => {
                    unsafe { (*object).on_drop(); }
                },
                DataDeviceEvent::Selection => {
                    let id = unsafe { *(*arguments.offset(0)).object() };
                    unsafe { (*object).on_selection(id); }
                },
            }
            0
        },
        _ => -1,
    }
}

pub trait DataDeviceEventHandler: Sized {
    fn connect_dispatcher(&mut self) {
        unsafe {
            ffi::wayland::wl_proxy_add_dispatcher(
                self.get_data_device().as_mut_ptr(),
                data_device_event_dispatcher::<Self>,
                self as *mut Self as *mut c_void,
                ptr::null_mut());
        }
    }

    fn get_data_device(&mut self) -> &mut DataDevice;
    
    /// The data_offer event introduces a new wl_data_offer object,
    /// which will subsequently be used in either the
    /// data_device.enter event (for drag-and-drop) or the
    /// data_device.selection event (for selections).  Immediately
    /// following the data_device_data_offer event, the new data_offer
    /// object will send out data_offer.offer events to describe the
    /// mime types it offers.
    #[allow(unused_variables)]
    fn on_data_offer(&mut self, id: ::client::protocol::DataOffer) {}
    
    /// This event is sent when an active drag-and-drop pointer enters
    /// a surface owned by the client.  The position of the pointer at
    /// enter time is provided by the x and y arguments, in surface
    /// local coordinates.
    #[allow(unused_variables)]
    fn on_enter(&mut self, serial: u32, surface: *mut ffi::wayland::WLObject, x: i32, y: i32, id: *mut ffi::wayland::WLObject) {}
    
    /// This event is sent when the drag-and-drop pointer leaves the
    /// surface and the session ends.  The client must destroy the
    /// wl_data_offer introduced at enter time at this point.
    #[allow(unused_variables)]
    fn on_leave(&mut self) {}
    
    /// This event is sent when the drag-and-drop pointer moves within
    /// the currently focused surface. The new position of the pointer
    /// is provided by the x and y arguments, in surface local
    /// coordinates.
    #[allow(unused_variables)]
    fn on_motion(&mut self, time: u32, x: i32, y: i32) {}
    
    /// The event is sent when a drag-and-drop operation is ended
    /// because the implicit grab is removed.
    #[allow(unused_variables)]
    fn on_drop(&mut self) {}
    
    /// The selection event is sent out to notify the client of a new
    /// wl_data_offer for the selection for this device.  The
    /// data_device.data_offer and the data_offer.offer events are
    /// sent out immediately before this event to introduce the data
    /// offer object.  The selection event is sent to a client
    /// immediately before receiving keyboard focus and when a new
    /// selection is set while the client has keyboard focus.  The
    /// data_offer is valid until a new data_offer or NULL is received
    /// or until the client loses keyboard focus.  The client must
    /// destroy the previous selection data_offer, if any, upon receiving
    /// this event.
    #[allow(unused_variables)]
    fn on_selection(&mut self, id: *mut ffi::wayland::WLObject) {}
}
