mod style;
mod config;
mod figure;
pub mod chart;

pub use chart::Chart;

pub use figure::Figure;

pub use config::{
    Config, ConfigArc,
};

pub use style::{
    PlotOpt,
    //PlotId, // PlotOpt, PlotRef, 
    //PlotArtist, //PathStyleArtist,
};
