// Copyright (c) <2015> <lummax>
// Licensed under MIT (http://opensource.org/licenses/MIT)

// Generated with version 1.7.0

mod display;
mod registry;
mod callback;
mod compositor;
mod shm_pool;
mod shm;
mod buffer;
mod data_offer;
mod data_source;
mod data_device;
mod data_device_manager;
mod shell;
mod shell_surface;
mod surface;
mod seat;
mod pointer;
mod keyboard;
mod touch;
mod output;
mod region;
mod subcompositor;
mod subsurface;

pub use self::display::{ Display, DisplayEventHandler, DisplayError };
pub use self::registry::{ Registry, RegistryEventHandler };
pub use self::callback::{ Callback, CallbackEventHandler };
pub use self::compositor::{ Compositor };
pub use self::shm_pool::{ ShmPool };
pub use self::shm::{ Shm, ShmEventHandler, ShmError, ShmFormat };
pub use self::buffer::{ Buffer, BufferEventHandler };
pub use self::data_offer::{ DataOffer, DataOfferEventHandler };
pub use self::data_source::{ DataSource, DataSourceEventHandler };
pub use self::data_device::{ DataDevice, DataDeviceEventHandler, DataDeviceError };
pub use self::data_device_manager::{ DataDeviceManager };
pub use self::shell::{ Shell, ShellError };
pub use self::shell_surface::{ ShellSurface, ShellSurfaceEventHandler, ShellSurfaceResize, ShellSurfaceTransient, ShellSurfaceFullscreenMethod };
pub use self::surface::{ Surface, SurfaceEventHandler, SurfaceError };
pub use self::seat::{ Seat, SeatEventHandler, SeatCapability };
pub use self::pointer::{ Pointer, PointerEventHandler, PointerError, PointerButtonState, PointerAxis };
pub use self::keyboard::{ Keyboard, KeyboardEventHandler, KeyboardKeymapFormat, KeyboardKeyState };
pub use self::touch::{ Touch, TouchEventHandler };
pub use self::output::{ Output, OutputEventHandler, OutputSubpixel, OutputTransform, OutputMode };
pub use self::region::{ Region };
pub use self::subcompositor::{ Subcompositor, SubcompositorError };
pub use self::subsurface::{ Subsurface, SubsurfaceError };

use ffi;

pub trait FromPrimitive {
    fn from_u32(num: u32) -> Option<Self>;
    fn from_i32(num: i32) -> Option<Self>;
}

pub trait GetInterface {
    fn get_interface() -> *const ffi::wayland::WLInterface;
}
