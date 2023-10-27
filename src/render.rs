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

pub fn save_to_path(path: &Path, surface: &ImageSurface) {
    let file = File::create(path);

    let mut file = match file {
        Ok(file) => file,
        Err(_) => return,
    };

    let _result = surface.write_to_png(&mut file);
}
