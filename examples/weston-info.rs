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

    fn print_it(&self) {
        for (key, val) in self.data.iter() {
            println!("{}", val);
            match key.as_ref() {
                "wl_shm" => {
                    println!("\tformats: {}",
                             self.shm_data.formats.connect(" "));
                },
                "wl_seat" => {
                    println!("\tname: {}", self.seat_data.name);
                    println!("\tcapabilities: {}",
                             self.seat_data.capabilities.connect(" "));
                },
                "wl_output" => {
                    println!("\tx: {}, y: {},", self.output_data.x,
                             self.output_data.y);
                    println!("\tphysical_width: {} mm, physical_height: {} mm,",
                             self.output_data.physical_width,
                             self.output_data.physical_height);
                    println!("\tmake: '{}', model: '{}',",
                             self.output_data.make, self.output_data.model);
                    println!("\tsubpixel_orientation: '{}', transformation: '{}',",
                             self.output_data.subpixel,
                             self.output_data.transform);
                    for mode in self.output_data.modes.iter() {
                        println!("\tmode:");
                        println!("\t\twidth: {} px, height: {} px, refresh: {} Hz,",
                                 mode.width, mode.height, mode.refresh / 1000);
                        println!("\t\tflags: {}", mode.flags);
                    }
                }
                _ => (),
            }
        }
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
        if capabilities & (SeatCapability::POINTER as u32) != 0 {
            self.seat_data.capabilities.push("pointer".to_string());
        }
        if capabilities & (SeatCapability::KEYBOARD as u32) != 0 {
            self.seat_data.capabilities.push("keyboard".to_string());
        }
        if capabilities & (SeatCapability::TOUCH as u32) != 0 {
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
        self.output_data.x = x;
        self.output_data.y = y;
        self.output_data.physical_width = physical_width;
        self.output_data.physical_height = physical_height;
        self.output_data.subpixel = format!("{:?}", OutputSubpixel::from_i32(subpixel).unwrap());
        self.output_data.make = make;
        self.output_data.model = model;
        self.output_data.transform = format!("{:?}", OutputTransform::from_i32(transform).unwrap());
    }

    fn on_mode(&mut self, flags: u32, width: i32, height: i32, refresh: i32) {
        let mut flags_ = Vec::new();
        if flags & (OutputMode::CURRENT as u32) != 0 {
            flags_.push("current".to_string());
        }
        if flags & (OutputMode::PREFERRED as u32) != 0 {
            flags_.push("preferred".to_string());
        }
        self.output_data.modes.push(OutputModeData {
            width: width,
            height: height,
            refresh: refresh,
            flags: flags_.connect(" "),
        })
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
    info.print_it();
}
