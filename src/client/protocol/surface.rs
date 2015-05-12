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
    static wl_surface_interface: ffi::wayland::WLInterface;
}

/// These errors can be emitted in response to wl_surface requests.
#[repr(C)]
#[derive(Debug)]
pub enum SurfaceError {
    /// buffer scale value is invalid
    InvalidScale = 0,
    /// buffer transform value is invalid
    InvalidTransform = 1,
}

impl FromPrimitive for SurfaceError {
    fn from_u32(num: u32) -> Option<Self> {
        return match num {
            0 => Some(SurfaceError::InvalidScale),
            1 => Some(SurfaceError::InvalidTransform),
            _ => None
        }
    }

    fn from_i32(num: i32) -> Option<Self> {
        return Self::from_u32(num as u32);
    }
}

pub trait SurfaceErrorSet {
    fn has_invalid_scale(&self) -> bool;
    fn has_invalid_transform(&self) -> bool;
}

impl SurfaceErrorSet for u32 {
    fn has_invalid_scale(&self) -> bool {
        return self & (SurfaceError::InvalidScale as u32) != 0;
    }
    fn has_invalid_transform(&self) -> bool {
        return self & (SurfaceError::InvalidTransform as u32) != 0;
    }
}

impl SurfaceErrorSet for i32 {
    fn has_invalid_scale(&self) -> bool {
        return self & (SurfaceError::InvalidScale as i32) != 0;
    }
    fn has_invalid_transform(&self) -> bool {
        return self & (SurfaceError::InvalidTransform as i32) != 0;
    }
}


#[repr(C)]
enum SurfaceEvent {
    Enter = 0,
    Leave = 1,
}

impl FromPrimitive for SurfaceEvent {
    fn from_u32(num: u32) -> Option<Self> {
        return match num {
            0 => Some(SurfaceEvent::Enter),
            1 => Some(SurfaceEvent::Leave),
            _ => None
        }
    }

    fn from_i32(num: i32) -> Option<Self> {
        return Self::from_u32(num as u32);
    }
}



#[repr(C)]
enum SurfaceRequest {
    Destroy = 0,
    Attach = 1,
    Damage = 2,
    Frame = 3,
    SetOpaqueRegion = 4,
    SetInputRegion = 5,
    Commit = 6,
    SetBufferTransform = 7,
    SetBufferScale = 8,
}

impl FromPrimitive for SurfaceRequest {
    fn from_u32(num: u32) -> Option<Self> {
        return match num {
            0 => Some(SurfaceRequest::Destroy),
            1 => Some(SurfaceRequest::Attach),
            2 => Some(SurfaceRequest::Damage),
            3 => Some(SurfaceRequest::Frame),
            4 => Some(SurfaceRequest::SetOpaqueRegion),
            5 => Some(SurfaceRequest::SetInputRegion),
            6 => Some(SurfaceRequest::Commit),
            7 => Some(SurfaceRequest::SetBufferTransform),
            8 => Some(SurfaceRequest::SetBufferScale),
            _ => None
        }
    }

    fn from_i32(num: i32) -> Option<Self> {
        return Self::from_u32(num as u32);
    }
}



