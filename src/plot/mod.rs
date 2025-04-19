use essay_graphics::api::{Path, PathCode, Point};
use essay_tensor::tensor::Tensor;

use crate::{
    artist::{patch::PathPatch, ContainerOpt, HorizontalLineOpt, ImageOpt, Lines2d, LinesOpt, TextOpt}, 
    chart::{Chart, PlotOpt, PolarChart}
};

mod bar;
mod contour;
mod fill_between;
mod grid_color;
mod histogram;
mod image;
mod matshow;
mod pie;
mod plot;
mod quiver;
mod radar;
mod scatter;
mod span;
mod specgram;
mod stem;
mod text;
mod triplot;
mod tricontour;

pub use bar::bar;

pub use grid_color::grid_color;

pub use contour::contour;

pub use tricontour::tricontour;

pub use fill_between::fill_between;

pub use histogram::hist;

pub use matshow::matshow;

pub use plot::{plot, plot_y};

pub use quiver::quiver;

pub use radar::{radar, RadarOpt};

pub use span::hline;

pub use stem::stem;

pub use text::text;

pub use pie::pie;

pub use scatter::{
    scatter, ScatterOpt,
};

pub use triplot::triplot;

use self::specgram::SpecGramOpt;

impl Chart {
    pub fn hline(
        &mut self, 
        y: f32,
    ) -> HorizontalLineOpt {
        span::hline(self, y)
    }

    pub fn image(
        &mut self, 
        y: impl Into<Tensor>,
    ) -> ImageOpt {
        image::image(self, y)
    }

    pub fn pie(
        &mut self,
        x: impl Into<Tensor>, 
    ) -> ContainerOpt {
        pie::pie(self, x)
    }

    pub fn plot(
        &mut self, 
        x: impl Into<Tensor>,
        y: impl Into<Tensor>,
    ) -> LinesOpt {
        plot::plot(self, x, y)
    }

    pub fn plot_xy(
        &mut self, 
        values: impl Into<Tensor>,
    ) -> LinesOpt {
        plot::plot_xy(self, values)
    }

    pub fn plot_y(
        &mut self, 
        y: impl Into<Tensor>,
    ) -> LinesOpt {
        plot::plot_y(self, y)
    }

    pub fn scatter(
        &mut self, 
        x: impl Into<Tensor>,
        y: impl Into<Tensor>,
    ) -> ScatterOpt {
        scatter::scatter(self, x, y)
    }

    pub fn specgram(
        &mut self, 
        y: impl Into<Tensor>,
    ) -> SpecGramOpt {
        specgram::specgram(self, y)
    }

    pub fn text(
        &mut self,
        pos: impl Into<Point>, 
        text: impl AsRef<str>,
    ) -> TextOpt {
        text::text(self, pos, text)
    }
}


impl PolarChart {
    pub fn plot(
        &mut self, 
        x: impl Into<Tensor>,
        y: impl Into<Tensor>,
    ) -> LinesOpt {
        let lines = Lines2d::from_xy(x, y);

        self.artist(lines)
    }

    pub fn plot_y(
        &mut self,
        y: impl Into<Tensor>,
    ) -> LinesOpt {
        let lines = Lines2d::from_y(y);

        self.artist(lines)
    }

    pub fn fill(
        &mut self, 
        x: impl Into<Tensor>,
        y: impl Into<Tensor>,
    ) -> PlotOpt {
        let x = x.into();
        let y = y.into();

        assert_eq!(x.shape(), y.shape());
        assert_eq!(x.rank(), 1);
 
        let mut vec = Vec::<PathCode>::new();

        vec.push(PathCode::MoveTo(Point(x[0], y[0])));
        
        for (x, y) in x.iter().zip(y.iter()).skip(1) {
            vec.push(PathCode::LineTo(Point(*x, *y)));
        }
    
        vec.push(PathCode::ClosePoly(Point(x[0], y[0])));
    
        let patch = PathPatch::new(Path::new(vec));
    
        self.add_simple_artist(patch)
    }
}
