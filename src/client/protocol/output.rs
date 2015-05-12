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
    static wl_output_interface: ffi::wayland::WLInterface;
}

/// This enumeration describes how the physical
/// pixels on an output are layed out.
#[repr(C)]
#[derive(Debug)]
pub enum OutputSubpixel {
    Unknown = 0,
    None = 1,
    HorizontalRgb = 2,
    HorizontalBgr = 3,
    VerticalRgb = 4,
    VerticalBgr = 5,
}

impl FromPrimitive for OutputSubpixel {
    fn from_u32(num: u32) -> Option<Self> {
        return match num {
            0 => Some(OutputSubpixel::Unknown),
            1 => Some(OutputSubpixel::None),
            2 => Some(OutputSubpixel::HorizontalRgb),
            3 => Some(OutputSubpixel::HorizontalBgr),
            4 => Some(OutputSubpixel::VerticalRgb),
            5 => Some(OutputSubpixel::VerticalBgr),
            _ => None
        }
    }

    fn from_i32(num: i32) -> Option<Self> {
        return Self::from_u32(num as u32);
    }
}

pub trait OutputSubpixelSet {
    fn has_unknown(&self) -> bool;
    fn has_none(&self) -> bool;
    fn has_horizontal_rgb(&self) -> bool;
    fn has_horizontal_bgr(&self) -> bool;
    fn has_vertical_rgb(&self) -> bool;
    fn has_vertical_bgr(&self) -> bool;
}

impl OutputSubpixelSet for u32 {
    fn is_unknown(&self) -> bool {
        return self & (OutputSubpixel::Unknown as u32) != 0;
    }
    fn is_none(&self) -> bool {
        return self & (OutputSubpixel::None as u32) != 0;
    }
    fn is_horizontal_rgb(&self) -> bool {
        return self & (OutputSubpixel::HorizontalRgb as u32) != 0;
    }
    fn is_horizontal_bgr(&self) -> bool {
        return self & (OutputSubpixel::HorizontalBgr as u32) != 0;
    }
    fn is_vertical_rgb(&self) -> bool {
        return self & (OutputSubpixel::VerticalRgb as u32) != 0;
    }
    fn is_vertical_bgr(&self) -> bool {
        return self & (OutputSubpixel::VerticalBgr as u32) != 0;
    }
}

impl OutputSubpixelSet for i32 {
    fn is_unknown(&self) -> bool {
        return self & (OutputSubpixel::Unknown as i32) != 0;
    }
    fn is_none(&self) -> bool {
        return self & (OutputSubpixel::None as i32) != 0;
    }
    fn is_horizontal_rgb(&self) -> bool {
        return self & (OutputSubpixel::HorizontalRgb as i32) != 0;
    }
    fn is_horizontal_bgr(&self) -> bool {
        return self & (OutputSubpixel::HorizontalBgr as i32) != 0;
    }
    fn is_vertical_rgb(&self) -> bool {
        return self & (OutputSubpixel::VerticalRgb as i32) != 0;
    }
    fn is_vertical_bgr(&self) -> bool {
        return self & (OutputSubpixel::VerticalBgr as i32) != 0;
    }
}


/// This describes the transform that a compositor will apply to a
/// surface to compensate for the rotation or mirroring of an
/// output device.
/// 
/// The flipped values correspond to an initial flip around a
/// vertical axis followed by rotation.
/// 
/// The purpose is mainly to allow clients render accordingly and
/// tell the compositor, so that for fullscreen surfaces, the
/// compositor will still be able to scan out directly from client
/// surfaces.
#[repr(C)]
#[derive(Debug)]
pub enum OutputTransform {
    Normal = 0,
    _90 = 1,
    _180 = 2,
    _270 = 3,
    Flipped = 4,
    Flipped90 = 5,
    Flipped180 = 6,
    Flipped270 = 7,
}

impl FromPrimitive for OutputTransform {
    fn from_u32(num: u32) -> Option<Self> {
        return match num {
            0 => Some(OutputTransform::Normal),
            1 => Some(OutputTransform::_90),
            2 => Some(OutputTransform::_180),
            3 => Some(OutputTransform::_270),
            4 => Some(OutputTransform::Flipped),
            5 => Some(OutputTransform::Flipped90),
            6 => Some(OutputTransform::Flipped180),
            7 => Some(OutputTransform::Flipped270),
            _ => None
        }
    }

    fn from_i32(num: i32) -> Option<Self> {
        return Self::from_u32(num as u32);
    }
}

pub trait OutputTransformSet {
    fn has_normal(&self) -> bool;
    fn has_90(&self) -> bool;
    fn has_180(&self) -> bool;
    fn has_270(&self) -> bool;
    fn has_flipped(&self) -> bool;
    fn has_flipped_90(&self) -> bool;
    fn has_flipped_180(&self) -> bool;
    fn has_flipped_270(&self) -> bool;
}

