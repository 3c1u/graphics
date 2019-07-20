//! Draw a textured mesh.

use math::Matrix2d;
use triangulation;
use types::{Color, SourceRectangle};
use DrawState;
use Graphics;
use ImageSize;

use std::sync::Arc;

/// A textured mesh.
#[derive(Clone)]
pub struct Mesh {
    /// The color
    pub color: Option<Color>,
    /// The image source rectangle
    pub source_rectangle: Option<SourceRectangle>,

    /// The UV coordinates.
    pub uvs: Option<Arc<[[f32; 2]]>>,
    /// The vertices.
    pub vertices: Option<Arc<[[f32; 2]]>>,
}

impl Mesh {
    /// Creates a textured mesh.
    pub fn new() -> Mesh {
        Mesh {
            color: None,
            source_rectangle: None,
            uvs: None,
            vertices: None,
        }
    }

    /// Draws a textured mesh.
    pub fn draw<G>(
        &self,
        texture: &<G as Graphics>::Texture,
        draw_state: &DrawState,
        transform: Matrix2d,
        g: &mut G,
    ) where
        G: Graphics,
    {
        use math::Scalar;

        let color = self.color.unwrap_or([1.0; 4]);

        let source_rectangle = self.source_rectangle.unwrap_or({
            let (w, h) = texture.get_size();
            [0.0, 0.0, w as Scalar, h as Scalar]
        });

        let rectangle = [
            0.0,
            0.0,
            source_rectangle[2] as Scalar,
            source_rectangle[3] as Scalar,
        ];

        let uvs = self
            .uvs
            .clone()
            .unwrap_or(Arc::new(triangulation::rect_tri_list_uv(
                texture,
                source_rectangle,
            )));
        let vertices = self
            .vertices
            .clone()
            .unwrap_or(Arc::new(triangulation::rect_tri_list_xy(
                transform, rectangle,
            )));

        g.tri_list_uv(draw_state, &color, texture, move |f| f(&uvs, &vertices));
    }
}

impl Default for Mesh {
    fn default() -> Self {
        Self::new()
    }
}