/// A surface is a rectangular area that is displayed on the screen.
/// It has a location, size and pixel contents.
/// 
/// The size of a surface (and relative positions on it) is described
/// in surface local coordinates, which may differ from the buffer
/// local coordinates of the pixel content, in case a buffer_transform
/// or a buffer_scale is used.
/// 
/// A surface without a "role" is fairly useless, a compositor does
/// not know where, when or how to present it. The role is the
/// purpose of a wl_surface. Examples of roles are a cursor for a
/// pointer (as set by wl_pointer.set_cursor), a drag icon
/// (wl_data_device.start_drag), a sub-surface
/// (wl_subcompositor.get_subsurface), and a window as defined by a
/// shell protocol (e.g. wl_shell.get_shell_surface).
/// 
/// A surface can have only one role at a time. Initially a
/// wl_surface does not have a role. Once a wl_surface is given a
/// role, it is set permanently for the whole lifetime of the
/// wl_surface object. Giving the current role again is allowed,
/// unless explicitly forbidden by the relevant interface
/// specification.
/// 
/// Surface roles are given by requests in other interfaces such as
/// wl_pointer.set_cursor. The request should explicitly mention
/// that this request gives a role to a wl_surface. Often, this
/// request also creates a new protocol object that represents the
/// role and adds additional functionality to wl_surface. When a
/// client wants to destroy a wl_surface, they must destroy this 'role
/// object' before the wl_surface.
/// 
/// Destroying the role object does not remove the role from the
/// wl_surface, but it may stop the wl_surface from "playing the role".
/// For instance, if a wl_subsurface object is destroyed, the wl_surface
/// it was created for will be unmapped and forget its position and
/// z-order. It is allowed to create a wl_subsurface for the same
/// wl_surface again, but it is not allowed to use the wl_surface as
/// a cursor (cursor is a different role than sub-surface, and role
/// switching is not allowed).
#[derive(Debug)]
pub struct Surface {
    proxy: BaseProxy,
}

impl Surface {
    
    /// Deletes the surface and invalidates its object ID.
    pub fn destroy(mut self) {
        let proxy = self.as_mut_ptr() as *mut ffi::wayland::WLProxy;
        unsafe {
            ffi::wayland::wl_proxy_marshal(
                proxy, SurfaceRequest::Destroy as u32
            );
        }
    }
    
    /// Set a buffer as the content of this surface.
    /// 
    /// The new size of the surface is calculated based on the buffer
    /// size transformed by the inverse buffer_transform and the
    /// inverse buffer_scale. This means that the supplied buffer
    /// must be an integer multiple of the buffer_scale.
    /// 
    /// The x and y arguments specify the location of the new pending
    /// buffer's upper left corner, relative to the current buffer's upper
    /// left corner, in surface local coordinates. In other words, the
    /// x and y, combined with the new surface size define in which
    /// directions the surface's size changes.
    /// 
    /// Surface contents are double-buffered state, see wl_surface.commit.
    /// 
    /// The initial surface contents are void; there is no content.
    /// wl_surface.attach assigns the given wl_buffer as the pending
    /// wl_buffer. wl_surface.commit makes the pending wl_buffer the new
    /// surface contents, and the size of the surface becomes the size
    /// calculated from the wl_buffer, as described above. After commit,
    /// there is no pending buffer until the next attach.
    /// 
    /// Committing a pending wl_buffer allows the compositor to read the
    /// pixels in the wl_buffer. The compositor may access the pixels at
    /// any time after the wl_surface.commit request. When the compositor
    /// will not access the pixels anymore, it will send the
    /// wl_buffer.release event. Only after receiving wl_buffer.release,
    /// the client may re-use the wl_buffer. A wl_buffer that has been
    /// attached and then replaced by another attach instead of committed
    /// will not receive a release event, and is not used by the
    /// compositor.
    /// 
    /// Destroying the wl_buffer after wl_buffer.release does not change
    /// the surface contents. However, if the client destroys the
    /// wl_buffer before receiving the wl_buffer.release event, the surface
    /// contents become undefined immediately.
    /// 
    /// If wl_surface.attach is sent with a NULL wl_buffer, the
    /// following wl_surface.commit will remove the surface content.
    pub fn attach(&mut self, buffer: Option<&mut ::client::protocol::Buffer>, x: i32, y: i32) {
        let proxy = self.as_mut_ptr() as *mut ffi::wayland::WLProxy;
        let bufferpointer = buffer.map(|o| o.as_mut_ptr() as *mut ffi::wayland::WLProxy).unwrap_or(ptr::null_mut());
        unsafe {
            ffi::wayland::wl_proxy_marshal(
                proxy, SurfaceRequest::Attach as u32, bufferpointer, x, y
            );
        }
    }
    
