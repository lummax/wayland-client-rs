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
    static wl_shell_surface_interface: ffi::wayland::WLInterface;
}

/// These values are used to indicate which edge of a surface
/// is being dragged in a resize operation. The server may
/// use this information to adapt its behavior, e.g. choose
/// an appropriate cursor image.
#[repr(C)]
#[derive(Debug)]
pub enum ShellSurfaceResize {
    None = 0,
    Top = 1,
    Bottom = 2,
    Left = 4,
    TopLeft = 5,
    BottomLeft = 6,
    Right = 8,
    TopRight = 9,
    BottomRight = 10,
}

impl FromPrimitive for ShellSurfaceResize {
    fn from_u32(num: u32) -> Option<Self> {
        return match num {
            0 => Some(ShellSurfaceResize::None),
            1 => Some(ShellSurfaceResize::Top),
            2 => Some(ShellSurfaceResize::Bottom),
            4 => Some(ShellSurfaceResize::Left),
            5 => Some(ShellSurfaceResize::TopLeft),
            6 => Some(ShellSurfaceResize::BottomLeft),
            8 => Some(ShellSurfaceResize::Right),
            9 => Some(ShellSurfaceResize::TopRight),
            10 => Some(ShellSurfaceResize::BottomRight),
            _ => None
        }
    }

    fn from_i32(num: i32) -> Option<Self> {
        return Self::from_u32(num as u32);
    }
}

pub trait ShellSurfaceResizeSet {
    fn has_none(&self) -> bool;
    fn has_top(&self) -> bool;
    fn has_bottom(&self) -> bool;
    fn has_left(&self) -> bool;
    fn has_top_left(&self) -> bool;
    fn has_bottom_left(&self) -> bool;
    fn has_right(&self) -> bool;
    fn has_top_right(&self) -> bool;
    fn has_bottom_right(&self) -> bool;
}

impl ShellSurfaceResizeSet for u32 {
    fn is_none(&self) -> bool {
        return self & (ShellSurfaceResize::None as u32) != 0;
    }
    fn is_top(&self) -> bool {
        return self & (ShellSurfaceResize::Top as u32) != 0;
    }
    fn is_bottom(&self) -> bool {
        return self & (ShellSurfaceResize::Bottom as u32) != 0;
    }
    fn is_left(&self) -> bool {
        return self & (ShellSurfaceResize::Left as u32) != 0;
    }
    fn is_top_left(&self) -> bool {
        return self & (ShellSurfaceResize::TopLeft as u32) != 0;
    }
    fn is_bottom_left(&self) -> bool {
        return self & (ShellSurfaceResize::BottomLeft as u32) != 0;
    }
    fn is_right(&self) -> bool {
        return self & (ShellSurfaceResize::Right as u32) != 0;
    }
    fn is_top_right(&self) -> bool {
        return self & (ShellSurfaceResize::TopRight as u32) != 0;
    }
    fn is_bottom_right(&self) -> bool {
        return self & (ShellSurfaceResize::BottomRight as u32) != 0;
    }
}

impl ShellSurfaceResizeSet for i32 {
    fn is_none(&self) -> bool {
        return self & (ShellSurfaceResize::None as i32) != 0;
    }
    fn is_top(&self) -> bool {
        return self & (ShellSurfaceResize::Top as i32) != 0;
    }
    fn is_bottom(&self) -> bool {
        return self & (ShellSurfaceResize::Bottom as i32) != 0;
    }
    fn is_left(&self) -> bool {
        return self & (ShellSurfaceResize::Left as i32) != 0;
    }
    fn is_top_left(&self) -> bool {
        return self & (ShellSurfaceResize::TopLeft as i32) != 0;
    }
    fn is_bottom_left(&self) -> bool {
        return self & (ShellSurfaceResize::BottomLeft as i32) != 0;
    }
    fn is_right(&self) -> bool {
        return self & (ShellSurfaceResize::Right as i32) != 0;
    }
    fn is_top_right(&self) -> bool {
        return self & (ShellSurfaceResize::TopRight as i32) != 0;
    }
    fn is_bottom_right(&self) -> bool {
        return self & (ShellSurfaceResize::BottomRight as i32) != 0;
    }
}


/// These flags specify details of the expected behaviour
/// of transient surfaces. Used in the set_transient request.
#[repr(C)]
#[derive(Debug)]
pub enum ShellSurfaceTransient {
    /// do not set keyboard focus
    Inactive = 0x1,
    
    _Dummy,
}

impl FromPrimitive for ShellSurfaceTransient {
    fn from_u32(num: u32) -> Option<Self> {
        return match num {
            0x1 => Some(ShellSurfaceTransient::Inactive),
            _ => None
        }
    }

