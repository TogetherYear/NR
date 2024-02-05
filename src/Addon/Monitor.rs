use super::Automatic::GetMousePosition;
use super::Image;

#[napi(js_name = "GetAllMonitors")]
pub fn GetAllMonitors() -> Vec<Monitor> {
    let monitors = xcap::Monitor::all().unwrap();
    let mut ms: Vec<Monitor> = vec![];
    for monitor in monitors.iter() {
        ms.push(Monitor::New(monitor));
    }
    ms
}

#[napi(js_name = "GetMonitorFromPoint")]
pub fn GetMonitorFromPoint(x: i32, y: i32) -> Monitor {
    Monitor::New(&xcap::Monitor::from_point(x, y).unwrap())
}

#[napi(js_name = "GetCurrentMouseMonitor")]
pub fn GetCurrentMouseMonitor() -> Monitor {
    let point = GetMousePosition();
    Monitor::New(&xcap::Monitor::from_point(point.x as i32, point.y as i32).unwrap())
}

#[napi(js_name = "GetPrimaryMonitor")]
pub fn GetPrimaryMonitor() -> Monitor {
    let monitors = xcap::Monitor::all().unwrap();
    for m in monitors.iter() {
        if m.is_primary() {
            return Monitor::New(m);
        }
    }
    Monitor::Default()
}

#[napi(constructor)]
pub struct Monitor {
    pub id: u32,
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub rotation: f64,
    pub scaleFactor: f64,
    pub frequency: f64,
    pub isPrimary: bool,
}

impl Monitor {
    pub fn New(m: &xcap::Monitor) -> Monitor {
        Monitor {
            id: m.id(),
            name: m.name().to_string(),
            x: m.x(),
            y: m.y(),
            width: m.width(),
            height: m.height(),
            rotation: m.rotation() as f64,
            scaleFactor: m.scale_factor() as f64,
            frequency: m.frequency() as f64,
            isPrimary: m.is_primary(),
        }
    }

    pub fn Default() -> Monitor {
        Monitor {
            id: 0,
            name: String::from(""),
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            rotation: 0.0,
            scaleFactor: 0.0,
            frequency: 0.0,
            isPrimary: false,
        }
    }
}

#[napi]
impl Monitor {
    #[napi(js_name = "Capture")]
    pub fn Capture(&self, path: String, format: Image::ImageFormat) -> bool {
        let monitors = xcap::Monitor::all().unwrap();
        for m in monitors.iter() {
            if m.id() == self.id {
                m.capture_image()
                    .unwrap()
                    .save_with_format(path, Image::TransformFormat(format))
                    .unwrap();
                return true;
            }
        }
        false
    }
}
