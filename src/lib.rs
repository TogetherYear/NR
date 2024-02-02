#![deny(clippy::all)]
#![allow(non_snake_case)]

#[macro_use]
extern crate napi_derive;

use autopilot::{geometry, mouse};

use enigo::{Enigo, KeyboardControllable};

#[napi]
pub fn GetMousePosition() -> Point {
    let t = mouse::location();
    Point::New(t.x, t.y)
}

#[napi]
pub fn SetMousePosition(x: f64, y: f64) {
    mouse::move_to(geometry::Point::new(x, y)).unwrap();
}

#[napi]
pub fn SetButtonClick(button: MosueButton, delay: u32) {
    match button {
        MosueButton::Left => mouse::click(mouse::Button::Left, Some(delay as u64)),
        MosueButton::Middle => mouse::click(mouse::Button::Middle, Some(delay as u64)),
        MosueButton::Right => mouse::click(mouse::Button::Right, Some(delay as u64)),
    }
}

#[napi]
pub fn SetButtonToggle(button: MosueButton, down: bool) {
    match button {
        MosueButton::Left => mouse::toggle(mouse::Button::Left, down),
        MosueButton::Middle => mouse::toggle(mouse::Button::Middle, down),
        MosueButton::Right => mouse::toggle(mouse::Button::Right, down),
    }
}

#[napi]
pub fn SetMouseScroll(direction: ScrollDirection, clicks: u32) {
    match direction {
        ScrollDirection::Down => mouse::scroll(mouse::ScrollDirection::Down, clicks),
        ScrollDirection::Up => mouse::scroll(mouse::ScrollDirection::Up, clicks),
    }
}

#[napi]
pub fn GetColorFromPosition(x: f64, y: f64) -> Color {
    let monitor = xcap::Monitor::from_point(x as i32, y as i32).unwrap();
    let capture = monitor.capture_image().unwrap();
    let pixel = capture.get_pixel_checked(
        ((x as i32) - monitor.x()) as u32,
        ((y as i32) - monitor.y()) as u32,
    );
    match pixel {
        Some(color) => Color::New(color.0[0], color.0[1], color.0[2], color.0[3]),
        None => Color::Default(),
    }
}

#[napi]
pub fn GetCurrentPositionColor() -> Color {
    let point = mouse::location();
    GetColorFromPosition(point.x, point.y)
}

#[napi]
pub fn WriteText(content: String) {
    let mut e = Enigo::new();
    e.key_sequence(content.as_str());
}

#[napi]
pub fn SetKeysToggle(keys: Vec<ToggleKey>) {
    let mut e = Enigo::new();
    for key in keys {
        match key.down {
            true => e.key_down(TransformKey(key.value)),
            false => e.key_up(TransformKey(key.value)),
        }
    }
}

#[napi]
pub fn SetKeysClick(keys: Vec<KeyboardKey>) {
    let mut e = Enigo::new();
    for key in keys {
        e.key_click(TransformKey(key));
    }
}

#[napi]
pub enum ScrollDirection {
    Down = 0,
    Up = 1,
}

#[napi]
pub enum MosueButton {
    Left = 0,
    Middle = 1,
    Right = 2,
}

#[napi]
pub enum KeyboardKey {
    Num0 = 0,
    Num1 = 1,
    Num2 = 2,
    Num3 = 3,
    Num4 = 4,
    Num5 = 5,
    Num6 = 6,
    Num7 = 7,
    Num8 = 8,
    Num9 = 9,
    A = 10,
    B = 11,
    C = 12,
    D = 13,
    E = 14,
    F = 15,
    G = 16,
    H = 17,
    I = 18,
    J = 19,
    K = 20,
    L = 21,
    M = 22,
    N = 23,
    O = 24,
    P = 25,
    Q = 26,
    R = 27,
    S = 28,
    T = 29,
    U = 30,
    V = 31,
    W = 32,
    X = 33,
    Y = 34,
    Z = 35,
    Add = 36,
    Subtract = 37,
    Multiply = 38,
    Divide = 39,
    OEM2 = 40,
    Tab = 41,
    CapsLock = 42,
    Shift = 43,
    Control = 44,
    Alt = 45,
    Space = 46,
    Backspace = 47,
    Return = 48,
    Escape = 49,
    UpArrow = 50,
    DownArrow = 51,
    LeftArrow = 52,
    RightArrow = 53,
}

