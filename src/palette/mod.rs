mod colorcet;
mod diverging;
mod category;
mod sequential;
pub mod color;
mod palette;
mod colormap;
mod colormaps;
pub(super) mod palette_brewer;
pub(super) mod palette_vega;

pub use color::{
    Hsv, Hsva, Hsl,
};

pub use category::Category;

pub use colorcet::Colorcet;

pub use colormap::ColorMap;

pub use colormaps::ColorMaps;

pub use diverging::Diverging;

pub use palette::Palette;

pub use sequential::Sequential;


