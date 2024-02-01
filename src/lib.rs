#![deny(clippy::all)]
#![allow(non_snake_case)]

#[macro_use]
extern crate napi_derive;

use autopilot::{geometry, mouse};

#[napi]
pub fn SetMousePosition(x: f64, y: f64) {
    mouse::move_to(geometry::Point::new(x, y)).unwrap();
}
