// Copyright (c) <2015> <lummax>
// Licensed under MIT (http://opensource.org/licenses/MIT)

#![feature(libc)]

extern crate wayland_client;
extern crate tempfile;
extern crate libc;

use std::{ptr, cmp};
use std::num::Wrapping;
use std::io::Result;
use std::os::unix::io::AsRawFd;

use tempfile::TempFile;

use wayland_client::client::{Display, Compositor, Shell, Shm, ShmFormat, Surface,
                             Registry, RegistryEventHandler,
                             ShellSurface, ShellSurfaceEventHandler,
                             Buffer, BufferEventHandler,
                             Callback, CallbackEventHandler};

struct MyRegistry {
    registry: Registry,
    compositor: Option<Compositor>,
    shell: Option<Shell>,
    shm: Option<Shm>,
}

impl MyRegistry {
    fn new(display: &mut Display) -> Box<MyRegistry> {
        let registry = display.get_registry().unwrap();
        return Box::new(MyRegistry {
            registry: registry,
            compositor: None,
            shell: None,
            shm: None,
        });
    }

    fn take_compositor(&mut self) -> Compositor {
        return self.compositor.take().unwrap();
    }

    fn take_shell(&mut self) -> Shell {
        return self.shell.take().unwrap();
    }

    fn take_shm(&mut self) -> Shm {
        return self.shm.take().unwrap();
    }
}

impl RegistryEventHandler for MyRegistry {
    fn get_registry(&mut self) -> &mut Registry {
        return &mut self.registry;
    }

    fn on_global(&mut self, name: u32, interface: String, version: u32) {
        if interface == "wl_compositor" {
            self.compositor = self.registry.bind(name, version).ok();
        } else if interface == "wl_shell" {
            self.shell = self.registry.bind(name, version).ok();
        } else if interface == "wl_shm" {
            self.shm = self.registry.bind(name, version).ok();
        }
    }
}

struct MyShellSurface {
    pub shell_surface: ShellSurface,
}

impl MyShellSurface {
    fn new(shell: &mut Shell, surface: &mut Surface) -> Box<MyShellSurface> {
        let mut shell_surface = shell.get_shell_surface(surface).unwrap();
        shell_surface.set_title("simple shm");
        shell_surface.set_toplevel();
        return Box::new(MyShellSurface {
            shell_surface: shell_surface,
        });
    }
}

impl ShellSurfaceEventHandler for MyShellSurface {
    fn get_shell_surface(&mut self) -> &mut ShellSurface {
        return &mut self.shell_surface;
    }

    fn on_ping(&mut self, serial: u32) {
        self.shell_surface.pong(serial);
    }
}

struct MyBuffer {
    buffer: Buffer,
    _file: TempFile,
    memory: *mut libc::c_void,
    size: i32,
    busy: bool,
}

impl MyBuffer {
    fn new(width: i32, height: i32, shm: &mut Shm) -> Result<Box<MyBuffer>> {
        let size = width * height * 4;
        let file = try!(TempFile::new());
        file.set_len(size as u64).unwrap();

        let memory = unsafe {
            libc::mmap(ptr::null_mut(), size as libc::size_t,
                       libc::PROT_WRITE | libc::PROT_READ,
                       libc::MAP_SHARED | libc::MAP_FILE,
                       file.as_raw_fd(), 0)
        };

        if memory == libc::MAP_FAILED {
            panic!("Failed to allocate the memory map");
        }

        let mut pool = shm.create_pool(file.as_raw_fd(), size).unwrap();
        let buffer = pool.create_buffer(0, width, height, width * 4,
                                        ShmFormat::XRGB8888 as u32).unwrap();
        pool.destroy();
        return Ok(Box::new(MyBuffer {
            buffer: buffer,
            _file: file,
            memory: memory,
            size: size,
            busy: false,
        }));
    }

    fn is_not_busy(&self) -> bool {
        return !self.busy;
    }

    fn set_busy(&mut self) {
        self.busy = true;
    }

    fn get_pointer(&mut self) -> *mut u32 {
        return self.memory as *mut u32;
    }
}

impl BufferEventHandler for MyBuffer {
    fn get_buffer(&mut self) -> &mut Buffer {
        return &mut self.buffer;
    }

