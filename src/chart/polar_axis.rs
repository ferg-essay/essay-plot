use std::f32::consts::{PI, TAU};

use essay_graphics::api::{
    renderer::{self, Canvas, Renderer, Result}, 
    Bounds, Color, HorizAlign, Path, PathOpt, Point, TextStyle, VertAlign
};

use crate::{
    artist::paths,
    config::Config, 
    frame_option_struct, path_style_options,
};

use super::{
    axis::{Axis, AxisTicks}, 
    data_frame::DataFrame, polar_frame::PolarFrame, 
    tick_formatter::TickFormatter,
    tick_locator::TickLocator,
    FrameArtist, 
    ShowGrid 
};

pub struct PolarXAxis {
    axis: Axis,

    ticks: Vec<XTick>,
}

impl PolarXAxis {
    pub(crate) fn new(cfg: &Config, prefix: &str) -> Self {
        let x_axis = Self {
            axis: Axis::new(cfg, prefix),

            ticks: Vec::new(),
        };

        x_axis
    }

    pub fn resize(&mut self, data: &DataFrame) {
        self.ticks = Vec::new();

        let xmin = data.data_bounds().xmin();
        let xmax = data.data_bounds().xmax();

        let xvalues = if let Some(ticks) = &self.axis.ticks  {
            ticks.clone()
        } else {
            let dx = (xmax - xmin) / 6.;
            (0..6).map(|i| i as f32 * dx).collect()
        };

        let xmin = data.data_bounds().xmin();
        let xmax = data.data_bounds().xmax();

        let delta = Axis::value_delta(&xvalues);

        for (i, xv) in xvalues.iter().enumerate() {
            if xmin <= *xv && *xv <= xmax {
                let label = if let Some(label) = &self.axis.labels {
                    Some(label[i].clone())
                } else {
                    Some(self.axis.major().format(&self.axis, *xv, delta))
                };

                let theta = TAU * *xv / (xmax - xmin).max(f32::EPSILON);

                self.ticks.push(XTick::new(theta, data.pos(), label, true));
            };
        }
    }

    pub(crate) fn draw(
        &mut self, 
        ui: &mut dyn Renderer,
        style: &dyn PathOpt,
    ) -> Result<()> {
        let major_style = self.axis.major().grid_style.push(style);
        for tick in &self.ticks {
            tick.draw_grid(ui, &major_style)?;
        }

        for tick in &self.ticks {
            self.axis.major().grid_style.push(style);
            tick.draw_text(ui, style)?;
        }

        Ok(())
    }

    pub(crate) fn axis_mut(&mut self) -> &mut Axis {
        &mut self.axis
    }
}

#[derive(Debug)]
struct XTick {
    pos: Point,
    label: Option<String>,
    halign: HorizAlign,
    valign: VertAlign,
    grid: Option<Path<Canvas>>,
}

impl XTick {
    fn new(theta: f32, pos: Bounds<Canvas>, label: Option<String>, is_grid: bool) -> Self {
        let (sin, cos) = theta.sin_cos();

        let y_max = pos.width() * 0.5;
        let (xmid, ymid) = (pos.xmid(), pos.ymid());

        let grid = if is_grid {
            Some(Path::move_to(xmid, ymid)
                .line_to(xmid + cos * y_max, ymid + sin * y_max)
                .to_path()
            )
        } else {
            None
        };

        let pad = 4.;
        let pos = Point(xmid + cos * (y_max + pad), ymid + sin * (y_max + pad));

        let (halign, valign) = text_angle_align(theta);

        Self {
            pos,
            label,
            halign,
            valign,
            grid,
        }
    }

    fn draw_text(
        &self, 
        ui: &mut dyn Renderer,
        style: &dyn PathOpt,
    ) -> renderer::Result<()> {
        if let Some(label) = &self.label {
            let mut text_style = TextStyle::new();
            text_style.halign(self.halign);
            text_style.valign(self.valign);

            ui.draw_text(self.pos, label, 0., style, &text_style)?;
        }

        Ok(())
    }

    fn draw_grid(
        &self, 
        ui: &mut dyn Renderer, 
        style: &dyn PathOpt
    ) -> renderer::Result<()> {
        if let Some(path) = &self.grid {
            ui.draw_path(path, style)?;
        }    

        Ok(())
    }
}

//
// YAxis
//

pub struct PolarYAxis {
    axis: Axis,

    ticks: Vec<YTick>,
}

impl PolarYAxis {
    pub(crate) fn new(cfg: &Config, prefix: &str) -> Self {
        let y_axis = Self {
            axis: Axis::new(cfg, prefix),

            ticks: Vec::new(),
        };

        y_axis
    }

