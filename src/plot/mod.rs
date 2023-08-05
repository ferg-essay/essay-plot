use essay_tensor::Tensor;
use crate::{graph::Graph, artist::{LinesOpt, ContainerOpt}};

mod bar;
mod contour;
mod fill_between;
mod grid_color;
mod histogram;
mod matshow;
mod pie;
mod plot;
mod quiver;
mod scatter;
mod stem;
mod triplot;
mod tricontour;

pub use bar::bar;

pub use grid_color::grid_color;

pub use contour::contour;

pub use tricontour::tricontour;

pub use fill_between::fill_between;

pub use histogram::hist;

pub use plot::plot;

pub use quiver::quiver;

pub use matshow::matshow;

pub use stem::stem;

pub use pie::pie;

pub use scatter::{
    scatter, ScatterOpt,
};

pub use triplot::{
    triplot,
};

impl Graph {
    pub fn plot(
        &mut self, 
        x: impl Into<Tensor>,
        y: impl Into<Tensor>,
    ) -> LinesOpt {
        plot::plot(self, x, y)
    }

    pub fn scatter(
        &mut self, 
        x: impl Into<Tensor>,
        y: impl Into<Tensor>,
    ) -> ScatterOpt {
        scatter::scatter(self, x, y)
    }

    pub fn pie(
        &mut self,
        x: impl Into<Tensor>, 
    ) -> ContainerOpt {
        pie::pie(self, x)
    }
    /*
    pub fn bar_y(
        &mut self, 
        y: impl Into<Tensor>,
    ) -> BarOpt {
        bar::bar_y(self, y)
    }
    */

}
