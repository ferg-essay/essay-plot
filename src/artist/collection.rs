use core::fmt;

use essay_tensor::tensor::Tensor;
use essay_graphics::api::{affine2d, path_style::MeshStyle, renderer::{Canvas, Renderer, Result}, Affine2d, Bounds, Color, Path, PathOpt};

use crate::{chart::Data, config::PathStyle, transform::ToCanvas};

use super::ArtistDraw;

///
/// Collection of a single path displayed at multiple locations with optional
/// colors and sizes
/// 
pub struct PathCollection {
    path: Path<Canvas>,
    xy: Tensor, // 2d tensor representing a graph
    color: Tensor<u32>,
    scale: Tensor,
    style: PathStyle,
    affine: Vec<Affine2d>,
    bounds: Bounds<Data>,
}

impl PathCollection {
    pub fn new(path: Path<Canvas>, xy: impl Into<Tensor>) -> Self {
        let xy = xy.into();

        assert!(xy.cols() == 2, "Collection requires two-column data [x, y]*");

        let affine = xy.iter_row().map(|xy| {
            affine2d::translate(xy[0], xy[1])
        }).collect();

        Self {
            path,
            bounds: Bounds::from(&xy),
            xy,
            color: Tensor::from(None),
            scale: Tensor::from(None),
            style: PathStyle::new(), // needs to be loop
            affine,
        }
    }

    pub fn style_mut(&mut self) -> &mut PathStyle {
        &mut self.style
    }
}

impl ArtistDraw<Data> for PathCollection {
    fn bounds(&mut self) -> Bounds<Data> {
        self.bounds.clone()
    }

    fn draw(
        &mut self, 
        ui: &mut dyn Renderer, 
        _to_canvas: &ToCanvas<Data>,
        style: &dyn PathOpt,
    ) -> Result<()> {
        let style = self.style.push(style);

        // TODO: rework with markers
        if style.get_face_color().is_some() && ! style.get_face_color().unwrap().is_none() {
            let markers: Vec<MeshStyle> = self.affine.iter().map(|affine| {
                MeshStyle {
                    color: style.get_face_color().unwrap_or(Color::black()),
                    affine: affine.clone(),
                }
            }).collect();

            ui.draw_markers(&self.path, &style, markers.as_slice())
        } else {
            let markers: Vec<MeshStyle> = self.affine.iter().map(|affine| {
                MeshStyle {
                    color: style.get_edge_color().unwrap_or(Color::black()),
                    affine: affine.clone(),
                }
            }).collect();

            ui.draw_markers(&self.path, &style, markers.as_slice())
        }
    }
}

impl fmt::Debug for PathCollection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.xy.dim(0) {
            0 => {
                write!(f, "Collection[]")
            },
            1 => {
                write!(f, "Collection[({}, {})]", self.xy[(0, 0)], self.xy[(0, 1)])
            },
            2 => {
                write!(f, "Collection[({}, {}), ({}, {})]", 
                    self.xy[(0, 0)], self.xy[(0, 1)],
                    self.xy[(1, 0)], self.xy[(1, 1)])
            },
            n => {
                write!(f, "Collection[({}, {}), ({}, {}), ..., ({}, {})]", 
                    self.xy[(0, 0)], self.xy[(0, 1)],
                    self.xy[(1, 0)], self.xy[(1, 1)],
                    self.xy[(n - 1, 0)], self.xy[(n - 1, 1)])
            }
        }
    }
}
