pub mod color;
pub mod config;
pub mod tri;
pub mod contour;
pub mod macros;
pub mod chart;
pub mod artist;
pub mod plot;

pub mod api {
    pub use essay_graphics::api::*;
}

pub mod wgpu {
    pub use essay_graphics::wgpu::*;
}

pub mod prelude {
    pub use crate::chart::Figure;
    // pub use crate::plot::{Plot, PlotOpt};

    pub use crate::api::*;
}