
//
// XAxis
//

use essay_graphics::api::{
    renderer::{self, Canvas, Renderer}, 
    Bounds, HorizAlign, Path, PathOpt, Point, TextStyle, VertAlign
};

use crate::{
    config::Config, 
    transform::Transform
};

use super::{axis::Axis, data_frame::DataFrame, Data};

pub struct XAxis {
    axis: Axis,
    major_ticks: Vec<XTick>,

    is_bottom: bool,

}

impl XAxis {
    pub(crate) fn new(cfg: &Config, prefix: &str) -> Self {
        let mut x_axis = Self {
            axis: Axis::new(cfg, prefix),

            major_ticks: Vec::new(),

            is_bottom: true,
        };

        if x_axis.is_bottom {
            x_axis.axis.major_mut().label_style_mut().valign(VertAlign::Top);
            x_axis.axis.minor_mut().label_style_mut().valign(VertAlign::Top);
        } else {
            x_axis.axis.major_mut().label_style_mut().valign(VertAlign::Bottom);
            x_axis.axis.minor_mut().label_style_mut().valign(VertAlign::Bottom);
        }

        x_axis
    }

    pub fn update_axis(
        &mut self, 
        data: &DataFrame,
        to_canvas: &dyn Transform<Data>,
    ) {
        self.major_ticks = Vec::new();

        let xmin = data.data_bounds().xmin();
        let xmax = data.data_bounds().xmax();

        let xvalues : Vec<f32> = self
            .x_ticks(data.pos(), data.data_bounds())
            .iter()
            .map(|x| x.0)
            .collect();

        let delta = Axis::value_delta(&xvalues);

        for (i, xv) in xvalues.iter().enumerate() {
            if xmin <= *xv && *xv <= xmax {
                let label = if let Some(labels) = &self.axis.labels {
                    labels[i].clone()
                } else {
                    self.axis.major().format(&self.axis, *xv, delta)
                };

                let is_grid = self.axis.get_show_grid().is_show_major();

                self.major_ticks.push(XTick::new(*xv, data, to_canvas, Some(label), is_grid));
            };
        }
    }

    pub fn x_ticks(
        &self, 
        pos: Bounds<Canvas>,
        bounds: Bounds<Data>,
    ) -> Vec<(f32, f32)> {
        let c_width = pos.width();

        let view = bounds;
        let v_width = view.width();

        if view.is_none() {
            Vec::new()
        } else if let Some(ticks) = &self.axis.ticks {
            ticks.iter().map(|x| (*x, *x)).collect::<Vec<(f32, f32)>>()
        } else {
            let (vmin, vmax) = (view.xmin(), view.xmax());
            let (min, max) = self.axis.locator.view_limits(vmin, vmax);

            // self.locator.tick_values(min, max)

            let mut x_vec = Vec::<(f32, f32)>::new();

            for x in self.axis.locator.tick_values(min, max).iter() {
                x_vec.push((*x, ((x - vmin) * c_width / v_width).round()));
            }

            x_vec
        }
    }

    pub(crate) fn draw(
        &mut self, 
        ui: &mut dyn Renderer,
        data: &DataFrame,
        style: &dyn PathOpt,
    ) -> renderer::Result<f32> {
        let pos = data.pos();

        let mut y = if self.is_bottom { pos.ymin() } else { pos.ymax() };
        let sign = if self.is_bottom { -1.0f32 } else { 1.0f32 };

        if self.axis.get_show_grid().is_show_major() {
            let style = self.axis.major().grid_style.push(style);
            for tick in &self.major_ticks {
                tick.draw_grid(ui, &style)?;
            }
        }

        if self.axis.is_visible() {
            self.draw_ticks(ui, style)?;

            y += sign * ui.to_px(self.axis.major().get_size());
            y += sign * ui.to_px(self.axis.major().get_pad());
            y += sign * self.axis.major().get_label_height();
        }

        Ok(y)
    }

