#[napi(js_name = "GetWallpaper")]
pub fn GetWallpaper() -> String {
    wallpaper::get().unwrap()
}

#[napi(js_name = "SetWallpaper")]
pub fn SetWallpaper(path: String, mode: WallpaperMode) {
    wallpaper::set_from_path(path.as_str()).unwrap();
    wallpaper::set_mode(TransformMode(mode)).unwrap();
}

#[napi]
pub enum WallpaperMode {
    Center = 0,
    Crop = 1,
    Fit = 2,
    Span = 3,
    Stretch = 4,
    Tile = 5,
}

fn TransformMode(mode: WallpaperMode) -> wallpaper::Mode {
    match mode {
        WallpaperMode::Center => wallpaper::Mode::Center,
        WallpaperMode::Crop => wallpaper::Mode::Crop,
        WallpaperMode::Fit => wallpaper::Mode::Fit,
        WallpaperMode::Span => wallpaper::Mode::Span,
        WallpaperMode::Stretch => wallpaper::Mode::Stretch,
        WallpaperMode::Tile => wallpaper::Mode::Tile,
    }
}
