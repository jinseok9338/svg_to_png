use std::error::Error;
use std::fmt;

// Define a custom error enum
#[derive(Debug)]
pub enum SVGError {
    InvalidExtension, // this is for not valid extension
    InvalidSVG,       // this is for not valid svg
    RSVGError(rsvg::Error),
    InvalidPath,
}

// Implement the Error trait for the custom error enum
impl Error for SVGError {}

// Implement the Display trait to provide a user-friendly error message
impl fmt::Display for SVGError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SVGError::InvalidSVG => write!(f, "This is wrong svg file check the file again"),
            SVGError::InvalidExtension => write!(f, "This is invlid extension use .svg"),
            SVGError::RSVGError(e) => write!(f, "This is rsvg error: {}", e),
            SVGError::InvalidPath => write!(f, "This is invalid path"),
        }
    }
}

#[derive(Debug)]
pub enum RenderError {
    CairoError(cairo::Status),
    SVGError(SVGError),
}

impl Error for RenderError {}

impl fmt::Display for RenderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RenderError::CairoError(e) => write!(f, "This is cairo error: {:?}", e),
            RenderError::SVGError(e) => write!(f, "This is svg error: {}", e),
        }
    }
}