    fn draw_ticks(
        &mut self, 
        ui: &mut dyn Renderer,
        style: &dyn PathOpt,
    ) -> renderer::Result<()> {
        if self.axis.is_visible {
            let tick_style = self.axis.major().tick_style().push(style);

            for tick in &self.major_ticks {
                tick.draw_tick(ui, &tick_style)?;
                tick.draw_text(ui, style, self.axis.major().label_style())?;
            }
        }

        Ok(())
    }

    pub(crate) fn resize(&mut self, renderer: &mut dyn Renderer, pos: Bounds<Canvas>) {
        self.axis.resize(renderer, pos);
    }

    pub(crate) fn axis_mut(&mut self) -> &mut Axis {
        &mut self.axis
    }
}

#[derive(Debug)]
struct XTick {
    pos: Point,
    label: Option<String>,
    tick: Option<Path<Canvas>>,
    grid: Option<Path<Canvas>>,
}

impl XTick {
    fn new(
        x: f32, 
        data: &DataFrame, 
        to_canvas: &dyn Transform<Data>,
        label: Option<String>, 
        is_grid: bool
    ) -> Self {
        let (ymin, ymax) = (data.data_bounds().ymin(), data.data_bounds(). ymax());

        let Point(x1, y1) = to_canvas.transform_point(Point(x, ymin));

        let grid = if is_grid {
            let Point(x2, y2) = to_canvas.transform_point(Point(x, ymax));
    
            Some(Path::move_to(x1, y1)
                .line_to(x2, y2)
                .to_path()
            )
        } else {
            None
        };

        // let tick_length = renderer.to_px(major.get_size());
        let tick_length = 10.;
        let sign = -1.;

        let tick = Path::<Canvas>::move_to(x1, y1)
            .line_to(x1, y1 + sign * tick_length).to_path();

        // let pad = renderer.to_px(major.get_pad());
        let pad = 6.;
        let pos = Point(x1, y1 + sign * (tick_length + pad));

        Self {
            tick: Some(tick),
            pos,
            label,
            grid,
        }
    }

    fn draw_text(
        &self, 
        ui: &mut dyn Renderer,
        style: &dyn PathOpt,
        text_style: &TextStyle,
    ) -> renderer::Result<()> {
        if let Some(label) = &self.label {
            ui.draw_text(self.pos, label, 0., style, &text_style)?;
        }

        Ok(())
    }

