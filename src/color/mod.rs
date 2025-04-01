pub mod color;
mod color_cycle;
mod colormap;
mod colormaps;

pub use color::{
    Hsv, Hsva, Hsl,
};

pub use color_cycle::{ColorCycle, Sequential, Qualitative, Diverging};

pub use colormap::ColorMap;

pub use colormaps::ColorMaps;

