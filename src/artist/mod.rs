mod artist;
mod bar;
mod collection;
mod colorbar;
mod container;
mod contour;
mod grid_color;
mod histogram;
mod image;
mod lines;
mod markers;
mod norm;
pub mod patch;
mod triplot;
pub mod paths;
mod quiver;
mod span;
mod stem;
mod text;
mod tricontour;

pub use artist::{
    Artist, ArtistDraw, ArtistContainer, ArtistView, IntoArtist, Stale,
};

pub use bar::{
    Bar, BarOpt
};

pub use collection::PathCollection;

pub use container::{
    Container, ContainerOpt
};

pub use colorbar::Colorbar;

pub use grid_color::{
    GridColor, GridColorOpt, Shading,
};

pub use contour::Contour;

pub use norm::{
    Norm, Norms,
};

pub use histogram::{
    Histogram, HistogramOpt, 
};

pub use image::{
    Image, ImageOpt
};

pub use tricontour::TriContour;

pub use triplot::TriPlot;

pub use lines::{
    Lines2d, LinesOpt, DrawStyle,
};

pub use markers::{
    Markers, MarkerStyle, IntoMarker,
};

pub use patch::{
    Patch,
    PatchTrait,
    arrow, Arrow,
};

pub use quiver::{
    Quiver,
    QuiverOpt,
};

pub use span::{
    HorizontalLine, HorizontalLineOpt
};

pub use stem::{
    Stem, StemOpt,
};

pub use text::{
    Text, TextOpt, TextCoords, TextCanvas, // TextStyle,
};
