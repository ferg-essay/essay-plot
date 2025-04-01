pub mod color;
mod color_cycle;
mod colormap;
mod colormaps;
pub(super) mod palette_brewer;
pub(super) mod palette_vega;

pub use color::{
    Hsv, Hsva, Hsl,
};

pub use color_cycle::{ColorCycle, Sequential, Qualitative, Diverging};

pub use colormap::ColorMap;

pub use colormaps::ColorMaps;

