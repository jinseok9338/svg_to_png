#![forbid(unsafe_code)]

pub mod errors;
pub mod render;
pub mod svgs;

#[cfg(test)]
mod tests {
    const TEST_FILE_PATH: &str = "./src/assets/example.svg";

    use std::path::Path;

    use crate::{
        render::{make_surface_into_dynamic_image, render_image, save_png_to_path},
        svgs::get_svg_handler,
    };

    use super::*;

    /// Test for valid extension
    /// This test should return SVG file type
    #[test]
    fn test_valid_svg_extension() {
        let file_path = Path::new(TEST_FILE_PATH);
        let file_type = svgs::check_svg(file_path);

        assert_eq!(file_type, true);
    }

    /// Test for valid extension
    /// This test should return SVG file type
    #[test]
    fn test_invalid_svg_extension() {
        let file_path = Path::new("./src/assets/example.png");
        let file_type = svgs::check_svg(file_path);
        assert_eq!(file_type, false);
    }

    #[test]
    fn test_make_handle_for_svg() {
        let handler = get_svg_handler(Path::new(TEST_FILE_PATH));
        assert_eq!(handler.is_ok(), true);
        let handler = handler.unwrap();
        println!("{:?}", handler);
        let surface = render_image(handler.width, handler.height, handler.handle, None);
        assert_eq!(surface.is_ok(), true);
        println!("{:?}", surface);
        let _ = save_png_to_path(&Path::new("./src/assets/example.png"), &surface.unwrap());
    }

    #[test]
    fn test_make_dynamic_image() {
        let handler = get_svg_handler(Path::new(TEST_FILE_PATH));
        assert_eq!(handler.is_ok(), true);
        let handler = handler.unwrap();
        let surface = render_image(handler.width, handler.height, handler.handle, None);
        assert_eq!(surface.is_ok(), true);
        let image = make_surface_into_dynamic_image(&mut surface.unwrap());
        assert_eq!(image.is_ok(), true);
    }
}