    fn from_i32(num: i32) -> Option<Self> {
        return Self::from_u32(num as u32);
    }
}

pub trait ShellSurfaceTransientSet {
    fn has_inactive(&self) -> bool;
    fn has_(&self) -> bool;
}

impl ShellSurfaceTransientSet for u32 {
    fn is_inactive(&self) -> bool {
        return self & (ShellSurfaceTransient::Inactive as u32) != 0;
    }
    fn is_(&self) -> bool {
        return self & (ShellSurfaceTransient::_Dummy as u32) != 0;
    }
}

impl ShellSurfaceTransientSet for i32 {
    fn is_inactive(&self) -> bool {
        return self & (ShellSurfaceTransient::Inactive as i32) != 0;
    }
    fn is_(&self) -> bool {
        return self & (ShellSurfaceTransient::_Dummy as i32) != 0;
    }
}


/// Hints to indicate to the compositor how to deal with a conflict
/// between the dimensions of the surface and the dimensions of the
/// output. The compositor is free to ignore this parameter.
#[repr(C)]
#[derive(Debug)]
pub enum ShellSurfaceFullscreenMethod {
    /// no preference, apply default policy
    Default = 0,
    /// scale, preserve the surface's aspect ratio and center on output
    Scale = 1,
    /// switch output mode to the smallest mode that can fit the surface, add black borders to compensate size mismatch
    Driver = 2,
    /// no upscaling, center on output and add black borders to compensate size mismatch
    Fill = 3,
}

impl FromPrimitive for ShellSurfaceFullscreenMethod {
    fn from_u32(num: u32) -> Option<Self> {
        return match num {
            0 => Some(ShellSurfaceFullscreenMethod::Default),
            1 => Some(ShellSurfaceFullscreenMethod::Scale),
            2 => Some(ShellSurfaceFullscreenMethod::Driver),
            3 => Some(ShellSurfaceFullscreenMethod::Fill),
            _ => None
        }
    }

    fn from_i32(num: i32) -> Option<Self> {
        return Self::from_u32(num as u32);
    }
}

pub trait ShellSurfaceFullscreenMethodSet {
    fn has_default(&self) -> bool;
    fn has_scale(&self) -> bool;
    fn has_driver(&self) -> bool;
    fn has_fill(&self) -> bool;
}

impl ShellSurfaceFullscreenMethodSet for u32 {
    fn is_default(&self) -> bool {
        return self & (ShellSurfaceFullscreenMethod::Default as u32) != 0;
    }
    fn is_scale(&self) -> bool {
        return self & (ShellSurfaceFullscreenMethod::Scale as u32) != 0;
    }
    fn is_driver(&self) -> bool {
        return self & (ShellSurfaceFullscreenMethod::Driver as u32) != 0;
    }
    fn is_fill(&self) -> bool {
        return self & (ShellSurfaceFullscreenMethod::Fill as u32) != 0;
    }
}

impl ShellSurfaceFullscreenMethodSet for i32 {
    fn is_default(&self) -> bool {
        return self & (ShellSurfaceFullscreenMethod::Default as i32) != 0;
    }
    fn is_scale(&self) -> bool {
        return self & (ShellSurfaceFullscreenMethod::Scale as i32) != 0;
    }
    fn is_driver(&self) -> bool {
        return self & (ShellSurfaceFullscreenMethod::Driver as i32) != 0;
    }
    fn is_fill(&self) -> bool {
        return self & (ShellSurfaceFullscreenMethod::Fill as i32) != 0;
    }
}


#[repr(C)]
enum ShellSurfaceEvent {
    Ping = 0,
    Configure = 1,
    PopupDone = 2,
}

impl FromPrimitive for ShellSurfaceEvent {
    fn from_u32(num: u32) -> Option<Self> {
        return match num {
            0 => Some(ShellSurfaceEvent::Ping),
            1 => Some(ShellSurfaceEvent::Configure),
            2 => Some(ShellSurfaceEvent::PopupDone),
            _ => None
        }
    }

    fn from_i32(num: i32) -> Option<Self> {
        return Self::from_u32(num as u32);
    }
}



#[repr(C)]
enum ShellSurfaceRequest {
    Pong = 0,
    Move = 1,
    Resize = 2,
    SetToplevel = 3,
    SetTransient = 4,
    SetFullscreen = 5,
    SetPopup = 6,
    SetMaximized = 7,
    SetTitle = 8,
    SetClass = 9,
}

