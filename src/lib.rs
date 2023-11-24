#![forbid(unsafe_code)]

pub mod errors;
pub mod render;
pub mod svgs;

#[cfg(test)]
mod tests {
    const TEST_FILE_PATH: &str = "./src/assets/example.svg";
    const TEST_FILES_PATH: &str = "./src/assets/test_files";

    use std::path::Path;

    use webp::*;

    use crate::{
        render::{make_surface_into_dynamic_image, render_image, save_png_to_path},
        svgs::{check_svg, get_scaled_svg_handler, get_svg_handler},
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
    fn test_change_svg_to_png() {
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
    fn test_scale_svg() {
        let handler = get_scaled_svg_handler(Path::new(TEST_FILE_PATH), 2.0);
        assert_eq!(handler.is_ok(), true);
        let handler = handler.unwrap();
        println!("{:?}", handler);
        let surface = render_image(handler.width, handler.height, handler.handle, None);
        assert_eq!(surface.is_ok(), true);
        println!("{:?}", surface);
        let _ = save_png_to_path(
            &Path::new("./src/assets/example_with_scaled.png"),
            &surface.unwrap(),
        );
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

        let image = image.unwrap();
        let _ = image.save("./src/assets/example2.png");
    }

    #[test]
    fn test_with_multiple_files() {
        // there are svg files in test files directory I want to loop through them and test them
        let files = std::fs::read_dir(TEST_FILES_PATH);
        assert_eq!(files.is_ok(), true);
        let files = files.unwrap();
        for file in files {
            let file = file.unwrap();
            let path = file.path();
            if !check_svg(&path) {
                continue;
            }
            let handler = get_svg_handler(&path);
            assert_eq!(handler.is_ok(), true);
            let handler = handler.unwrap();

            let surface = render_image(handler.width, handler.height, handler.handle, None);

            assert_eq!(surface.is_ok(), true);
            // save it to png
            let path = path.with_extension("png");
            let _ = save_png_to_path(&path, &surface.unwrap());
        }
    }

    #[test]
    fn make_svg_to_webp_files() {
        let files = std::fs::read_dir(TEST_FILES_PATH);
        assert_eq!(files.is_ok(), true);
        let files = files.unwrap();

        for file in files {
            let file = file.unwrap();
            let path = file.path();
            if !check_svg(&path) {
                continue;
            }
            let handler = get_svg_handler(&path);
            assert_eq!(handler.is_ok(), true);
            let handler = handler.unwrap();

            let surface = render_image(handler.width, handler.height, handler.handle, None);
            let image = make_surface_into_dynamic_image(&mut surface.unwrap());

            let image = image.unwrap();
            let new_img = image.resize(
                handler.width as u32,
                handler.height as u32,
                // Cubic Filter.
                image::imageops::FilterType::CatmullRom,
            );
            // Create the WebP encoder for the above image
            let encoder: Encoder = Encoder::from_image(&new_img).unwrap();
            // Encode the image at a specified quality 0-100
            let webp: WebPMemory = encoder.encode(50 as f32);
            let save_path = path.with_extension("webp");
            std::fs::write(save_path, &*webp).unwrap();

            // save it to png
        }
    }
}
