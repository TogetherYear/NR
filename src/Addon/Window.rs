use super::Image;

#[napi(js_name = "GetAllWindows")]
pub fn GetAllWindows() -> Vec<Window> {
    let windows = xcap::Window::all().unwrap();
    let mut ws: Vec<Window> = vec![];
    for window in windows.iter() {
        ws.push(Window::New(window));
    }
    ws
}

#[napi(constructor)]
pub struct Window {
    pub id: u32,
    pub title: String,
    pub appName: String,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub isMinimized: bool,
    pub isMaximized: bool,
}

impl Window {
    pub fn New(w: &xcap::Window) -> Window {
        Window {
            id: w.id(),
            title: w.title().to_string(),
            appName: w.app_name().to_string(),
            x: w.x(),
            y: w.y(),
            width: w.width(),
            height: w.height(),
            isMinimized: w.is_minimized(),
            isMaximized: w.is_maximized(),
        }
    }
}

#[napi]
impl Window {
    #[napi(js_name = "Capture")]
    pub fn Capture(&self, path: String, format: Image::ImageFormat) -> bool {
        let windows = xcap::Window::all().unwrap();
        for w in windows.iter() {
            if w.id() == self.id && !w.is_minimized() {
                w.capture_image()
                    .unwrap()
                    .save_with_format(path, Image::TransformFormat(format))
                    .unwrap();
                return true;
            }
        }
        false
    }
}