impl FromPrimitive for ShellSurfaceRequest {
    fn from_u32(num: u32) -> Option<Self> {
        return match num {
            0 => Some(ShellSurfaceRequest::Pong),
            1 => Some(ShellSurfaceRequest::Move),
            2 => Some(ShellSurfaceRequest::Resize),
            3 => Some(ShellSurfaceRequest::SetToplevel),
            4 => Some(ShellSurfaceRequest::SetTransient),
            5 => Some(ShellSurfaceRequest::SetFullscreen),
            6 => Some(ShellSurfaceRequest::SetPopup),
            7 => Some(ShellSurfaceRequest::SetMaximized),
            8 => Some(ShellSurfaceRequest::SetTitle),
            9 => Some(ShellSurfaceRequest::SetClass),
            _ => None
        }
    }

    fn from_i32(num: i32) -> Option<Self> {
        return Self::from_u32(num as u32);
    }
}



/// An interface that may be implemented by a wl_surface, for
/// implementations that provide a desktop-style user interface.
/// 
/// It provides requests to treat surfaces like toplevel, fullscreen
/// or popup windows, move, resize or maximize them, associate
/// metadata like title and class, etc.
/// 
/// On the server side the object is automatically destroyed when
/// the related wl_surface is destroyed.  On client side,
/// wl_shell_surface_destroy() must be called before destroying
/// the wl_surface object.
#[derive(Debug)]
pub struct ShellSurface {
    proxy: BaseProxy,
}

impl ShellSurface {
    
    /// A client must respond to a ping event with a pong request or
    /// the client may be deemed unresponsive.
    pub fn pong(&mut self, serial: u32) {
        let proxy = self.as_mut_ptr() as *mut ffi::wayland::WLProxy;
        unsafe {
            ffi::wayland::wl_proxy_marshal(
                proxy, ShellSurfaceRequest::Pong as u32, serial
            );
        }
    }
    
    /// Start a pointer-driven move of the surface.
    /// 
    /// This request must be used in response to a button press event.
    /// The server may ignore move requests depending on the state of
    /// the surface (e.g. fullscreen or maximized).
    pub fn move_(&mut self, seat: &mut ::client::protocol::Seat, serial: u32) {
        let proxy = self.as_mut_ptr() as *mut ffi::wayland::WLProxy;
        let seatpointer = seat.as_mut_ptr() as *mut ffi::wayland::WLProxy;
        unsafe {
            ffi::wayland::wl_proxy_marshal(
                proxy, ShellSurfaceRequest::Move as u32, seatpointer, serial
            );
        }
    }
    
    /// Start a pointer-driven resizing of the surface.
    /// 
    /// This request must be used in response to a button press event.
    /// The server may ignore resize requests depending on the state of
    /// the surface (e.g. fullscreen or maximized).
    pub fn resize(&mut self, seat: &mut ::client::protocol::Seat, serial: u32, edges: u32) {
        let proxy = self.as_mut_ptr() as *mut ffi::wayland::WLProxy;
        let seatpointer = seat.as_mut_ptr() as *mut ffi::wayland::WLProxy;
        unsafe {
            ffi::wayland::wl_proxy_marshal(
                proxy, ShellSurfaceRequest::Resize as u32, seatpointer, serial, edges
            );
        }
    }
    
    /// Map the surface as a toplevel surface.
    /// 
    /// A toplevel surface is not fullscreen, maximized or transient.
    pub fn set_toplevel(&mut self) {
        let proxy = self.as_mut_ptr() as *mut ffi::wayland::WLProxy;
        unsafe {
            ffi::wayland::wl_proxy_marshal(
                proxy, ShellSurfaceRequest::SetToplevel as u32
            );
        }
    }
    
    /// Map the surface relative to an existing surface.
    /// 
    /// The x and y arguments specify the locations of the upper left
    /// corner of the surface relative to the upper left corner of the
    /// parent surface, in surface local coordinates.
    /// 
    /// The flags argument controls details of the transient behaviour.
    pub fn set_transient(&mut self, parent: &mut ::client::protocol::Surface, x: i32, y: i32, flags: u32) {
        let proxy = self.as_mut_ptr() as *mut ffi::wayland::WLProxy;
        let parentpointer = parent.as_mut_ptr() as *mut ffi::wayland::WLProxy;
        unsafe {
            ffi::wayland::wl_proxy_marshal(
                proxy, ShellSurfaceRequest::SetTransient as u32, parentpointer, x, y, flags
            );
        }
    }
    