    /// This request is used to describe the regions where the pending
    /// buffer is different from the current surface contents, and where
    /// the surface therefore needs to be repainted. The pending buffer
    /// must be set by wl_surface.attach before sending damage. The
    /// compositor ignores the parts of the damage that fall outside of
    /// the surface.
    /// 
    /// Damage is double-buffered state, see wl_surface.commit.
    /// 
    /// The damage rectangle is specified in surface local coordinates.
    /// 
    /// The initial value for pending damage is empty: no damage.
    /// wl_surface.damage adds pending damage: the new pending damage
    /// is the union of old pending damage and the given rectangle.
    /// 
    /// wl_surface.commit assigns pending damage as the current damage,
    /// and clears pending damage. The server will clear the current
    /// damage as it repaints the surface.
    pub fn damage(&mut self, x: i32, y: i32, width: i32, height: i32) {
        let proxy = self.as_mut_ptr() as *mut ffi::wayland::WLProxy;
        unsafe {
            ffi::wayland::wl_proxy_marshal(
                proxy, SurfaceRequest::Damage as u32, x, y, width, height
            );
        }
    }
    
    /// Request a notification when it is a good time start drawing a new
    /// frame, by creating a frame callback. This is useful for throttling
    /// redrawing operations, and driving animations.
    /// 
    /// When a client is animating on a wl_surface, it can use the 'frame'
    /// request to get notified when it is a good time to draw and commit the
    /// next frame of animation. If the client commits an update earlier than
    /// that, it is likely that some updates will not make it to the display,
    /// and the client is wasting resources by drawing too often.
    /// 
    /// The frame request will take effect on the next wl_surface.commit.
    /// The notification will only be posted for one frame unless
    /// requested again. For a wl_surface, the notifications are posted in
    /// the order the frame requests were committed.
    /// 
    /// The server must send the notifications so that a client
    /// will not send excessive updates, while still allowing
    /// the highest possible update rate for clients that wait for the reply
    /// before drawing again. The server should give some time for the client
    /// to draw and commit after sending the frame callback events to let them
    /// hit the next output refresh.
    /// 
    /// A server should avoid signalling the frame callbacks if the
    /// surface is not visible in any way, e.g. the surface is off-screen,
    /// or completely obscured by other opaque surfaces.
    /// 
    /// The object returned by this request will be destroyed by the
    /// compositor after the callback is fired and as such the client must not
    /// attempt to use it after that point.
    /// 
    /// The callback_data passed in the callback is the current time, in
    /// milliseconds, with an undefined base.
    pub fn frame(&mut self) -> Result<::client::protocol::Callback, &'static str> {
        let proxy = self.as_mut_ptr() as *mut ffi::wayland::WLProxy;
        let object = unsafe {
            ffi::wayland::wl_proxy_marshal_constructor(
                proxy, SurfaceRequest::Frame as u32, ::client::protocol::Callback::get_interface(), ptr::null::<c_void>()
            )
        };
        return ::client::protocol::Callback::from_mut_ptr(object);
    }
    
    /// This request sets the region of the surface that contains
    /// opaque content.
    /// 
    /// The opaque region is an optimization hint for the compositor
    /// that lets it optimize out redrawing of content behind opaque
    /// regions.  Setting an opaque region is not required for correct
    /// behaviour, but marking transparent content as opaque will result
    /// in repaint artifacts.
    /// 
    /// The opaque region is specified in surface local coordinates.
    /// 
    /// The compositor ignores the parts of the opaque region that fall
    /// outside of the surface.
    /// 
    /// Opaque region is double-buffered state, see wl_surface.commit.
    /// 
    /// wl_surface.set_opaque_region changes the pending opaque region.
    /// wl_surface.commit copies the pending region to the current region.
    /// Otherwise, the pending and current regions are never changed.
    /// 
    /// The initial value for opaque region is empty. Setting the pending
    /// opaque region has copy semantics, and the wl_region object can be
    /// destroyed immediately. A NULL wl_region causes the pending opaque
    /// region to be set to empty.
    pub fn set_opaque_region(&mut self, region: Option<&mut ::client::protocol::Region>) {
        let proxy = self.as_mut_ptr() as *mut ffi::wayland::WLProxy;
        let regionpointer = region.map(|o| o.as_mut_ptr() as *mut ffi::wayland::WLProxy).unwrap_or(ptr::null_mut());
        unsafe {
            ffi::wayland::wl_proxy_marshal(
                proxy, SurfaceRequest::SetOpaqueRegion as u32, regionpointer
            );
        }
    }
    
    /// This request sets the region of the surface that can receive
    /// pointer and touch events.
    /// 
    /// Input events happening outside of this region will try the next
    /// surface in the server surface stack. The compositor ignores the
    /// parts of the input region that fall outside of the surface.
    /// 
    /// The input region is specified in surface local coordinates.
    /// 
    /// Input region is double-buffered state, see wl_surface.commit.
    /// 
    /// wl_surface.set_input_region changes the pending input region.
    /// wl_surface.commit copies the pending region to the current region.
    /// Otherwise the pending and current regions are never changed,
    /// except cursor and icon surfaces are special cases, see
    /// wl_pointer.set_cursor and wl_data_device.start_drag.
    /// 
    /// The initial value for input region is infinite. That means the
    /// whole surface will accept input. Setting the pending input region
    /// has copy semantics, and the wl_region object can be destroyed
    /// immediately. A NULL wl_region causes the input region to be set
    /// to infinite.
    pub fn set_input_region(&mut self, region: Option<&mut ::client::protocol::Region>) {
        let proxy = self.as_mut_ptr() as *mut ffi::wayland::WLProxy;
        let regionpointer = region.map(|o| o.as_mut_ptr() as *mut ffi::wayland::WLProxy).unwrap_or(ptr::null_mut());
        unsafe {
            ffi::wayland::wl_proxy_marshal(
                proxy, SurfaceRequest::SetInputRegion as u32, regionpointer
            );
        }
    }
    
    /// Surface state (input, opaque, and damage regions, attached buffers,
    /// etc.) is double-buffered. Protocol requests modify the pending
    /// state, as opposed to current state in use by the compositor. Commit
    /// request atomically applies all pending state, replacing the current
    /// state. After commit, the new pending state is as documented for each
    /// related request.
    /// 
    /// On commit, a pending wl_buffer is applied first, all other state
    /// second. This means that all coordinates in double-buffered state are
    /// relative to the new wl_buffer coming into use, except for
    /// wl_surface.attach itself. If there is no pending wl_buffer, the
    /// coordinates are relative to the current surface contents.
    /// 
    /// All requests that need a commit to become effective are documented
    /// to affect double-buffered state.
    /// 
    /// Other interfaces may add further double-buffered surface state.
    pub fn commit(&mut self) {
        let proxy = self.as_mut_ptr() as *mut ffi::wayland::WLProxy;
        unsafe {
            ffi::wayland::wl_proxy_marshal(
                proxy, SurfaceRequest::Commit as u32
            );
        }
    }
    
    /// This request sets an optional transformation on how the compositor
    /// interprets the contents of the buffer attached to the surface. The
    /// accepted values for the transform parameter are the values for
    /// wl_output.transform.
    /// 
    /// Buffer transform is double-buffered state, see wl_surface.commit.
    /// 
    /// A newly created surface has its buffer transformation set to normal.
    /// 
    /// wl_surface.set_buffer_transform changes the pending buffer
    /// transformation. wl_surface.commit copies the pending buffer
    /// transformation to the current one. Otherwise, the pending and current
    /// values are never changed.
    /// 
    /// The purpose of this request is to allow clients to render content
    /// according to the output transform, thus permiting the compositor to
    /// use certain optimizations even if the display is rotated. Using
    /// hardware overlays and scanning out a client buffer for fullscreen
    /// surfaces are examples of such optimizations. Those optimizations are
    /// highly dependent on the compositor implementation, so the use of this
    /// request should be considered on a case-by-case basis.
    /// 
    /// Note that if the transform value includes 90 or 270 degree rotation,
    /// the width of the buffer will become the surface height and the height
    /// of the buffer will become the surface width.
    /// 
    /// If transform is not one of the values from the
    /// wl_output.transform enum the invalid_transform protocol error
    /// is raised.
    pub fn set_buffer_transform(&mut self, transform: i32) {
        let proxy = self.as_mut_ptr() as *mut ffi::wayland::WLProxy;
        unsafe {
            ffi::wayland::wl_proxy_marshal(
                proxy, SurfaceRequest::SetBufferTransform as u32, transform
            );
        }
    }
    
    /// This request sets an optional scaling factor on how the compositor
    /// interprets the contents of the buffer attached to the window.
    /// 
    /// Buffer scale is double-buffered state, see wl_surface.commit.
    /// 
    /// A newly created surface has its buffer scale set to 1.
    /// 
    /// wl_surface.set_buffer_scale changes the pending buffer scale.
    /// wl_surface.commit copies the pending buffer scale to the current one.
    /// Otherwise, the pending and current values are never changed.
    /// 
    /// The purpose of this request is to allow clients to supply higher
    /// resolution buffer data for use on high resolution outputs. Its
    /// intended that you pick the same	buffer scale as the scale of the
    /// output that the surface is displayed on.This means the compositor
    /// can avoid scaling when rendering the surface on that output.
    /// 
    /// Note that if the scale is larger than 1, then you have to attach
    /// a buffer that is larger (by a factor of scale in each dimension)
    /// than the desired surface size.
    /// 
    /// If scale is not positive the invalid_scale protocol error is
    /// raised.
    pub fn set_buffer_scale(&mut self, scale: i32) {
        let proxy = self.as_mut_ptr() as *mut ffi::wayland::WLProxy;
        unsafe {
            ffi::wayland::wl_proxy_marshal(
                proxy, SurfaceRequest::SetBufferScale as u32, scale
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


impl FromRawPtr<ffi::wayland::WLProxy> for Surface {
    fn from_mut_ptr(ptr: *mut ffi::wayland::WLProxy) -> Result<Self, &'static str> {
        return match FromRawPtr::from_mut_ptr(ptr) {
            Ok(proxy) => Ok(Surface {
                proxy: proxy,
            }),
            Err(str) => Err(str),
        }
    }
}

impl AsRawPtr<ffi::wayland::WLProxy> for Surface {
    fn as_mut_ptr(&mut self) -> *mut ffi::wayland::WLProxy {
        return self.proxy.as_mut_ptr();
    }
}

impl GetInterface for Surface {
    fn get_interface() -> *const ffi::wayland::WLInterface {
        return &wl_surface_interface as *const ffi::wayland::WLInterface;
    }
}

#[allow(unused_variables)]
extern fn surface_event_dispatcher<T: SurfaceEventHandler>(
        user_data: *mut c_void,
        _target: *mut c_void,
        opcode: uint32_t,
        _message: *const ffi::wayland::WLMessage,
        arguments: *mut ffi::wayland::WLArgument) -> c_int {
    let object = user_data as *mut T;
    return match SurfaceEvent::from_u32(opcode) {
        Some(event) => {
            match event {
                SurfaceEvent::Enter => {
                    let output = unsafe { *(*arguments.offset(0)).object() };
                    unsafe { (*object).on_enter(output); }
                },
                SurfaceEvent::Leave => {
                    let output = unsafe { *(*arguments.offset(0)).object() };
                    unsafe { (*object).on_leave(output); }
                },
            }
            0
        },
        _ => -1,
    }
}

pub trait SurfaceEventHandler: Sized {
    fn connect_dispatcher(&mut self) {
        unsafe {
            ffi::wayland::wl_proxy_add_dispatcher(
                self.get_surface().as_mut_ptr(),
                surface_event_dispatcher::<Self>,
                self as *mut Self as *mut c_void,
                ptr::null_mut());
        }
    }

    fn get_surface(&mut self) -> &mut Surface;
    
    /// This is emitted whenever a surface's creation, movement, or resizing
    /// results in some part of it being within the scanout region of an
    /// output.
    /// 
    /// Note that a surface may be overlapping with zero or more outputs.
    #[allow(unused_variables)]
    fn on_enter(&mut self, output: *mut ffi::wayland::WLObject) {}
    
    /// This is emitted whenever a surface's creation, movement, or resizing
    /// results in it no longer having any part of it within the scanout region
    /// of an output.
    #[allow(unused_variables)]
    fn on_leave(&mut self, output: *mut ffi::wayland::WLObject) {}
}
