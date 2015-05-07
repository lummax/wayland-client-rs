// Copyright (c) <2015> <lummax>
// Licensed under MIT (http://opensource.org/licenses/MIT)

use ffi;
use client::base::{FromRawPtr, AsRawPtr};

pub struct EventQueue {
    wl_object: *mut ffi::wayland::WLEventQueue,
}

impl Drop for EventQueue {
    fn drop(&mut self) {
        unsafe {
            ffi::wayland::wl_event_queue_destroy(self.wl_object);
        }
    }
}

impl FromRawPtr<ffi::wayland::WLEventQueue> for EventQueue {
    fn from_mut_ptr(ptr: *mut ffi::wayland::WLEventQueue) -> Result<Self, &'static str> {
        if ptr.is_null() {
            return Err("pointer for EventQueue is null");
        }
        return Ok(EventQueue {
            wl_object: ptr,
        });
    }
}

impl AsRawPtr<ffi::wayland::WLEventQueue> for EventQueue {
    fn as_mut_ptr(&mut self) -> *mut ffi::wayland::WLEventQueue {
        return self.wl_object;
    }
}

