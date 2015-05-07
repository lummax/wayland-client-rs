// Copyright (c) <2015> <lummax>
// Licensed under MIT (http://opensource.org/licenses/MIT)

mod event_queue;
mod proxy;
mod display;

pub use self::event_queue::EventQueue;
pub use self::proxy::Proxy;
pub use self::display::Display;

pub trait FromRawPtr<T> {
    fn from_mut_ptr(ptr: *mut T) -> Result<Self, &'static str>;
}

pub trait AsRawPtr<T> {
    fn as_mut_ptr(&mut self) -> *mut T;
}