impl OutputTransformSet for u32 {
    fn is_normal(&self) -> bool {
        return self & (OutputTransform::Normal as u32) != 0;
    }
    fn is_90(&self) -> bool {
        return self & (OutputTransform::_90 as u32) != 0;
    }
    fn is_180(&self) -> bool {
        return self & (OutputTransform::_180 as u32) != 0;
    }
    fn is_270(&self) -> bool {
        return self & (OutputTransform::_270 as u32) != 0;
    }
    fn is_flipped(&self) -> bool {
        return self & (OutputTransform::Flipped as u32) != 0;
    }
    fn is_flipped_90(&self) -> bool {
        return self & (OutputTransform::Flipped90 as u32) != 0;
    }
    fn is_flipped_180(&self) -> bool {
        return self & (OutputTransform::Flipped180 as u32) != 0;
    }
    fn is_flipped_270(&self) -> bool {
        return self & (OutputTransform::Flipped270 as u32) != 0;
    }
}

impl OutputTransformSet for i32 {
    fn is_normal(&self) -> bool {
        return self & (OutputTransform::Normal as i32) != 0;
    }
    fn is_90(&self) -> bool {
        return self & (OutputTransform::_90 as i32) != 0;
    }
    fn is_180(&self) -> bool {
        return self & (OutputTransform::_180 as i32) != 0;
    }
    fn is_270(&self) -> bool {
        return self & (OutputTransform::_270 as i32) != 0;
    }
    fn is_flipped(&self) -> bool {
        return self & (OutputTransform::Flipped as i32) != 0;
    }
    fn is_flipped_90(&self) -> bool {
        return self & (OutputTransform::Flipped90 as i32) != 0;
    }
    fn is_flipped_180(&self) -> bool {
        return self & (OutputTransform::Flipped180 as i32) != 0;
    }
    fn is_flipped_270(&self) -> bool {
        return self & (OutputTransform::Flipped270 as i32) != 0;
    }
}


/// These flags describe properties of an output mode.
/// They are used in the flags bitfield of the mode event.
#[repr(C)]
#[derive(Debug)]
pub enum OutputMode {
    /// indicates this is the current mode
    Current = 0x1,
    /// indicates this is the preferred mode
    Preferred = 0x2,
}

impl FromPrimitive for OutputMode {
    fn from_u32(num: u32) -> Option<Self> {
        return match num {
            0x1 => Some(OutputMode::Current),
            0x2 => Some(OutputMode::Preferred),
            _ => None
        }
    }

    fn from_i32(num: i32) -> Option<Self> {
        return Self::from_u32(num as u32);
    }
}

pub trait OutputModeSet {
    fn has_current(&self) -> bool;
    fn has_preferred(&self) -> bool;
}

impl OutputModeSet for u32 {
    fn is_current(&self) -> bool {
        return self & (OutputMode::Current as u32) != 0;
    }
    fn is_preferred(&self) -> bool {
        return self & (OutputMode::Preferred as u32) != 0;
    }
}

impl OutputModeSet for i32 {
    fn is_current(&self) -> bool {
        return self & (OutputMode::Current as i32) != 0;
    }
    fn is_preferred(&self) -> bool {
        return self & (OutputMode::Preferred as i32) != 0;
    }
}


#[repr(C)]
enum OutputEvent {
    Geometry = 0,
    Mode = 1,
    Done = 2,
    Scale = 3,
}

impl FromPrimitive for OutputEvent {
    fn from_u32(num: u32) -> Option<Self> {
        return match num {
            0 => Some(OutputEvent::Geometry),
            1 => Some(OutputEvent::Mode),
            2 => Some(OutputEvent::Done),
            3 => Some(OutputEvent::Scale),
            _ => None
        }
    }

    fn from_i32(num: i32) -> Option<Self> {
        return Self::from_u32(num as u32);
    }
}




/// An output describes part of the compositor geometry.  The
/// compositor works in the 'compositor coordinate system' and an
/// output corresponds to rectangular area in that space that is
/// actually visible.  This typically corresponds to a monitor that
/// displays part of the compositor space.  This object is published
/// as global during start up, or when a monitor is hotplugged.
#[derive(Debug)]
pub struct Output {
    proxy: BaseProxy,
}

impl Output {

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


impl FromRawPtr<ffi::wayland::WLProxy> for Output {
    fn from_mut_ptr(ptr: *mut ffi::wayland::WLProxy) -> Result<Self, &'static str> {
        return match FromRawPtr::from_mut_ptr(ptr) {
            Ok(proxy) => Ok(Output {
                proxy: proxy,
            }),
            Err(str) => Err(str),
        }
    }
}

impl AsRawPtr<ffi::wayland::WLProxy> for Output {
    fn as_mut_ptr(&mut self) -> *mut ffi::wayland::WLProxy {
        return self.proxy.as_mut_ptr();
    }
}

