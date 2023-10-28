use std::{fs::File, path::Path};

use cairo::{Context, Format, ImageSurface};
use rsvg::{Handle, HandleExt};

use crate::errors::RenderError;

pub fn render_image(
    width: i32,
    height: i32,
    handle: Handle,
    background: Option<(f64, f64, f64, f64)>,
) -> Result<ImageSurface, RenderError> {
    let surface = ImageSurface::create(Format::ARgb32, width, height);
    let surface = match surface {
        Ok(surface) => surface,
        Err(_) => return Err(RenderError::CairoError(cairo::Status::SurfaceFinished)),
    };
    let context = Context::new(&surface);

    let background = background.unwrap_or((1.0, 1.0, 1.0, 1.0));
    context.set_source_rgba(background.0, background.1, background.2, background.3);
    context.paint();
    let _result = handle.render_cairo(&context);
    Ok(surface)
}

pub fn save_png_to_path(path: &Path, surface: &ImageSurface) -> Result<(), RenderError> {
    let file = File::create(path);

    let mut file = match file {
        Ok(file) => file,
        Err(_) => {
            return Err(RenderError::FileSaveError(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Error while saving file",
            )))
        }
    };

    let _result = surface.write_to_png(&mut file);
    Ok(())
}

pub fn make_surface_into_dynamic_image(
    surface: &mut ImageSurface,
) -> Result<image::DynamicImage, RenderError> {
    let width = surface.get_width() as u32;
    let height = surface.get_height() as u32;

    // Get the pixel data from the ImageSurface
    let surface_data = surface
        .get_data()
        .map_err(|_| RenderError::CairoError(cairo::Status::SurfaceFinished))?;

    // Create a new buffer and copy the pixel data into it
    let mut buffer = Vec::with_capacity(surface_data.len());

    for chunk in surface_data.chunks_exact(4) {
        let b = chunk[0];
        let g = chunk[1];
        let r = chunk[2];
        let a = chunk[3];
        buffer.extend_from_slice(&[r, g, b, a]);
    }
    let data = image::RgbaImage::from_raw(width, height, buffer).expect("Error while making image");

    let image = image::DynamicImage::ImageRgba8(data);

    Ok(image)
}
