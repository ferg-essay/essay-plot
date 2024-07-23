mod axis;
mod config;
mod data_box;
mod figure;
mod frame;
mod chart;
mod legend;
mod tick_formatter;
mod tick_locator;
mod style;

pub use chart::{Chart, IntoArtist};

pub use figure::Figure;

pub use config::{
    Config, ConfigArc,
};

pub use style::PlotOpt;

pub use tick_locator::IndexLocator;

pub use axis::AxisOpt;

pub use data_box::{
    Data, AspectMode, ArtistView,
};

pub use frame::{
    Frame, FrameArtist, FrameTextOpt,
};

pub use legend::{
    Legend, LegendHandler,
};