impl GetInterface for Output {
    fn get_interface() -> *const ffi::wayland::WLInterface {
        return &wl_output_interface as *const ffi::wayland::WLInterface;
    }
}

#[allow(unused_variables)]
extern fn output_event_dispatcher<T: OutputEventHandler>(
        user_data: *mut c_void,
        _target: *mut c_void,
        opcode: uint32_t,
        _message: *const ffi::wayland::WLMessage,
        arguments: *mut ffi::wayland::WLArgument) -> c_int {
    let object = user_data as *mut T;
    return match OutputEvent::from_u32(opcode) {
        Some(event) => {
            match event {
                OutputEvent::Geometry => {
                    let x = unsafe { *(*arguments.offset(0)).int() };
                    let y = unsafe { *(*arguments.offset(1)).int() };
                    let physical_width = unsafe { *(*arguments.offset(2)).int() };
                    let physical_height = unsafe { *(*arguments.offset(3)).int() };
                    let subpixel = unsafe { *(*arguments.offset(4)).int() };
                    let make_raw = unsafe { *(*arguments.offset(5)).string() };
                    let make_buffer = unsafe { CStr::from_ptr(make_raw).to_bytes() };
                    let make = String::from_utf8(make_buffer.to_vec()).unwrap();
                    let model_raw = unsafe { *(*arguments.offset(6)).string() };
                    let model_buffer = unsafe { CStr::from_ptr(model_raw).to_bytes() };
                    let model = String::from_utf8(model_buffer.to_vec()).unwrap();
                    let transform = unsafe { *(*arguments.offset(7)).int() };
                    unsafe { (*object).on_geometry(x, y, physical_width, physical_height, subpixel, make, model, transform); }
                },
                OutputEvent::Mode => {
                    let flags = unsafe { *(*arguments.offset(0)).uint() };
                    let width = unsafe { *(*arguments.offset(1)).int() };
                    let height = unsafe { *(*arguments.offset(2)).int() };
                    let refresh = unsafe { *(*arguments.offset(3)).int() };
                    unsafe { (*object).on_mode(flags, width, height, refresh); }
                },
                OutputEvent::Done => {
                    unsafe { (*object).on_done(); }
                },
                OutputEvent::Scale => {
                    let factor = unsafe { *(*arguments.offset(0)).int() };
                    unsafe { (*object).on_scale(factor); }
                },
            }
            0
        },
        _ => -1,
    }
}

pub trait OutputEventHandler: Sized {
    fn connect_dispatcher(&mut self) {
        unsafe {
            ffi::wayland::wl_proxy_add_dispatcher(
                self.get_output().as_mut_ptr(),
                output_event_dispatcher::<Self>,
                self as *mut Self as *mut c_void,
                ptr::null_mut());
        }
    }

    fn get_output(&mut self) -> &mut Output;
    
    /// The geometry event describes geometric properties of the output.
    /// The event is sent when binding to the output object and whenever
    /// any of the properties change.
    #[allow(unused_variables)]
    fn on_geometry(&mut self, x: i32, y: i32, physical_width: i32, physical_height: i32, subpixel: i32, make: String, model: String, transform: i32) {}
    
    /// The mode event describes an available mode for the output.
    /// 
    /// The event is sent when binding to the output object and there
    /// will always be one mode, the current mode.  The event is sent
    /// again if an output changes mode, for the mode that is now
    /// current.  In other words, the current mode is always the last
    /// mode that was received with the current flag set.
    /// 
    /// The size of a mode is given in physical hardware units of
    /// the output device. This is not necessarily the same as
    /// the output size in the global compositor space. For instance,
    /// the output may be scaled, as described in wl_output.scale,
    /// or transformed , as described in wl_output.transform.
    #[allow(unused_variables)]
    fn on_mode(&mut self, flags: u32, width: i32, height: i32, refresh: i32) {}
    
    /// This event is sent after all other properties has been
    /// sent after binding to the output object and after any
    /// other property changes done after that. This allows
    /// changes to the output properties to be seen as
    /// atomic, even if they happen via multiple events.
    #[allow(unused_variables)]
    fn on_done(&mut self) {}
    
    /// This event contains scaling geometry information
    /// that is not in the geometry event. It may be sent after
    /// binding the output object or if the output scale changes
    /// later. If it is not sent, the client should assume a
    /// scale of 1.
    /// 
    /// A scale larger than 1 means that the compositor will
    /// automatically scale surface buffers by this amount
    /// when rendering. This is used for very high resolution
    /// displays where applications rendering at the native
    /// resolution would be too small to be legible.
    /// 
    /// It is intended that scaling aware clients track the
    /// current output of a surface, and if it is on a scaled
    /// output it should use wl_surface.set_buffer_scale with
    /// the scale of the output. That way the compositor can
    /// avoid scaling the surface, and the client can supply
    /// a higher detail image.
    #[allow(unused_variables)]
    fn on_scale(&mut self, factor: i32) {}
}
