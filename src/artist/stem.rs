use core::fmt;

use essay_tensor::tensor::{Axis, Tensor};

use essay_graphics::api::{
    renderer::{Canvas, Renderer, Result}, 
    Bounds, Path, PathCode, PathOpt, Point
};

use crate::{
    artist::Markers, 
    chart::{Data, LegendHandler}, 
    config::{ConfigArc, PathStyle},
    data_artist_option_struct, path_style_options, transform::ToCanvas
};

use super::{Artist, ArtistDraw, ArtistView, PathCollection};

pub struct Stem {
    xy: Tensor,
    paths: Vec<Path<Data>>,

    marker: Option<Markers>,
    markers: Option<PathCollection>,

    line_style: PathStyle,
    baseline_style: PathStyle,
    marker_style: PathStyle,

    label: Option<String>,

    extent: Bounds<Data>,
    is_stale: bool,
}

impl Stem {
    pub fn from_xy(x: impl Into<Tensor>, y: impl Into<Tensor>) -> Self {
        let x = x.into();
        let y = y.into();

        assert_eq!(x.len(), y.len(), "stem data lengths must match x={:?} y={:?}", 
            x.shape(), y.shape()
        );

        let xy = x.stack([y], Axis::axis(-1));

        Self {
            xy,
            paths: Vec::new(),
            marker: Some(Markers::Circle),
            markers: None,

            line_style: PathStyle::new(),
            baseline_style: PathStyle::new(),
            marker_style: PathStyle::new(),

            label: None,

            extent: Bounds::<Data>::none(),
            is_stale: true,
        }
    }

    fn resize(&mut self, renderer: &mut dyn Renderer) {
        if self.is_stale {
            self.is_stale = false;

            self.extent = Bounds::from(&self.xy);
            self.paths = build_paths(&self.xy);

            // 0.5 because source is [-1, 1]
            let scale = renderer.to_px(3.);

            if let Some(marker) = &self.marker {
                let path: Path<Canvas> = marker.get_scaled_path(scale);

                self.markers = Some(PathCollection::new(path, &self.xy));
            }
        }

        /*
        if let Some(markers) = &mut self.markers {
            markers.resize(renderer, pos);
        }
        */
    }
    
}

impl ArtistDraw<Data> for Stem {
    fn bounds(&mut self) -> Bounds<Data> {
        self.extent.clone()
    }

    fn draw(
        &mut self, 
        renderer: &mut dyn Renderer, 
        to_canvas: &ToCanvas<Data>,
        style: &dyn PathOpt,
    ) -> Result<()> {
        self.resize(renderer);

        let line_style = self.line_style.push(style);

        for path in &self.paths {
            let path = to_canvas.transform_path(path);
            renderer.draw_path(&path, &line_style)?;
        }
        
        if let Some(markers) = &mut self.markers {
            let marker_style = self.marker_style.push(style);

            markers.draw(renderer, to_canvas, &marker_style)?;
        }

        let baseline = Path::<Data>::new(vec![
            PathCode::MoveTo(Point(self.extent.xmin(), 0.)),
            PathCode::LineTo(Point(self.extent.xmax(), 0.)),
        ]);

        let baseline: Path<Canvas> = to_canvas.transform_path(&baseline);
        let baseline_style = self.baseline_style.push(style);

        renderer.draw_path(&baseline, &baseline_style)
    }
}

impl Artist<Data> for Stem {
    type Opt = StemOpt;

    fn config(&mut self, cfg: &ConfigArc) {
        self.line_style = PathStyle::from_config(cfg, "stem.lines");
        self.baseline_style = PathStyle::from_config(cfg, "stem.lines");
        self.marker_style = PathStyle::from_config(cfg, "stem.marker");

        self.baseline_style.color("red"); // C3
    }

    fn opt(&mut self, view: ArtistView<Data, Stem>) -> Self::Opt {
        StemOpt::new(view)
    }

    fn get_legend(&self) -> Option<LegendHandler> {
        match &self.label {
            Some(label) => {
                let style = self.line_style.clone();
                Some(LegendHandler::new(label.clone(), 
                move |renderer, parent_style, bounds| {
                    let line = Path::<Canvas>::from([
                        [bounds.xmin(), bounds.ymid()],
                        [bounds.xmax(), bounds.ymid()],
                    ]);

                    renderer.draw_path(
                        &line,
                        &style.push(parent_style), 
                    )
                }))
            },
            None => None,
        }
    }
}

fn build_paths(xy: &Tensor) -> Vec<Path<Data>> {
    assert!(xy.rank() == 2 && xy.cols() == 2);
    
    let mut paths = Vec::<Path<Data>>::new();

    for xy in xy.iter_row() {
        let path = Path::<Data>::new(vec![
            PathCode::MoveTo(Point(xy[0], 0.)),
            PathCode::LineTo(Point(xy[0], xy[1])),
        ]);

        paths.push(path);
    }

    paths
}

data_artist_option_struct!(StemOpt, Stem);

impl StemOpt {
    path_style_options!(line_style);

    pub fn label(&mut self, label: &str) -> &mut Self {
        self.write(|artist| {
            if label.len() > 0 {
                artist.label = Some(label.to_string());
            } else {
                artist.label = None;
            }
        });

        self
    }
}

//impl PathStyleArtist for Lines2d {
//    fn style_mut(&mut self) -> &mut PathStyle {
//        &mut self.style
//    }
//}

impl fmt::Debug for Stem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.xy.dim(0) {
            0 => {
                write!(f, "Stem[]")
            },
            1 => {
                write!(f, "Stem[({}, {})]", self.xy[(0, 0)], self.xy[(0, 1)])
            },
            2 => {
                write!(f, "Stem[({}, {}), ({}, {})]", 
                    self.xy[(0, 0)], self.xy[(0, 1)],
                    self.xy[(1, 0)], self.xy[(1, 1)])
            },
            n => {
                write!(f, "Stem[({}, {}), ({}, {}), ..., ({}, {})]", 
                    self.xy[(0, 0)], self.xy[(0, 1)],
                    self.xy[(1, 0)], self.xy[(1, 1)],
                    self.xy[(n - 1, 0)], self.xy[(n - 1, 1)])
            }
        }
    }
}

#[cfg(test)]
mod test {
    //use essay_tensor::prelude::*;
}