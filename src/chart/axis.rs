use essay_graphics::api::{
    renderer::{Canvas, Renderer}, 
    Bounds, TextStyle,
};

use crate::{
    artist::TextCanvas, 
    config::{Config, PathStyle}, frame_option_struct, path_style_options,
};

use super::{
    tick_formatter::{Formatter, TickFormatter}, tick_locator::{MaxNLocator, TickLocator}, CartesianFrame, FrameArtist 
};

pub struct Axis {
    pub(super) show_grid: ShowGrid,

    pub(super) major: AxisTicks,
    pub(super) minor: AxisTicks,

    pub(super) locator: Box<dyn TickLocator>,
    pub(super) formatter: Box<dyn TickFormatter>,

    pub(super) ticks: Option<Vec<f32>>,
    pub(super) labels: Option<Vec<String>>,

    pub(super) is_visible: bool,
}

impl Axis {
    pub fn new(cfg: &Config, prefix: &str) -> Self {
        Self {
            show_grid: ShowGrid::None,
            major: AxisTicks::new(cfg, &cfg.join(prefix, "major")),
            minor: AxisTicks::new(cfg, &cfg.join(prefix, "minor")),
            locator: Box::new(MaxNLocator::new(None)),
            formatter: Box::new(Formatter::Plain),
            ticks: None,
            labels: None,
            is_visible: true,
        }
    }

    pub(crate) fn major(&self) -> &AxisTicks {
        &self.major
    }

    pub(crate) fn major_mut(&mut self) -> &mut AxisTicks {
        &mut self.major
    }

    pub(crate) fn _minor(&self) -> &AxisTicks {
        &self.minor
    }

    pub(crate) fn minor_mut(&mut self) -> &mut AxisTicks {
        &mut self.minor
    }

    pub fn value_delta(xvalues: &Vec<f32>) -> f32 {
        let len = xvalues.len();

        if len <= 1 {
            return 1.;
        }

        let mut delta = (xvalues[len - 1] - xvalues[0]).abs();
        for i in 0..len - 1 {
            delta = (xvalues[i + 1] - xvalues[i]).abs().min(delta);
        }

        delta
    }

    pub(crate) fn get_show_grid(&self) -> &ShowGrid {
        &self.show_grid
    }

    pub(crate) fn _visible(&mut self, is_visible: bool) {
        self.is_visible = is_visible;
    }

    pub(crate) fn is_visible(&self) -> bool {
        self.is_visible
    }

    fn format(&self, value: f32, delta: f32) -> String {
        self.formatter.format(value, delta)
    }

    pub(crate) fn resize(&mut self, renderer: &mut dyn Renderer, pos: Bounds<Canvas>) {
        self.major.update(renderer, pos);
        self.minor.update(renderer, pos);
    }
}
pub struct AxisTicks {
    pub(super) grid_style: PathStyle,
    pub(super) ticks_style: PathStyle,
    pub(super) label_text: TextCanvas,
    pub(super) size: f32,
    pub(super) pad: f32,
    pub(super) locator: Option<Box<dyn TickLocator>>,
    pub(super) formatter: Option<Box<dyn TickFormatter>>,
}

impl AxisTicks {
    pub(crate) fn new(cfg: &Config, prefix: &str) -> Self {
        let mut ticks = Self {
            grid_style: PathStyle::from_config(cfg, &cfg.join(prefix, "grid")),
            ticks_style: PathStyle::from_config(cfg, &cfg.join(prefix, "ticks")),
            size: match cfg.get_as_type(prefix, "size") {
                Some(size) => size,
                None => 0.,
            },
            pad: match cfg.get_as_type(prefix, "pad") {
                Some(size) => size,
                None => 0.,
            },
            label_text: TextCanvas::new(),
            locator: None,
            formatter: None,
        };

        match cfg.get_as_type::<f32>(prefix, "width") {
            Some(width) => { ticks.ticks_style.line_width(width); }
            None => {}
        };

        ticks.label_text.label("0.0");
        
        ticks
    }

    pub(crate) fn grid_style(&self) -> &PathStyle {
        &self.grid_style
    }

    pub(crate) fn tick_style(&self) -> &PathStyle {
        &self.ticks_style
    }

    pub(crate) fn label_style(&self) -> &TextStyle {
        self.label_text.text_style()
    }

    pub(crate) fn label_style_mut(&mut self) -> &mut TextStyle {
        self.label_text.text_style_mut()
    }