    /// Map the surface as a fullscreen surface.
    /// 
    /// If an output parameter is given then the surface will be made
    /// fullscreen on that output. If the client does not specify the
    /// output then the compositor will apply its policy - usually
    /// choosing the output on which the surface has the biggest surface
    /// area.
    /// 
    /// The client may specify a method to resolve a size conflict
    /// between the output size and the surface size - this is provided
    /// through the method parameter.
    /// 
    /// The framerate parameter is used only when the method is set
    /// to "driver", to indicate the preferred framerate. A value of 0
    /// indicates that the app does not care about framerate.  The
    /// framerate is specified in mHz, that is framerate of 60000 is 60Hz.
    /// 
    /// A method of "scale" or "driver" implies a scaling operation of
    /// the surface, either via a direct scaling operation or a change of
    /// the output mode. This will override any kind of output scaling, so
    /// that mapping a surface with a buffer size equal to the mode can
    /// fill the screen independent of buffer_scale.
    /// 
    /// A method of "fill" means we don't scale up the buffer, however
    /// any output scale is applied. This means that you may run into
    /// an edge case where the application maps a buffer with the same
    /// size of the output mode but buffer_scale 1 (thus making a
    /// surface larger than the output). In this case it is allowed to
    /// downscale the results to fit the screen.
    /// 
    /// The compositor must reply to this request with a configure event
    /// with the dimensions for the output on which the surface will
    /// be made fullscreen.
    pub fn set_fullscreen(&mut self, method: u32, framerate: u32, output: Option<&mut ::client::protocol::Output>) {
        let proxy = self.as_mut_ptr() as *mut ffi::wayland::WLProxy;
        let outputpointer = output.map(|o| o.as_mut_ptr() as *mut ffi::wayland::WLProxy).unwrap_or(ptr::null_mut());
        unsafe {
            ffi::wayland::wl_proxy_marshal(
                proxy, ShellSurfaceRequest::SetFullscreen as u32, method, framerate, outputpointer
            );
        }
    }
    
    /// Map the surface as a popup.
    /// 
    /// A popup surface is a transient surface with an added pointer
    /// grab.
    /// 
    /// An existing implicit grab will be changed to owner-events mode,
    /// and the popup grab will continue after the implicit grab ends
    /// (i.e. releasing the mouse button does not cause the popup to
    /// be unmapped).
    /// 
    /// The popup grab continues until the window is destroyed or a
    /// mouse button is pressed in any other clients window. A click
    /// in any of the clients surfaces is reported as normal, however,
    /// clicks in other clients surfaces will be discarded and trigger
    /// the callback.
    /// 
    /// The x and y arguments specify the locations of the upper left
    /// corner of the surface relative to the upper left corner of the
    /// parent surface, in surface local coordinates.
    pub fn set_popup(&mut self, seat: &mut ::client::protocol::Seat, serial: u32, parent: &mut ::client::protocol::Surface, x: i32, y: i32, flags: u32) {
        let proxy = self.as_mut_ptr() as *mut ffi::wayland::WLProxy;
        let seatpointer = seat.as_mut_ptr() as *mut ffi::wayland::WLProxy;
        let parentpointer = parent.as_mut_ptr() as *mut ffi::wayland::WLProxy;
        unsafe {
            ffi::wayland::wl_proxy_marshal(
                proxy, ShellSurfaceRequest::SetPopup as u32, seatpointer, serial, parentpointer, x, y, flags
            );
        }
    }
    
    /// Map the surface as a maximized surface.
    /// 
    /// If an output parameter is given then the surface will be
    /// maximized on that output. If the client does not specify the
    /// output then the compositor will apply its policy - usually
    /// choosing the output on which the surface has the biggest surface
    /// area.
    /// 
    /// The compositor will reply with a configure event telling
    /// the expected new surface size. The operation is completed
    /// on the next buffer attach to this surface.
    /// 
    /// A maximized surface typically fills the entire output it is
    /// bound to, except for desktop element such as panels. This is
    /// the main difference between a maximized shell surface and a
    /// fullscreen shell surface.
    /// 
    /// The details depend on the compositor implementation.
    pub fn set_maximized(&mut self, output: Option<&mut ::client::protocol::Output>) {
        let proxy = self.as_mut_ptr() as *mut ffi::wayland::WLProxy;
        let outputpointer = output.map(|o| o.as_mut_ptr() as *mut ffi::wayland::WLProxy).unwrap_or(ptr::null_mut());
        unsafe {
            ffi::wayland::wl_proxy_marshal(
                proxy, ShellSurfaceRequest::SetMaximized as u32, outputpointer
            );
        }
    }
    
