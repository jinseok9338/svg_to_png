// I want to make function that load svg and check if the file is real svg

use std::{fs::File, io::Read, path::Path};

use rsvg::{Handle, HandleExt};
use xmltree::Element;

use crate::errors::SVGError;

#[derive(Debug, PartialEq)]
pub enum FileType {
    SVG,
    PNG,
    JPG,
}

#[derive(Debug, PartialEq)]
pub struct SVGHandler {
    pub handle: Handle,
    pub width: i32,
    pub height: i32,
}

fn string_to_file_type(input: &str) -> Result<FileType, SVGError> {
    match input.to_lowercase().as_str() {
        "svg" => Ok(FileType::SVG),
        "png" => Ok(FileType::PNG),
        "jpg" | "jpeg" => Ok(FileType::JPG),
        _ => Err(SVGError::InvalidExtension),
    }
}

pub fn check_svg(path: &Path) -> bool {
    let ext_string = path.extension();
    let ext_string = match ext_string {
        Some(ext) => ext.to_str().expect("Invalid extension"),
        None => return false,
    };

    let ext = string_to_file_type(ext_string);

    if ext.is_err() || ext.unwrap() != FileType::SVG {
        return false;
    }

    true
}

pub fn get_scaled_svg_handler(path: &Path, scale: f32) -> Result<SVGHandler, SVGError> {
    //read the file first
    let mut file = File::open(path).map_err(|_| SVGError::InvalidPath)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|_| SVGError::InvalidSVG)?;

    let mut svg =
        Element::parse(contents.as_bytes()).map_err(|error| SVGError::ParseError(error))?;

    if let Some(width_str) = svg.attributes.get("width") {
        let width: f32 = width_str
            .parse()
            .map_err(|error| SVGError::ParseError2(error))?;
        svg.attributes
            .insert("width".to_string(), (width * scale).to_string());
    }

    if let Some(height_str) = svg.attributes.get("height") {
        let height: f32 = height_str
            .parse()
            .map_err(|error| SVGError::ParseError2(error))?;
        svg.attributes
            .insert("height".to_string(), (height * scale).to_string());
    }

    let mut svg_data: Vec<u8> = vec![];
    let read = svg.get_text().unwrap();
    svg_data.extend_from_slice(read.as_bytes());

    let handle = Handle::new_from_data(&svg_data);

    let handle = handle.map_err(|_| SVGError::InvalidSVG)?;

    let dimensions = handle.get_dimensions();

    Ok(SVGHandler {
        handle,
        width: dimensions.width,
        height: dimensions.height,
    })
}

pub fn get_svg_handler(path: &Path) -> Result<SVGHandler, SVGError> {
    let file = File::open(path);
    let mut file = match file {
        Ok(file) => file,
        Err(_) => return Err(SVGError::InvalidPath),
    };

    let mut svg_data = vec![];
    let read = file.read_to_end(&mut svg_data);
    if read.is_err() {
        return Err(SVGError::InvalidSVG);
    }

    let handle = Handle::new_from_data(&svg_data);

    let handle = handle.map_err(|_| SVGError::InvalidSVG)?;

    let dimensions = handle.get_dimensions();

    Ok(SVGHandler {
        handle,
        width: dimensions.width,
        height: dimensions.height,
    })
}

#[cfg(test)]
mod tests {
    const TEST_FILES_PATH: &str = "./src/assets/test_files";

    use std::path::Path;

    use super::*;

    #[test]
    fn test_get_svg_handler() {
        let files_dir = Path::new(TEST_FILES_PATH);
        let files = files_dir.read_dir().unwrap();
        for file in files {
            let file = file.unwrap();
            let path = file.path();
            println!("{:?}", path);
            // if path extension is not svg ignore
            if !check_svg(&path) {
                continue;
            }
            let handler = get_svg_handler(&path);
            assert_eq!(handler.is_ok(), true);
            let handler = handler.unwrap();
            println!("{:?}", handler);
        }
    }
}