    fn on_release(&mut self) {
        self.busy = false;
    }
}

impl Drop for MyBuffer {
    fn drop(&mut self) {
        unsafe {
            libc::munmap(self.memory as *mut libc::c_void,
                         self.size as libc::size_t);
        }
    }
}

struct Window {
    width: i32,
    height: i32,
    surface: Surface,
    frame: Option<Callback>,
    first_buffer: Box<MyBuffer>,
    second_buffer: Box<MyBuffer>,
}


impl Window {
    fn new(mut shm: Shm, mut surface: Surface, width: i32, height: i32) -> Window {
        surface.damage(0, 0, width, height);
        let mut first_buffer = MyBuffer::new(width, height, &mut shm).unwrap();
        first_buffer.connect_dispatcher();
        let mut second_buffer = MyBuffer::new(width, height, &mut shm).unwrap();
        second_buffer.connect_dispatcher();
        return Window {
            width: width,
            height: height,
            surface: surface,
            frame: None,
            first_buffer: first_buffer,
            second_buffer: second_buffer,
        };
    }

    fn paint_pixels(&mut self, use_first_buffer: bool, time: i32) {
        let padding = 20;

        let halfh = padding + (self.height - padding * 2) / 2;
        let halfw = padding + (self.width  - padding * 2) / 2;

        let or = (cmp::min(halfw, halfh) - 8).pow(2);
        let ir = (cmp::min(halfw, halfh) - 40).pow(2);

        let mut pointer = if use_first_buffer { self.first_buffer.get_pointer() }
                          else { self.second_buffer.get_pointer() };

        pointer = unsafe { pointer.offset((padding * self.width) as isize)};

        for y in (20 .. self.height - padding) {
            let y2 = (y - halfh) * (y - halfh);

            pointer = unsafe { pointer.offset(padding as isize)};
            for x in (20 .. self.width - 20) {
                let r2 = (x - halfw) * (x - halfw) + y2;

                let v = Wrapping(if r2 < ir { (r2 / 32 + time / 64) }
                                 else if r2 < or { (y + time / 32) }
                                 else { (x + time / 16) } as u32)
                        * Wrapping(0x0080401);
                let mut v2 = v.0 & 0x00ffffff;

                if (x - y).abs() > 6 && (x + y - self.height).abs() > 6 {
                    v2 |= 0xff000000;
                }

                pointer = unsafe { pointer.offset(1)};
                unsafe { *pointer = v2; }
            }
            pointer = unsafe { pointer.offset(padding as isize)};
        }
    }

    fn redraw(&mut self, time: i32) {
        let use_first_buffer = if self.first_buffer.is_not_busy() { true }
                        else if self.second_buffer.is_not_busy() { false }
                        else { panic!("no buffers free") };

        if use_first_buffer { self.first_buffer.set_busy(); }
        else { self.second_buffer.set_busy(); }

        self.paint_pixels(use_first_buffer, time);

        if use_first_buffer { self.surface.attach(Some(self.first_buffer.get_buffer()), 0, 0); }
        else { self.surface.attach(Some(self.second_buffer.get_buffer()), 0, 0); }

        self.surface.damage(20, 20, self.width - 20, self.height - 20);
        self.frame = Some(self.surface.frame().unwrap());
        self.connect_dispatcher();
        self.surface.commit();
    }
}

impl CallbackEventHandler for Window {
    fn get_callback<'a>(&'a mut self) -> &'a mut Callback {
        return self.frame.as_mut().unwrap();
    }

    fn on_done(&mut self, callback_data: u32) {
        self.frame = None;
        self.redraw(callback_data as i32);
    }
}

fn main() {
    let mut display = Display::connect(None).unwrap();
    let mut registry = MyRegistry::new(&mut display);
    registry.connect_dispatcher();
    display.roundtrip();

    let shm = registry.take_shm();
    let mut shell = registry.take_shell();
    let mut compositor = registry.take_compositor();

    let mut surface = compositor.create_surface().unwrap();
    let mut shell_surface = MyShellSurface::new(&mut shell, &mut surface);
    shell_surface.connect_dispatcher();

    let mut window = Window::new(shm, surface, 250, 250);
    window.redraw(0);

    loop { display.dispatch(); }
}
