//! Draw a textured mesh.

use math::Matrix2d;
use triangulation;
use types::{Color, Rectangle, SourceRectangle};
use DrawState;
use Graphics;
use ImageSize;

#[derive(Copy, Clone)]
pub struct Mesh {
    /// The color
    pub color: Option<Color>,
    /// The image source rectangle
    pub source_rectangle: Option<SourceRectangle>,
}

impl Mesh {}

impl Default for Mesh {}