    pub(crate) fn _grid_style_mut(&mut self) -> &mut PathStyle {
        &mut self.grid_style
    }

    pub(crate) fn format(&self, axis: &Axis, value: f32, delta: f32) -> String {
        match &self.formatter {
            Some(formatter) => {
                formatter.format(value, delta)
            }
            None => { 
                axis.format(value, delta) 
            }
        }
    }

    pub(crate) fn get_size(&self) -> f32 {
        self.size
    }

    pub(crate) fn get_pad(&self) -> f32 {
        self.pad
    }

    pub(crate) fn get_label_height(&self) -> f32 {
        self.label_text.height()
    }

    pub(crate) fn update(&mut self, renderer: &mut dyn Renderer, pos: Bounds<Canvas>) {
        self.label_text.update_pos(renderer, pos);
    }
}

frame_option_struct!(AxisOpt, Axis, CartesianFrame, get_axis_mut);

impl AxisOpt {
    pub fn show_grid(&mut self, show: impl Into<ShowGrid>) -> &mut Self {
        self.write(|axis| { axis.show_grid = show.into(); });
        self
    }

    pub fn visible(&mut self, is_visible: bool) -> &mut Self {
        self.write(|axis| { axis.is_visible = is_visible; });
        self
    }

    pub fn locator(&mut self, locator: impl TickLocator + 'static) -> &mut Self {
        self.write(|axis| { 
            axis.locator = Box::new(locator); 
        });

        self
    }

    pub fn formatter(&mut self, formatter: impl TickFormatter + 'static) -> &mut Self {
        self.write(|axis| { 
            axis.formatter = Box::new(formatter); 
        });

        self
    }

    pub fn ticks(&mut self, ticks: &[f32]) -> &mut Self {
        self.write(|axis| {
            axis.ticks = Some(Vec::from(ticks));
            axis.labels = None;
        });

        self
    }

    pub fn tick_labels(&mut self, tick_labels: &[(f32, &str)]) -> &mut Self {
        let ticks = tick_labels.iter().map(|t| t.0).collect();
        let labels = tick_labels.iter().map(|t| String::from(t.1)).collect();

        self.write(|axis| {
            axis.ticks = Some(ticks);
            axis.labels = Some(labels);
        });

        self
    }

    pub fn major(&self) -> AxisTicksOpt {
        let artist = match self.artist {
            FrameArtist::X => FrameArtist::XMajor,
            FrameArtist::Y => FrameArtist::YMajor,
            _ => panic!("invalid major()")
        };

        AxisTicksOpt::new(&self.view, artist)
    }

    pub fn major_grid(&self) -> AxisGridOpt {
        let artist = match self.artist {
            FrameArtist::X => FrameArtist::XMajor,
            FrameArtist::Y => FrameArtist::YMajor,
            _ => panic!("invalid major()")
        };

        AxisGridOpt::new(&self.view, artist)
    }
}

frame_option_struct!(AxisGridOpt, AxisTicks, CartesianFrame, get_ticks_mut);

impl AxisGridOpt {
    path_style_options!(grid_style);
}

frame_option_struct!(AxisTicksOpt, AxisTicks, CartesianFrame, get_ticks_mut);

impl AxisTicksOpt {
    pub fn locator(&mut self, locator: impl TickLocator + 'static) -> &mut Self {
        self.write(|ticks| { 
            ticks.locator = Some(Box::new(locator)); 
        });
        self
    }

    pub fn formatter(&mut self, formatter: impl TickFormatter + 'static) -> &mut Self {
        self.write(|ticks| { 
            ticks.formatter = Some(Box::new(formatter)); 
        });
        self
    }

    path_style_options!(ticks_style);
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ShowGrid {
    None,
    Major,
    Minor,
    Both,
}

impl ShowGrid {
    pub(crate) fn is_show_major(&self) -> bool {
        match self {
            ShowGrid::None => false,
            ShowGrid::Major => true,
            ShowGrid::Minor => false,
            ShowGrid::Both => true,
        }
    }

    pub(crate) fn _is_show_minor(&self) -> bool {
        match self {
            ShowGrid::None => false,
            ShowGrid::Major => false,
            ShowGrid::Minor => true,
            ShowGrid::Both => true,
        }
    }
}

impl From<bool> for ShowGrid {
    fn from(value: bool) -> Self {
        if value {
            Self::Major
        } else {
            Self::None
        }
    }
}

