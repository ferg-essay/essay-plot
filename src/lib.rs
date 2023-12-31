pub mod tri;
pub mod contour;
pub mod macros;
pub mod graph;
pub mod artist;
pub mod frame;
pub mod plot;

pub mod api {
    pub use essay_plot_api::*;
}

pub mod wgpu {
    pub use essay_plot_wgpu::*;
}

pub mod prelude {
    pub use crate::graph::Figure;
    // pub use crate::plot::{Plot, PlotOpt};

    pub use crate::api::*;
}