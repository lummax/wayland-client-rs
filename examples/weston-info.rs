// Copyright (c) <2015> <lummax>
// Licensed under MIT (http://opensource.org/licenses/MIT)

extern crate wayland_client;
use wayland_client::client::{FromPrimitive, Display,
                             Registry, RegistryEventHandler,
                             Seat, SeatEventHandler, SeatCapability,
                             Shm, ShmEventHandler, ShmFormat,
                             Output, OutputEventHandler, OutputSubpixel,
                             OutputTransform, OutputMode};

#[derive(Debug)]
struct MyRegistry {
    registry: Registry,
    seat: Option<MySeat>,
    shm: Option<MyShm>,
    output: Option<MyOutput>,
    pub roundtrip: bool,
}

impl MyRegistry {
    fn new(registry: Registry) -> MyRegistry {
        return MyRegistry {
            registry: registry,
            seat: None,
            shm: None,
            output: None,
            roundtrip: true,
        };
    }
}

impl RegistryEventHandler for MyRegistry {
    fn get_registry(&mut self) -> &mut Registry {
        return &mut self.registry;
    }

    fn on_global(&mut self, name: u32, interface: String, version: u32) {
        println!("interface: '{}', version: {}, name: {}", interface, version, name);
        if interface == "wl_seat" {
            self.roundtrip = true;
            let mut seat = MySeat::new(self.registry.bind(name, version).unwrap());
            seat.connect_dispatcher();
            self.seat = Some(seat);
        } else if interface == "wl_shm" {
            self.roundtrip = true;
            let mut shm = MyShm::new(self.registry.bind(name, version).unwrap());
            shm.connect_dispatcher();
            self.shm = Some(shm);
        } else if interface == "wl_output" {
            self.roundtrip = true;
            let mut output = MyOutput::new(self.registry.bind(name, version).unwrap());
            output.connect_dispatcher();
            self.output = Some(output);
        }
    }

    fn on_global_remove(&mut self, name: u32) {
        println!("on_global_remove({})", name);
    }
}

#[derive(Debug)]
struct MySeat {
    seat: Seat,
}

impl MySeat {
    fn new(seat: Seat) -> MySeat {
        return MySeat { seat: seat };
    }
}

impl SeatEventHandler for MySeat {
    fn get_seat(&mut self) -> &mut Seat {
        return &mut self.seat;
    }

    fn on_capabilities(&mut self, capabilities: u32) {
        println!("on_capabilities({})", capabilities);
    }

    fn on_name(&mut self, name: String) {
        println!("on_name({})", name);
    }
}

#[derive(Debug)]
struct MyShm {
    shm: Shm,
}

impl MyShm {
    fn new(shm: Shm) -> MyShm {
        return MyShm { shm: shm };
    }
}

impl ShmEventHandler for MyShm {
    fn get_shm(&mut self) -> &mut Shm {
        return &mut self.shm;
    }

    fn on_format(&mut self, format: u32) {
        println!("on_format({:?})", ShmFormat::from_u32(format).unwrap());
    }
}

#[derive(Debug)]
struct MyOutput {
    output: Output,
}

impl MyOutput {
    fn new(output: Output) -> MyOutput {
        return MyOutput { output: output };
    }
}

impl OutputEventHandler for MyOutput {
    fn get_output(&mut self) -> &mut Output {
        return &mut self.output;
    }

    fn on_geometry(&mut self, x: i32, y: i32, physical_width: i32,
                   physical_height: i32, subpixel: i32, make: String, model:
                   String, transform: i32) {
        println!("on_geometry({}, {}, {}, {}, {:?}, {}, {}, {:?})", x, y,
            physical_width, physical_height,
            OutputSubpixel::from_u32(subpixel as u32).unwrap(), make, model,
            OutputTransform::from_u32(transform as u32).unwrap());
    }

    fn on_mode(&mut self, flags: u32, width: i32, height: i32, refresh: i32) {
        println!("on_mode({}, {}, {}, {})", flags, width, height, refresh);
    }

    fn on_done(&mut self) {
        println!("on_done()");
    }

    fn on_scale(&mut self, factor: i32) {
        println!("on_scale({})", factor);
    }
}

fn main() {
    let mut display = Display::connect(None).unwrap();
    let mut registry = MyRegistry::new(display.get_registry().unwrap());
    registry.connect_dispatcher();

    while registry.roundtrip {
        registry.roundtrip = false;
        display.roundtrip();
    }
}
