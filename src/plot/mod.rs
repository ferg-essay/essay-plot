use essay_tensor::Tensor;
use crate::{graph::Graph, artist::{LinesOpt, ContainerOpt}};

mod grid_color;
mod contour;
mod bar;
mod matshow;
mod pie;
mod scatter;
mod stem;
mod plot;
mod fill_between;
mod triplot;
mod tricontour;

pub use bar::{
    bar_y, BarOpt,
};

pub use grid_color::grid_color;

pub use contour::contour;

pub use tricontour::tricontour;

pub use fill_between::fill_between;

pub use plot::plot;

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
