// Copyright (c) <2015> <lummax>
// Licensed under MIT (http://opensource.org/licenses/MIT)

extern crate wayland_client;
use wayland_client::client::{FromPrimitive, Display,
                             Registry, RegistryEventHandler,
                             Seat, SeatEventHandler, SeatCapability,
                             Shm, ShmEventHandler, ShmFormat,
                             Output, OutputEventHandler, OutputSubpixel,
                             OutputTransform, OutputMode};

use std::collections::HashMap;

#[derive(Default)]
struct SeatData {
    name: String,
    capabilities: Vec<String>,
}


#[derive(Default)]
struct ShmData {
    formats: Vec<String>,
}

#[derive(Default)]
struct OutputData {
    x: i32,
    y: i32,
    physical_width: i32,
    physical_height: i32,
    subpixel: String,
    make: String,
    model: String,
    transform: String,
    modes: Vec<OutputModeData>,
}

struct OutputModeData {
    width: i32,
    height: i32,
    refresh: i32,
    flags: String,
}

struct Info {
    registry: Registry,
    seat: Option<Seat>,
    shm: Option<Shm>,
    output: Option<Output>,
    pub roundtrip: bool,
    data: HashMap<String, String>,
    seat_data: SeatData,
    shm_data: ShmData,
    output_data: OutputData,
}

impl Info {
    fn new(registry: Registry) -> Info {
        return Info {
            registry: registry,
            seat: None,
            shm: None,
            output: None,
            roundtrip: true,
            data: HashMap::new(),
            seat_data: Default::default(),
            shm_data: Default::default(),
            output_data: Default::default(),
        };
    }
}

impl RegistryEventHandler for Info {
    fn get_registry(&mut self) -> &mut Registry {
        return &mut self.registry;
    }

    fn on_global(&mut self, name: u32, interface: String, version: u32) {
        self.data.insert(interface.clone(),
                         format!("interface: '{}', version: {}, name: {}",
                                 interface, version, name));

        if interface == "wl_seat" {
            self.roundtrip = true;
            self.seat = self.registry.bind(name, version).ok();
            SeatEventHandler::connect_dispatcher(self);
        } else if interface == "wl_shm" {
            self.roundtrip = true;
            self.shm = self.registry.bind(name, version).ok();
            ShmEventHandler::connect_dispatcher(self);
        } else if interface == "wl_output" {
            self.roundtrip = true;
            self.output = self.registry.bind(name, version).ok();
            OutputEventHandler::connect_dispatcher(self);
        }
    }
}

impl SeatEventHandler for Info {
    fn get_seat(&mut self) -> &mut Seat {
        return self.seat.as_mut().unwrap();
    }

    fn on_capabilities(&mut self, capabilities: u32) {
        if capabilities & (SeatCapability::Pointer as u32) != 0 {
            self.seat_data.capabilities.push("pointer".to_string());
        }
        if capabilities & (SeatCapability::Keyboard as u32) != 0 {
            self.seat_data.capabilities.push("keyboard".to_string());
        }
        if capabilities & (SeatCapability::Touch as u32) != 0 {
            self.seat_data.capabilities.push("touch".to_string());
        }
    }

    fn on_name(&mut self, name: String) {
        self.seat_data.name = name;
    }
}

impl ShmEventHandler for Info {
    fn get_shm(&mut self) -> &mut Shm {
        return self.shm.as_mut().unwrap();
    }

    fn on_format(&mut self, format: u32) {
        self.shm_data.formats.push(format!("{:?}", ShmFormat::from_u32(format).unwrap()));
    }
}

impl OutputEventHandler for Info {
    fn get_output(&mut self) -> &mut Output {
        return self.output.as_mut().unwrap();
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
    let mut info = Info::new(display.get_registry().unwrap());
    RegistryEventHandler::connect_dispatcher(&mut info);

    while info.roundtrip {
        info.roundtrip = false;
        display.roundtrip();
    }
}
