use image::imageops::FilterType;

#[napi(js_name = "ConvertImageFormat")]
pub fn ConvertImageFormat(originPath: String, convertPath: String, options: ConvertOptions) {
    let mut r = image::open(originPath).unwrap();
    if options.width != 0
        && options.height != 0
        && (options.width != r.width() || options.height != r.height())
    {
        match options.keepAspectRatio {
            true => {
                r = r.resize(
                    options.width,
                    options.height,
                    TransformFilter(options.filter),
                )
            }
            false => {
                r = r.resize_exact(
                    options.width,
                    options.height,
                    TransformFilter(options.filter),
                )
            }
        }
    }
    r.save_with_format(convertPath, TransformFormat(options.format))
        .unwrap();
}

#[napi]
pub enum ImageFormat {
    Png = 0,
    Jpeg = 1,
    Gif = 2,
    WebP = 3,
    Pnm = 4,
    Tiff = 5,
    Tga = 6,
    Dds = 7,
    Bmp = 8,
    Ico = 9,
    Hdr = 10,
    OpenExr = 11,
    Farbfeld = 12,
    Avif = 13,
    Qoi = 14,
}

#[napi]
pub enum ImageFilter {
    Nearest = 0,
    Triangle = 1,
    CatmullRom = 2,
    Gaussian = 3,
    Lanczos3 = 4,
}

pub fn TransformFormat(format: ImageFormat) -> image::ImageFormat {
    match format {
        ImageFormat::Png => image::ImageFormat::Png,
        ImageFormat::Jpeg => image::ImageFormat::Jpeg,
        ImageFormat::Gif => image::ImageFormat::Gif,
        ImageFormat::WebP => image::ImageFormat::WebP,
        ImageFormat::Pnm => image::ImageFormat::Pnm,
        ImageFormat::Tiff => image::ImageFormat::Tiff,
        ImageFormat::Tga => image::ImageFormat::Tga,
        ImageFormat::Dds => image::ImageFormat::Dds,
        ImageFormat::Bmp => image::ImageFormat::Bmp,
        ImageFormat::Ico => image::ImageFormat::Ico,
        ImageFormat::Hdr => image::ImageFormat::Hdr,
        ImageFormat::OpenExr => image::ImageFormat::OpenExr,
        ImageFormat::Farbfeld => image::ImageFormat::Farbfeld,
        ImageFormat::Avif => image::ImageFormat::Avif,
        ImageFormat::Qoi => image::ImageFormat::Qoi,
    }
}

fn TransformFilter(filter: ImageFilter) -> FilterType {
    match filter {
        ImageFilter::Nearest => FilterType::Nearest,
        ImageFilter::Triangle => FilterType::Triangle,
        ImageFilter::CatmullRom => FilterType::CatmullRom,
        ImageFilter::Gaussian => FilterType::Gaussian,
        ImageFilter::Lanczos3 => FilterType::Lanczos3,
    }
}

#[napi(object)]
pub struct ConvertOptions {
    pub format: ImageFormat,
    pub filter: ImageFilter,
    pub keepAspectRatio: bool,
    pub width: u32,
    pub height: u32,
}