fn TransformKey(key: KeyboardKey) -> enigo::Key {
    match key {
        KeyboardKey::Num0 => enigo::Key::Num0,
        KeyboardKey::Num1 => enigo::Key::Num1,
        KeyboardKey::Num2 => enigo::Key::Num2,
        KeyboardKey::Num3 => enigo::Key::Num3,
        KeyboardKey::Num4 => enigo::Key::Num4,
        KeyboardKey::Num5 => enigo::Key::Num5,
        KeyboardKey::Num6 => enigo::Key::Num6,
        KeyboardKey::Num7 => enigo::Key::Num7,
        KeyboardKey::Num8 => enigo::Key::Num8,
        KeyboardKey::Num9 => enigo::Key::Num9,
        KeyboardKey::A => enigo::Key::A,
        KeyboardKey::B => enigo::Key::B,
        KeyboardKey::C => enigo::Key::C,
        KeyboardKey::D => enigo::Key::D,
        KeyboardKey::E => enigo::Key::E,
        KeyboardKey::F => enigo::Key::F,
        KeyboardKey::G => enigo::Key::G,
        KeyboardKey::H => enigo::Key::H,
        KeyboardKey::I => enigo::Key::I,
        KeyboardKey::J => enigo::Key::J,
        KeyboardKey::K => enigo::Key::K,
        KeyboardKey::L => enigo::Key::L,
        KeyboardKey::M => enigo::Key::M,
        KeyboardKey::N => enigo::Key::N,
        KeyboardKey::O => enigo::Key::O,
        KeyboardKey::P => enigo::Key::P,
        KeyboardKey::Q => enigo::Key::Q,
        KeyboardKey::R => enigo::Key::R,
        KeyboardKey::S => enigo::Key::S,
        KeyboardKey::T => enigo::Key::T,
        KeyboardKey::U => enigo::Key::U,
        KeyboardKey::V => enigo::Key::V,
        KeyboardKey::W => enigo::Key::W,
        KeyboardKey::X => enigo::Key::X,
        KeyboardKey::Y => enigo::Key::Y,
        KeyboardKey::Z => enigo::Key::Z,
        KeyboardKey::Add => enigo::Key::Add,
        KeyboardKey::Subtract => enigo::Key::Subtract,
        KeyboardKey::Multiply => enigo::Key::Multiply,
        KeyboardKey::Divide => enigo::Key::Divide,
        KeyboardKey::OEM2 => enigo::Key::OEM2,
        KeyboardKey::Tab => enigo::Key::Tab,
        KeyboardKey::CapsLock => enigo::Key::CapsLock,
        KeyboardKey::Shift => enigo::Key::Shift,
        KeyboardKey::Control => enigo::Key::Control,
        KeyboardKey::Alt => enigo::Key::Alt,
        KeyboardKey::Space => enigo::Key::Space,
        KeyboardKey::Backspace => enigo::Key::Backspace,
        KeyboardKey::Return => enigo::Key::Return,
        KeyboardKey::Escape => enigo::Key::Escape,
        KeyboardKey::UpArrow => enigo::Key::UpArrow,
        KeyboardKey::DownArrow => enigo::Key::DownArrow,
        KeyboardKey::LeftArrow => enigo::Key::LeftArrow,
        KeyboardKey::RightArrow => enigo::Key::RightArrow,
    }
}

#[napi(object)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn New(x: f64, y: f64) -> Point {
        Point { x, y }
    }
}

#[napi(object)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn New(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }

    pub fn Default() -> Color {
        Color {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        }
    }
}

#[napi(object)]
pub struct ToggleKey {
    pub value: KeyboardKey,
    pub down: bool,
}
