// Copyright (c) <2015> <lummax>
// Licensed under MIT (http://opensource.org/licenses/MIT)

#![allow(dead_code)]

use libc::{c_int, c_char, c_void, int32_t, uint32_t};

#[repr(C)]
pub struct WLInterface {
    pub name: *const c_char,
    pub version: c_int,
    pub method_count: c_int,
    pub methods: *const WLMessage,
    pub event_count: c_int,
    pub events: *const WLMessage,
}

#[repr(C)]
pub struct WLMessage;

#[repr(C)]
pub struct WLArray;

#[repr(C)]
pub struct WLProxy;

#[repr(C)]
pub struct WLDisplay;

#[repr(C)]
pub struct WLEventQueue;

#[repr(C)]
pub struct WLObject;

#[repr(C)]
pub struct WLArgument {
    data: u64,
}

impl WLArgument {
    pub fn int(&mut self) -> *mut int32_t {
        unsafe { ::std::mem::transmute(self) }
    }

    pub fn uint(&mut self) -> *mut uint32_t {
        unsafe { ::std::mem::transmute(self) }
    }

    pub fn fixed_point(&mut self) -> *mut int32_t {
        unsafe { ::std::mem::transmute(self) }
    }

    pub fn string(&mut self) -> *mut *const ::libc::c_char {
        unsafe { ::std::mem::transmute(self) }
    }

    pub fn object(&mut self) -> *mut *mut WLObject {
        unsafe { ::std::mem::transmute(self) }
    }

    pub fn new_id(&mut self) -> *mut *mut WLProxy {
        unsafe { ::std::mem::transmute(self) }
    }

    pub fn array(&mut self) -> *mut *mut WLArray {
        unsafe { ::std::mem::transmute(self) }
    }

    pub fn file_descriptor(&mut self) -> *mut int32_t {
        unsafe { ::std::mem::transmute(self) }
    }
}

#[repr(C)]
pub type wl_dispatcher_func_t = extern fn(*mut c_void, *mut c_void,
                                          uint32_t, *const WLMessage,
                                          *mut WLArgument) -> c_int;

#[repr(C)]
pub type wl_log_func_t = extern fn(_: *const c_char, ...);

#[link(name = "wayland-client")]
extern {
    pub fn wl_event_queue_destroy(queue: *mut WLEventQueue);

    pub fn wl_proxy_marshal(proxy: *mut WLProxy, opcode: uint32_t, ...);
    pub fn wl_proxy_marshal_array(proxy: *mut WLProxy, opcode: uint32_t,
                                  arguments: *mut WLArgument);
    pub fn wl_proxy_create(factory: *mut WLProxy,
                           interface: *mut WLInterface) -> *mut WLProxy;
    pub fn wl_proxy_marshal_constructor(proxy: *mut WLProxy,
                                        opcode: uint32_t,
                                        interface: *const WLInterface,
                                        ...) -> *mut WLProxy;
    pub fn wl_proxy_marshal_array_constructor(proxy: *mut WLProxy,
                                              opcode: uint32_t,
                                              arguments: *mut WLArgument,
                                              interface: *const WLInterface) -> *mut WLProxy;

    pub fn wl_proxy_destroy(proxy: *mut WLProxy);
    pub fn wl_proxy_add_listener(proxy: *mut WLProxy,
                                 implementation: *mut extern fn(),
                                 data: *mut c_void) -> c_int;
    pub fn wl_proxy_get_listener(proxy: *mut WLProxy) -> *const c_void;
    pub fn wl_proxy_add_dispatcher(proxy: *mut WLProxy,
                                   dispatcher_func: wl_dispatcher_func_t,
                                   dispatcher_data: *mut c_void,
                                   data: *mut c_void) -> c_int;
    pub fn wl_proxy_set_user_data(proxy: *mut WLProxy, user_data: *mut c_void);
    pub fn wl_proxy_get_user_data(proxy: *mut WLProxy) -> *mut c_void;
    pub fn wl_proxy_get_id(proxy: *mut WLProxy) -> uint32_t;
    pub fn wl_proxy_get_class(proxy: *mut WLProxy) -> *const c_char;
    pub fn wl_proxy_set_queue(proxy: *mut WLProxy, queue: *mut WLEventQueue);

    pub fn wl_display_connect(name: *const c_char) -> *mut WLDisplay;
    pub fn wl_display_connect_to_fd(fd: c_int) -> *mut WLDisplay;
    pub fn wl_display_disconnect(display: *mut WLDisplay);
    pub fn wl_display_get_fd(display: *mut WLDisplay) -> c_int;
    pub fn wl_display_dispatch(display: *mut WLDisplay) -> c_int;
    pub fn wl_display_dispatch_queue(display: *mut WLDisplay,
                                     queue: *mut WLEventQueue) -> c_int;
    pub fn wl_display_dispatch_queue_pending(display: *mut WLDisplay,
                                             queue: *mut WLEventQueue) -> c_int;
    pub fn wl_display_dispatch_pending(display: *mut WLDisplay) -> c_int;
    pub fn wl_display_get_error(display: *mut WLDisplay) -> c_int;
    pub fn wl_display_get_protocol_error(display: *mut WLDisplay,
                                         interface: *mut *mut WLInterface,
                                         id: *mut uint32_t) -> uint32_t;

    pub fn wl_display_flush(display: *mut WLDisplay) -> c_int;
    pub fn wl_display_roundtrip_queue(display: *mut WLDisplay,
                                      queue: *mut WLEventQueue) -> c_int;
    pub fn wl_display_roundtrip(display: *mut WLDisplay) -> c_int;
    pub fn wl_display_create_queue(display: *mut WLDisplay) -> *mut WLEventQueue;

    pub fn wl_display_prepare_read_queue(display: *mut WLDisplay,
                                         queue: *mut WLEventQueue) -> c_int;
    pub fn wl_display_prepare_read(display: *mut WLDisplay) -> c_int;
    pub fn wl_display_cancel_read(display: *mut WLDisplay);
    pub fn wl_display_read_events(display: *mut WLDisplay) -> c_int;

    pub fn wl_log_set_handler_client(handler: wl_log_func_t);
}
