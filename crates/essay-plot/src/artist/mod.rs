mod norm;
mod image;
mod gridmesh;
mod colorbar;
mod colormaps;
mod colormap;
mod contour;
mod cycle;
mod color;
mod style;
mod triplot;
pub mod paths;
mod text;
mod stem;
mod container;
mod tricontour;
mod markers;
pub mod patch;
mod artist;
mod collection;
mod lines;

pub use artist::{
    Artist, PlotArtist, PlotId,
};

pub use collection::{
    PathCollection
};

pub use container::{
    Container, ContainerOpt
};

pub use color::{
    ColorCycle,
};

pub use colorbar::{
    Colorbar,
};

pub use colormap::{
    ColorMap,
};

pub use colormaps::{
    ColorMaps,
};

pub use gridmesh::{
    ColorMesh,
};

pub use contour::{
    Contour,
};

pub use norm::{
    Norm, Norms,
};

pub use image::{
    Image, 
};

pub use tricontour::{
    TriContour,
};

pub use cycle::{
    StyleCycle,
};

pub use triplot::{
    TriPlot,
};

pub use lines::{
    Lines2d, LinesOpt, DrawStyle,
};

pub use markers::{
    Markers,
};

pub use patch::{
    PatchTrait,
};

pub use stem::{
    Stem, StemOpt,
};

pub use style::{
    PathStyle, 
};

pub use text::{
    Text, // TextStyle,
};