    fn draw_tick(
        &self, 
        ui: &mut dyn Renderer, 
        style: &dyn PathOpt
    ) -> renderer::Result<()> {
        if let Some(path) = &self.tick {
            ui.draw_path(path, style)?;
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

pub struct YAxis {
    axis: Axis,
    major_ticks: Vec<f32>,
    major_labels: Vec<String>,

    major_ticks2: Vec<YTick>,

    is_left: bool,
}

impl YAxis {
    pub(crate) fn new(cfg: &Config, prefix: &str) -> Self {
        let mut y_axis = Self {
            axis: Axis::new(cfg, prefix),

            major_ticks: Vec::new(),
            major_labels: Vec::new(),
            major_ticks2: Vec::new(),

            is_left: true,
        };

        y_axis.axis.major_mut().label_style_mut().valign(VertAlign::Center);
        if y_axis.is_left {
            y_axis.axis.major_mut().label_style_mut().halign(HorizAlign::Right);
        } else {
            y_axis.axis.major_mut().label_style_mut().halign(HorizAlign::Left);
        }

        y_axis
    }

    pub fn update_axis(
        &mut self, 
        data: &DataFrame,
        to_canvas: &dyn Transform<Data>,
    ) {
        self.major_ticks = Vec::new();
        self.major_labels = Vec::new();
        self.major_ticks2 = Vec::new();

        let ymin = data.data_bounds().ymin();
        let ymax = data.data_bounds().ymax();

        let yvalues : Vec<f32> = self.y_ticks(data);

        let delta = Axis::value_delta(&yvalues);

        for (i, yv) in yvalues.iter().enumerate() {
            if ymin <= *yv && *yv <= ymax {
                self.major_ticks.push(*yv);
                let tick_v = *yv;

                let label = if let Some(labels) = &self.axis.labels {
                    labels[i].clone()
                } else {
                    self.axis.major().format(&self.axis, tick_v, delta)
                };

                self.major_labels.push(label.clone());
                self.major_ticks2.push(YTick::new(*yv, data, to_canvas, Some(label), true));
            };
        }
    }

    pub fn y_ticks(&self, data: &DataFrame) -> Vec<f32> {
        let view = data.data_bounds();

        if view.is_none() {
            Vec::new()
        } else if let Some(ticks) = &self.axis.ticks {
            ticks.clone()
        } else {
            let (vmin, vmax) = (view.ymin(), view.ymax());
            let (min, max) = self.axis.locator.view_limits(vmin, vmax);

            self.axis.locator.tick_values(min, max).as_slice().into()
        }
    }

    pub(crate) fn draw(
        &mut self, 
        ui: &mut dyn Renderer,
        data: &DataFrame,
        style: &dyn PathOpt,
    ) -> renderer::Result<f32> {
        let pos = data.pos();

        let mut x = if self.is_left { pos.xmin() } else { pos.xmax() };
        let sign = if self.is_left { -1.0f32 } else { 1.0f32 };

        // Grid
        if self.axis.get_show_grid().is_show_major() {
            let style = self.axis.major().grid_style().push(style);

            for tick in &self.major_ticks2 {
                tick.draw_grid(ui, &style)?;
            }
        }

        if self.axis.is_visible() {
            for tick in &self.major_ticks2 {
                tick.draw_tick(ui, style)?;
                tick.draw_text(ui, style, self.axis.major().label_style())?;
            }
            // self.draw_ticks(renderer, &data, style)?;

            let width = self.major_labels.iter().map(|s| s.len()).max().unwrap();
        
            x += sign * ui.to_px(self.axis.major().get_size());
            x += sign * ui.to_px(self.axis.major().get_pad());
            x += sign * 0.5 * width as f32 * self.axis.major().get_label_height();
        }

        Ok(x)
    }

    pub(crate) fn update(&mut self, renderer: &mut dyn Renderer, pos: Bounds<Canvas>) {
        self.axis.resize(renderer, pos);
    }

    pub(crate) fn axis_mut(&mut self) -> &mut Axis {
        &mut self.axis
    }
}


#[derive(Debug)]
struct YTick {
    pos: Point,
    label: Option<String>,
    tick: Option<Path<Canvas>>,
    grid: Option<Path<Canvas>>,
}

impl YTick {
    fn new(
        y: f32, 
        data: &DataFrame, 
        to_canvas: &dyn Transform<Data>,
        label: Option<String>, 
        is_grid: bool
    ) -> Self {
        let (xmin, xmax) = (data.data_bounds().xmin(), data.data_bounds().xmax());

        // todo: left vs right
        let Point(x1, y1) = to_canvas.transform_point(Point(xmin, y));

        let grid = if is_grid {
            let Point(x2, y2) = to_canvas.transform_point(Point(xmax, y));
    
            Some(Path::move_to(x1, y1)
                .line_to(x2, y2)
                .to_path()
            )
        } else {
            None
        };

        // let tick_length = renderer.to_px(major.get_size());
        let tick_length = 10.;
        let sign = -1.;

        let tick = Path::<Canvas>::move_to(x1, y1)
            .line_to(x1 + sign * tick_length, y1).to_path();

        // let pad = renderer.to_px(major.get_pad());
        let pad = 6.;
        let pos = Point(x1 + sign * (tick_length + pad), y1);

        Self {
            tick: Some(tick),
            pos,
            label,
            grid,
        }
    }

    fn draw_text(
        &self, 
        ui: &mut dyn Renderer,
        style: &dyn PathOpt,
        text_style: &TextStyle,
    ) -> renderer::Result<()> {
        if let Some(label) = &self.label {
            ui.draw_text(self.pos, label, 0., style, &text_style)?;
        }

        Ok(())
    }

    fn draw_tick(
        &self, 
        ui: &mut dyn Renderer, 
        style: &dyn PathOpt
    ) -> renderer::Result<()> {
        if let Some(path) = &self.tick {
            ui.draw_path(path, style)?;
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
