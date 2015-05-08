// Copyright (c) <2015> <lummax>
// Licensed under MIT (http://opensource.org/licenses/MIT)

use std::ptr;
use std::ffi::{CStr, CString};
use std::os::unix::io::RawFd;

use ffi;
use client::base::{FromRawPtr, AsRawPtr, EventQueue};

#[derive(Debug)]
#[allow(raw_pointer_derive)]
pub struct Display {
    wl_object: *mut ffi::wayland::WLDisplay,
}

impl Display {
    pub fn connect(name: Option<&str>) -> Result<Self, &'static str> {
        let name_ptr = match name {
            Some(string) => CString::new(string).unwrap().as_ptr(),
            None => ptr::null(),
        };
        let display = unsafe {
            ffi::wayland::wl_display_connect(name_ptr)
        };
        if display.is_null() {
            return Err("error on connect");
        }
        return Ok(Display {
            wl_object: display,
        });
    }

    pub fn connect_to_fd(fd: RawFd) -> Result<Self, &'static str> {
        let display = unsafe { ffi::wayland::wl_display_connect_to_fd(fd) };
        if display.is_null() {
            return Err("error on connect");
        }
        return Ok(Display {
            wl_object: display,
        });
    }

    pub fn get_id(&mut self) -> u32 {
        return unsafe {
            ffi::wayland::wl_proxy_get_id(self.wl_object
                                          as *mut ffi::wayland::WLProxy)
        };
    }

    pub fn get_class(&mut self) -> String {
        let string_ptr =  unsafe {
            ffi::wayland::wl_proxy_get_class(self.wl_object
                                             as *mut ffi::wayland::WLProxy)
        };
        let buffer = unsafe { CStr::from_ptr(string_ptr).to_bytes() };
        return String::from_utf8(buffer.to_vec()).unwrap();
    }

    pub fn set_queue(&mut self, queue: &mut EventQueue) {
        unsafe {
            ffi::wayland::wl_proxy_set_queue(self.wl_object
                                             as *mut ffi::wayland::WLProxy,
                                             queue.as_mut_ptr());
        }
    }

    pub fn get_fd(&mut self) -> RawFd {
        return unsafe { ffi::wayland::wl_display_get_fd(self.wl_object) };
    }

    pub fn dispatch(&mut self) -> Option<u32> {
        let len = unsafe { ffi::wayland::wl_display_dispatch(self.wl_object) };
        if len < 0 {
            return None;
        }
        return Some(len as u32);
    }

    pub fn dispatch_queue(&mut self, queue: &mut EventQueue) -> Option<u32> {
        let len = unsafe {
            ffi::wayland::wl_display_dispatch_queue(self.wl_object,
                                                    queue.as_mut_ptr())
        };
        if len < 0 {
            return None;
        }
        return Some(len as u32);
    }

    pub fn dispatch_queue_pending(&mut self, queue: &mut EventQueue) -> Option<u32> {
        let len = unsafe {
            ffi::wayland::wl_display_dispatch_queue_pending(self.wl_object,
                                                            queue.as_mut_ptr())
        };
        if len < 0 {
            return None;
        }
        return Some(len as u32);
    }

    pub fn dispatch_pending(&mut self) -> Option<u32> {
        let len = unsafe {
            ffi::wayland::wl_display_dispatch_pending(self.wl_object)
        };
        if len < 0 {
            return None;
        }
        return Some(len as u32);
    }

    pub fn get_error(&mut self) -> i32 {
        return unsafe {
            ffi::wayland::wl_display_get_error(self.wl_object)
        };
    }

    // XXX
    //pub fn get_protocol_error(&mut self, interface: *mut *mut
    //                      WLInterface, id: *mut uint32_t) -> uint32_t {
    //    ffi::wayland::wl_display_get_protocol_error(self.wl_object,
    //                                                interface: *mut *mut
    //                                                WLInterface, id: *mut
    //                                                uint32_t) -> uint32_t;
    //}

    pub fn flush(&mut self) -> Option<u32> {
        let len = unsafe { ffi::wayland::wl_display_flush(self.wl_object) };
        if len < 0 {
            return None;
        }
        return Some(len as u32);
    }

    pub fn roundtrip_queue(&mut self, queue: &mut EventQueue) -> Option<u32> {
        let len = unsafe {
            ffi::wayland::wl_display_roundtrip_queue(self.wl_object,
                                                     queue.as_mut_ptr())
        };
        if len < 0 {
            return None;
        }
        return Some(len as u32);
    }

    pub fn roundtrip(&mut self) -> Option<u32> {
        let len = unsafe {
            ffi::wayland::wl_display_roundtrip(self.wl_object)
        };
        if len < 0 {
            return None;
        }
        return Some(len as u32);
    }

    pub fn create_queue(&mut self) -> Result<EventQueue, &'static str> {
        let queue = unsafe {
            ffi::wayland::wl_display_create_queue(self.wl_object)
        };
        return EventQueue::from_mut_ptr(queue);
    }

    pub fn prepare_read_queue(&mut self, queue: &mut EventQueue) -> bool {
        return unsafe {
            ffi::wayland::wl_display_prepare_read_queue(self.wl_object,
                                                        queue.as_mut_ptr())
        } == 0;
    }

    pub fn prepare_read(&mut self) -> bool {
        return unsafe {
            ffi::wayland::wl_display_prepare_read(self.wl_object)
        } == 0;
    }

    pub fn cancel_read(&mut self) {
        unsafe {
            ffi::wayland::wl_display_cancel_read(self.wl_object);
        }
    }

    pub fn read_events(&mut self) -> bool {
        // XXX errno
        return unsafe {
            ffi::wayland::wl_display_read_events(self.wl_object)
        } == 0;
    }
}

impl Drop for Display {
    fn drop(&mut self) {
        unsafe {
            ffi::wayland::wl_display_disconnect(self.wl_object);
        }
    }
}

impl FromRawPtr<ffi::wayland::WLDisplay> for Display {
    fn from_mut_ptr(ptr: *mut ffi::wayland::WLDisplay) -> Result<Self, &'static str> {
        if ptr.is_null() {
            return Err("pointer for Display is null");
        }
        return Ok(Display {
            wl_object: ptr,
        });
    }
}

impl AsRawPtr<ffi::wayland::WLDisplay> for Display {
    fn as_mut_ptr(&mut self) -> *mut ffi::wayland::WLDisplay {
        return self.wl_object;
    }
}