    /// Set a short title for the surface.
    /// 
    /// This string may be used to identify the surface in a task bar,
    /// window list, or other user interface elements provided by the
    /// compositor.
    /// 
    /// The string must be encoded in UTF-8.
    pub fn set_title(&mut self, title: &str) {
        let proxy = self.as_mut_ptr() as *mut ffi::wayland::WLProxy;
        let titlepointer = CString::new(title).unwrap().as_ptr();
        unsafe {
            ffi::wayland::wl_proxy_marshal(
                proxy, ShellSurfaceRequest::SetTitle as u32, titlepointer
            );
        }
    }
    
    /// Set a class for the surface.
    /// 
    /// The surface class identifies the general class of applications
    /// to which the surface belongs. A common convention is to use the
    /// file name (or the full path if it is a non-standard location) of
    /// the application's .desktop file as the class.
    pub fn set_class(&mut self, class_: &str) {
        let proxy = self.as_mut_ptr() as *mut ffi::wayland::WLProxy;
        let class_pointer = CString::new(class_).unwrap().as_ptr();
        unsafe {
            ffi::wayland::wl_proxy_marshal(
                proxy, ShellSurfaceRequest::SetClass as u32, class_pointer
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


impl FromRawPtr<ffi::wayland::WLProxy> for ShellSurface {
    fn from_mut_ptr(ptr: *mut ffi::wayland::WLProxy) -> Result<Self, &'static str> {
        return match FromRawPtr::from_mut_ptr(ptr) {
            Ok(proxy) => Ok(ShellSurface {
                proxy: proxy,
            }),
            Err(str) => Err(str),
        }
    }
}

impl AsRawPtr<ffi::wayland::WLProxy> for ShellSurface {
    fn as_mut_ptr(&mut self) -> *mut ffi::wayland::WLProxy {
        return self.proxy.as_mut_ptr();
    }
}

impl GetInterface for ShellSurface {
    fn get_interface() -> *const ffi::wayland::WLInterface {
        return &wl_shell_surface_interface as *const ffi::wayland::WLInterface;
    }
}

#[allow(unused_variables)]
extern fn shell_surface_event_dispatcher<T: ShellSurfaceEventHandler>(
        user_data: *mut c_void,
        _target: *mut c_void,
        opcode: uint32_t,
        _message: *const ffi::wayland::WLMessage,
        arguments: *mut ffi::wayland::WLArgument) -> c_int {
    let object = user_data as *mut T;
    return match ShellSurfaceEvent::from_u32(opcode) {
        Some(event) => {
            match event {
                ShellSurfaceEvent::Ping => {
                    let serial = unsafe { *(*arguments.offset(0)).uint() };
                    unsafe { (*object).on_ping(serial); }
                },
                ShellSurfaceEvent::Configure => {
                    let edges = unsafe { *(*arguments.offset(0)).uint() };
                    let width = unsafe { *(*arguments.offset(1)).int() };
                    let height = unsafe { *(*arguments.offset(2)).int() };
                    unsafe { (*object).on_configure(edges, width, height); }
                },
                ShellSurfaceEvent::PopupDone => {
                    unsafe { (*object).on_popup_done(); }
                },
            }
            0
        },
        _ => -1,
    }
}

pub trait ShellSurfaceEventHandler: Sized {
    fn connect_dispatcher(&mut self) {
        unsafe {
            ffi::wayland::wl_proxy_add_dispatcher(
                self.get_shell_surface().as_mut_ptr(),
                shell_surface_event_dispatcher::<Self>,
                self as *mut Self as *mut c_void,
                ptr::null_mut());
        }
    }

    fn get_shell_surface(&mut self) -> &mut ShellSurface;
    
    /// Ping a client to check if it is receiving events and sending
    /// requests. A client is expected to reply with a pong request.
    #[allow(unused_variables)]
    fn on_ping(&mut self, serial: u32) {}
    
    /// The configure event asks the client to resize its surface.
    /// 
    /// The size is a hint, in the sense that the client is free to
    /// ignore it if it doesn't resize, pick a smaller size (to
    /// satisfy aspect ratio or resize in steps of NxM pixels).
    /// 
    /// The edges parameter provides a hint about how the surface
    /// was resized. The client may use this information to decide
    /// how to adjust its content to the new size (e.g. a scrolling
    /// area might adjust its content position to leave the viewable
    /// content unmoved).
    /// 
    /// The client is free to dismiss all but the last configure
    /// event it received.
    /// 
    /// The width and height arguments specify the size of the window
    /// in surface local coordinates.
    #[allow(unused_variables)]
    fn on_configure(&mut self, edges: u32, width: i32, height: i32) {}
    
    /// The popup_done event is sent out when a popup grab is broken,
    /// that is, when the user clicks a surface that doesn't belong
    /// to the client owning the popup surface.
    #[allow(unused_variables)]
    fn on_popup_done(&mut self) {}
}