    pub fn resize(&mut self, data: &DataFrame) {
        self.ticks = Vec::new();

        let ymin = data.data_bounds().ymin();
        let ymax = data.data_bounds().ymax();
        let ymax = ymin.abs().max(ymax.abs()); // * 0.5;

        let y_values = if let Some(ticks) = &self.axis.ticks  {
            ticks.clone()
        } else {
            let len = 4;
            let dy = ymax / len as f32;
            (1..len + 1).map(|i| i as f32 * dy).collect()
        };

        let delta = Axis::value_delta(&y_values);

        for (i, yv) in y_values.iter().enumerate() {
            if yv.abs() <= ymax {
                let label = if let Some(label) = &self.axis.labels {
                    Some(label[i].clone())
                } else {
                    Some(self.axis.major().format(&self.axis, *yv, delta))
                };

                let angle = PI / 2.;

                self.ticks.push(YTick::new(*yv / ymax, data.pos(), angle, label, true));
            };
        }
    }

    pub(crate) fn draw(
        &mut self, 
        ui: &mut dyn Renderer,
        style: &dyn PathOpt,
    ) -> Result<()> {
        let mut path_style = self.axis.major().grid_style().clone();
        path_style.face_color(Color(0xffffff00));

        let grid_style = path_style.push(style);
        
        for tick in &self.ticks {
            tick.draw_grid(ui, &grid_style)?;
        }

        ui.flush(); // todo: fix bezier requirement for flush
        
        for tick in &self.ticks {
            tick.draw_text(ui, style)?;
        }

        Ok(())
    }

    pub(crate) fn axis_mut(&mut self) -> &mut Axis {
        &mut self.axis
    }
}

#[derive(Debug)]
struct YTick {
    pos: Point,
    label: Option<String>,
    halign: HorizAlign,
    valign: VertAlign,
    grid: Option<Path<Canvas>>,
}

impl YTick {
    fn new(y: f32, pos: Bounds<Canvas>, angle: f32, label: Option<String>, is_grid: bool) -> Self {
        let y_max = pos.width() * 0.5;
        let (xmid, ymid) = (pos.xmid(), pos.ymid());

        let grid = if is_grid {
            let r = y_max * y;
            let circle = paths::circle()
                .scale::<Canvas>(r, r)
                .translate(xmid, ymid);

            Some(circle)
        } else {
            None
        };

        let (sin, cos) = angle.sin_cos();

        let pad = 4.;
        let pos = Point(xmid + y * cos * (y_max + pad), ymid + y * sin * (y_max + pad));

        let (halign, valign) = text_angle_align(angle);

        Self {
            pos,
            label,
            halign,
            valign,
            grid,
        }
    }

    fn draw_text(
        &self, 
        ui: &mut dyn Renderer,
        style: &dyn PathOpt,
    ) -> renderer::Result<()> {
        if let Some(label) = &self.label {
            let mut text_style = TextStyle::new();
            text_style.halign(self.halign);
            text_style.valign(self.valign);

            ui.draw_text(self.pos, label, 0., style, &text_style)?;
        }

        Ok(())
    }

    fn draw_grid(
        &self, 
        ui: &mut dyn Renderer, 
        style: &dyn PathOpt
    ) -> renderer::Result<()> {
        if let Some(path) = &self.grid {
            ui.draw_path(path, style)?;
        }    

        Ok(())
    }
}

fn text_angle_align(theta: f32) -> (HorizAlign, VertAlign) {
    let center = PI / 12.;

    let halign = if PI / 2. - center < theta && theta < PI / 2. + center {
        HorizAlign::Center
    } else if PI / 2. < theta && theta < 3. * PI / 2. {
        HorizAlign::Right
    } else {
        HorizAlign::Left
    };

    let valign = if theta < center || TAU - center < theta {
        VertAlign::Center
    } else if PI - center < theta && theta < PI + center {
        VertAlign::Center
    } else if theta < PI {
        VertAlign::Bottom
    } else {
        VertAlign::Top
    };

    (halign, valign)
}

frame_option_struct!(PolarAxisOpt, Axis, PolarFrame, get_axis_mut);

impl PolarAxisOpt {
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

    pub fn major(&self) -> PolarAxisTicksOpt {
        let artist = match self.artist {
            FrameArtist::X => FrameArtist::XMajor,
            FrameArtist::Y => FrameArtist::YMajor,
            _ => panic!("invalid major()")
        };

        PolarAxisTicksOpt::new(&self.view, artist)
    }

    pub fn major_grid(&self) -> PolarAxisGridOpt {
        let artist = match self.artist {
            FrameArtist::X => FrameArtist::XMajor,
            FrameArtist::Y => FrameArtist::YMajor,
            _ => panic!("invalid major()")
        };

        PolarAxisGridOpt::new(&self.view, artist)
    }
}

frame_option_struct!(PolarAxisGridOpt, AxisTicks, PolarFrame, get_ticks_mut);

impl PolarAxisGridOpt {
    path_style_options!(grid_style);
}

frame_option_struct!(PolarAxisTicksOpt, AxisTicks, PolarFrame, get_ticks_mut);

impl PolarAxisTicksOpt {
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
