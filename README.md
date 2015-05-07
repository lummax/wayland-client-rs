wayland-client-rs
=================

This is an attempt at convenient rust bindings to the wayland-client library.

You'll need [Wayland](http://wayland.freedesktop.org/),
[Rust](http://rust-lang.org/) and [cargo](http://crates.io) installed.

Status
------

This might not be usable at the moment. Major `TODO`s are:

- testing and documentation
- get `WLArray` to work
- get `WWLObject` to work
- respect the `allow-null` field in the protocol specification
- error handling on wayland-ffi calls

Using
-----

Start up a weston instance and compile one of the examples (e.g. `cargo build
--example simple-shm`). These
[examples](https://github.com/lummax/wayland-client-rs/tree/master/examples)
might be a good starting point to dive into the code.
