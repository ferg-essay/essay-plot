
//
// XAxis
//

use essay_graphics::api::{renderer::{self, Canvas, Renderer}, Bounds, HorizAlign, Path, PathCode, PathOpt, Point, VertAlign};

use crate::{artist::{patch::CanvasPatch, paths, ArtistDraw}, config::Config, transform::ToCanvas};

use super::{axis::Axis, data_frame::DataFrame, Data};

pub struct XAxis {
    spine: Option<CanvasPatch>,

    axis: Axis,
    major_ticks: Vec<f32>,
    major_labels: Vec<String>,

    is_bottom: bool,

}

impl XAxis {
    pub(crate) fn new(cfg: &Config, prefix: &str) -> Self {
        let mut x_axis = Self {
            spine: Some(CanvasPatch::new(paths::line([0., 0.], [1., 0.]))),
            axis: Axis::new(cfg, prefix),

            major_ticks: Vec::new(),
            major_labels: Vec::new(),

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

    pub fn update_axis(&mut self, data: &DataFrame) {
        self.major_ticks = Vec::new();
        self.major_labels = Vec::new();

        let xmin = data.data_bounds().xmin();
        let xmax = data.data_bounds().xmax();

        let xvalues : Vec<f32> = self.x_ticks(data.pos(), data.data_bounds())
            .iter().map(|x| x.0).collect();

        let delta = Axis::value_delta(&xvalues);

        for xv in xvalues {
            if xmin <= xv && xv <= xmax {
                self.major_ticks.push(xv);
                self.major_labels.push(
                    self.axis.major().format(&self.axis, xv, delta)
                );
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
        renderer: &mut dyn Renderer,
        data: &DataFrame,
        to_canvas: &ToCanvas<Canvas>,
        style: &dyn PathOpt,
    ) -> renderer::Result<f32> {
        let pos = data.pos();

        let mut y = if self.is_bottom { pos.ymin() } else { pos.ymax() };
        let sign = if self.is_bottom { -1.0f32 } else { 1.0f32 };

        if let Some(patch) = &mut self.spine {
            let line_width = 1.;

            patch.set_pos([
                [pos.xmin(), y - sign * line_width],
                [pos.xmax(), y],
            ]);

            patch.draw(renderer, to_canvas, style)?;
        }

        // let mut y = data.get_pos().ymin();

        if self.axis.is_visible() {
            self.draw_ticks(renderer, &data, style)?;

            y += sign * renderer.to_px(self.axis.major().get_size());
            y += sign * renderer.to_px(self.axis.major().get_pad());
            y += sign * self.axis.major().get_label_height();
        }

        Ok(y)
    }

    fn draw_ticks(
        &mut self, 
        renderer: &mut dyn Renderer, 
        data: &DataFrame,
        style: &dyn PathOpt,
    ) -> renderer::Result<()> {
        let pos = &data.pos();

        let yv = if self.is_bottom { pos.ymin() } else { pos.ymax() };
        let sign = if self.is_bottom { -1.0f32 } else { 1.0f32 };

        let to_canvas = data.get_canvas_transform();

        for (xv, label) in self.major_ticks.iter().zip(self.major_labels.iter()) {
            let point = to_canvas.transform_point(Point(*xv, yv));

            let x = point.x();
            let mut y = yv;
            let major = self.axis.major();

            // Grid
            if self.axis.get_show_grid().is_show_major() {
                let style = major.grid_style().push(style);
                // grid
                let grid = Path::<Canvas>::move_to(x, pos.ymin())
                    .line_to(x, pos.ymax())
                    .to_path();

                renderer.draw_path(&grid, &style)?;
            }

            // Tick
            {
                let style = major.tick_style().push(style);
                let tick_length = renderer.to_px(major.get_size());

                let tick = Path::<Canvas>::move_to(x, y)
                    .line_to(x, y + sign * tick_length).to_path();

                renderer.draw_path(&tick, &style)?;

                y += sign * tick_length;
                y += sign * renderer.to_px(major.get_pad());
            }

            // Label
            renderer.draw_text(Point(x, y), label, 0., style, major.label_style())?;
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

//
// YAxis
//

pub struct YAxis {
    spine: Option<CanvasPatch>,

    axis: Axis,
    major_ticks: Vec<f32>,
    major_labels: Vec<String>,

    is_left: bool,
}

impl YAxis {
    pub(crate) fn new(cfg: &Config, prefix: &str) -> Self {
        let mut y_axis = Self {
            spine: Some(CanvasPatch::new(paths::line([0., 0.], [0., 1.]))),
            axis: Axis::new(cfg, prefix),

            major_ticks: Vec::new(),
            major_labels: Vec::new(),

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

    pub fn update_axis(&mut self, data: &DataFrame) {
        self.major_ticks = Vec::new();
        self.major_labels = Vec::new();

        let ymin = data.data_bounds().ymin();
        let ymax = data.data_bounds().ymax();

        let yvalues : Vec<f32> = self.y_ticks(data).iter().map(|y| y.0).collect();

        let delta = Axis::value_delta(&yvalues);

        for yv in yvalues {
            if ymin <= yv && yv <= ymax {
                self.major_ticks.push(yv);
                let tick_v = yv;
                self.major_labels.push(
                    self.axis.major().format(&self.axis, tick_v, delta)
                );
            };
        }
    }

    pub fn y_ticks(&self, data: &DataFrame) -> Vec<(f32, f32)> {
        let v_height = data.data_bounds().height();
        let c_height = data.pos().height();

        let view = data.data_bounds();

        if view.is_none() {
            Vec::new()
        } else {
            let (vmin, vmax) = (view.ymin(), view.ymax());
            let (min, max) = self.axis.locator.view_limits(vmin, vmax);

            // self.locator.tick_values(min, max)

            let mut y_vec = Vec::<(f32, f32)>::new();

            for y in self.axis.locator.tick_values(min, max).iter() {
                y_vec.push((*y, ((y - vmin) * c_height / v_height).round()));
            }

            y_vec
        }
    }

    pub(crate) fn draw(
        &mut self, 
        renderer: &mut dyn Renderer,
        data: &DataFrame,
        to_canvas: &ToCanvas<Canvas>,
        style: &dyn PathOpt,
    ) -> renderer::Result<f32> {
        let pos = data.pos();

        let mut x = if self.is_left { pos.xmin() } else { pos.xmax() };
        let sign = if self.is_left { -1.0f32 } else { 1.0f32 };

        if let Some(patch) = &mut self.spine {
            let line_width = 1.;

            patch.set_pos(Bounds::new(
                Point(x, pos.ymin()),
                Point(x - sign * line_width, pos.ymax()),
            ));

            x += sign * line_width;

            patch.draw(renderer, to_canvas, style)?;
        }

        if self.axis.is_visible() {
            self.draw_ticks(renderer, &data, style)?;

            let width = self.major_labels.iter().map(|s| s.len()).max().unwrap();
        
            x += sign * renderer.to_px(self.axis.major().get_size());
            x += sign * renderer.to_px(self.axis.major().get_pad());
            x += sign * 0.5 * width as f32 * self.axis.major().get_label_height();
        }

        Ok(x)
    }

    fn draw_ticks(
        &mut self, 
        renderer: &mut dyn Renderer, 
        data: &DataFrame,
        style: &dyn PathOpt,
    ) -> renderer::Result<()> {
        let pos = &data.pos();

        let xv = if self.is_left { pos.xmin() } else { pos.xmax() };
        let sign: f32 = if self.is_left { -1. } else { 1. };
        let to_canvas = data.get_canvas_transform();

        for (yv, label) in self.major_ticks.iter().zip(self.major_labels.iter()) {
            let point = to_canvas.transform_point(Point(xv, *yv));

            let y = point.y();
            let mut x = xv;
            let major = self.axis.major();

            // Grid
            if self.axis.get_show_grid().is_show_major() {
                let style = major.grid_style().push(style);
                // grid
                let grid = Path::<Canvas>::new(vec![
                    PathCode::MoveTo(Point(pos.xmin(), y)),
                    PathCode::LineTo(Point(pos.xmax(), y)),
                ]);

                renderer.draw_path(&grid, &style)?;
            }

            // Tick
            {
                let style = major.tick_style().push(style);
                let tick_length = renderer.to_px(major.get_size());
                
                let tick = Path::<Canvas>::new(vec![
                    PathCode::MoveTo(Point(x + sign * tick_length, y)),
                    PathCode::LineTo(Point(x, y)),
                ]);

                renderer.draw_path(&tick, &style)?;

                x += sign * tick_length;
                x += sign * renderer.to_px(major.get_pad());
            }

            // Label
            renderer.draw_text(Point(x, y), label, 0., style, major.label_style())?;
        }

        Ok(())
    }

    pub(crate) fn update(&mut self, renderer: &mut dyn Renderer, pos: Bounds<Canvas>) {
        self.axis.resize(renderer, pos);
    }

    pub(crate) fn axis_mut(&mut self) -> &mut Axis {
        &mut self.axis
    }
}

