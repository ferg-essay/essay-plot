mod axis;
mod config;
mod data_frame;
mod figure;
mod frame;
mod chart;
mod legend;
mod tick_formatter;
mod tick_locator;
mod style;

pub use chart::{Chart, // Builder, 
    IntoArtist};

pub use figure::{Figure, SubFigure};

pub use config::{
    Config, ConfigArc,
};

pub use style::PlotOpt;

pub use tick_locator::IndexLocator;

pub use axis::{AxisOpt, ShowGrid};

pub use data_frame::{
    Data, Scaling, AspectMode, ArtistViewImpl,
};

pub use frame::{
    FrameArtist, FrameTextOpt,
};
pub use frame::ChartFrame;

pub use legend::{
    Legend, LegendHandler,
};
