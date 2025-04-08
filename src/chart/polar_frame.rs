use std::f32::consts::{PI, TAU};

use essay_graphics::api::{
        color::Grey, renderer::{self, Canvas, Drawable, Renderer, Result}, Affine2d, Bounds, Color, Path, Point, Size 
    };
use essay_tensor::tensor::Tensor;

use crate::{
    artist::{
        paths, ArtistDraw, Stale, TextCanvas
    }, chart::cartesian_frame::CartesianTransform, config::{Config, ConfigArc, PathStyle}, palette::Palette, transform::{ToCanvas, Transform, TransformAffine} 
};

use super::{
    axis::{Axis, AxisTicks, XAxis, YAxis}, cartesian_frame::{FrameMargins, FrameWithTextArtist}, data_frame::DataFrame, legend::Legend, Data, FrameArtist, Scaling
};

pub struct PolarFrame {
    pos: Bounds<Canvas>,

    config: ConfigArc,

    to_canvas: Affine2d,

    margins: FrameMargins,

    path_style: PathStyle,

    data: DataFrame,

    x_axis: XAxis,
    y_axis: YAxis,

    x_rays: Vec<Path<Canvas>>,
    y_circles: Vec<Path<Canvas>>,

    title: TextCanvas,

    legend: Legend,
}

impl PolarFrame {
    pub(crate) fn new(cfg: &ConfigArc) -> Self {
        let mut frame = Self {
            config: cfg.clone(),

            pos: Bounds::none(),

            data: DataFrame::new(cfg, "polar"),

            x_axis: XAxis::new(cfg, "t_axis"),
            y_axis: YAxis::new(cfg, "r_axis"),

            x_rays: Vec::new(),
            y_circles: Vec::new(),

            title: TextCanvas::new(),

            margins: FrameMargins::new(cfg),

            path_style: PathStyle::default(),

            to_canvas: Affine2d::eye(),

            legend: Legend::new(cfg),
        };

        frame.data.scaling(Scaling::Image);

        frame
    }

    pub(crate) fn config(&self) -> &ConfigArc {
        &self.config
    }

    pub(crate) fn data_mut(&mut self) -> &mut DataFrame {
        &mut self.data
    }

    pub(crate) fn get_text_mut(&mut self, artist: FrameArtist) -> &mut TextCanvas {
        match artist {
            FrameArtist::Title => &mut self.title,

            _ => panic!("Invalid text {:?}", artist)
        }
    }

    pub(crate) fn get_axis_mut(&mut self, artist: FrameArtist) -> &mut Axis {
        match artist {
            FrameArtist::X => self.x_axis.axis_mut(),
            FrameArtist::Y => self.y_axis.axis_mut(),

            _ => panic!("Invalid axis {:?}", artist)
        }
    }

    pub(crate) fn get_ticks_mut(&mut self, artist: FrameArtist) -> &mut AxisTicks {
        match artist {
            FrameArtist::XMajor => self.x_axis.axis_mut().major_mut(),
            // FrameArtist::XMinor => self.x_axis.axis_mut().minor_mut(),
            FrameArtist::YMajor => self.y_axis.axis_mut().major_mut(),
            // FrameArtist::YMinor => self.left.axis_mut().minor_mut(),

            _ => panic!("Invalid axis-texts {:?}", artist)
        }
    }

    pub(crate) fn colorbar(&mut self) {
        // self.right.colorbar();
        todo!();
    }

    pub(crate) fn color_cycle(&mut self, cycle: impl Into<Palette>) {
        self.data.color_cycle(cycle);
    }

    fn resize(&mut self, renderer: &mut dyn Renderer) {
        let pos = renderer.pos();

        let pos = Bounds::from([
            [pos.xmin() + pos.width() * self.margins.left,
            pos.ymin() + pos.height() * self.margins.top],
            [pos.xmin() + pos.width() * self.margins.right,
            pos.ymin() + pos.height() * self.margins.bottom],
        ]);
    
        self.pos = pos.clone();
    
        let title = self.title.bounds();
    
        // title exists outside the pos bounds
        self.title.update_pos(
            renderer,
            Bounds::from([
                [pos.xmin(), pos.ymax()], 
                [pos.xmax(), pos.ymax() + title.height()]
            ])
        );
    
        let pos_data = Bounds::<Canvas>::new(
            Point(pos.xmin(), pos.ymin()), 
            Point(pos.xmax(), pos.ymax()),
        );
        let pos_data = pos_data.with_aspect(1.);

        self.data.update_pos(renderer, &pos_data);

        self.update_axis();
    
        let pos_data = self.data.get_pos();
    
        let pos_legend = Bounds::<Canvas>::new(
            Point(pos_data.xmin(), pos_data.ymax()),
            Point(pos_data.xmin(), pos_data.ymax()),
        );
        self.legend.resize(renderer, &pos_legend);
    
        // TODO:
        self.x_axis.update_axis(&self.data);
        self.y_axis.update_axis(&self.data);
    
        self.legend.update_handlers(&mut self.data);
    }

    fn update_axis(&mut self) {
        let mut major = Vec::new();

        let bounds: Bounds<Data> = ([-1., -1.], [2., 2.]).into();
        let transform = CartesianTransform::bounds_to(bounds, self.data.get_pos());

        let circle = paths::circle();

        for y in [1., 0.75, 0.5, 0.25] {
            let circle = circle.scale(y, y);
            let circle = transform.transform_path(&circle);

            major.push(circle);
        }

        self.y_circles = major;

        let mut major = Vec::new();

        let len = 6;
        for x in 0..len {
            let (sin, cos) = (x as f32 * TAU / len as f32).sin_cos();

            let path = Path::move_to(0., 0.).line_to(cos, sin).to_path();
            let path = transform.transform_path(&path);

            major.push(path);
        }

        self.x_rays = major;
    }

    fn draw_axis(
        &self, 
        ui: &mut dyn Renderer, 
    ) -> renderer::Result<()> {
        let mut style = PathStyle::new();
        style.edge_color(Grey(0.90));
        style.face_color(Color::white());
        style.line_width(2.);

        for circle in &self.y_circles {
            ui.draw_path(&circle, &style)?;
        }
        ui.flush();

        for ray in &self.x_rays {
            ui.draw_path(&ray, &style)?;
        }

        Ok(())
    }
}

impl Drawable for PolarFrame {
    fn draw(&mut self, renderer: &mut dyn Renderer) -> Result<()> {
        if self.pos != renderer.pos() {
            self.resize(renderer);
        }

        let stale = Stale::new_for_update();
        let frame_transform = TransformAffine::new(self.to_canvas.clone());
        let frame_to_canvas = ToCanvas::new(
            stale,
            self.pos,
            &frame_transform,
        );

        let polar_transform = PolarTransform::new(self.data.data_bounds(), self.data.get_pos());
        let draw_to_canvas = ToCanvas::<Data>::new(
            stale,
            self.data.data_bounds(),
            &polar_transform,
        );

        self.title.draw(renderer, &frame_to_canvas, &self.path_style)?;

        renderer.draw_with_closure(self.data.get_pos(), Box::new(|ui| {
            self.draw_axis(ui)?;
            self.data.draw(ui, &draw_to_canvas, &self.path_style)
        }))?;

        self.legend.draw(renderer, &frame_to_canvas, &self.path_style)?;

        Ok(())
    }
}

impl FrameWithTextArtist for PolarFrame {
    fn get_text_mut(&mut self, artist: FrameArtist) -> &mut TextCanvas {
        match artist {
            FrameArtist::Title => &mut self.title,

            _ => panic!("Invalid text {:?}", artist)
        }
    }
}

#[derive(Debug)]
pub struct PolarTransform {
    xf: f32,
    yf: f32,

    sx: f32,
    sy: f32,
    tx: f32,
    ty: f32,
}

impl PolarTransform {
    fn new(
        data: Bounds<Data>,
        pos: Bounds<Canvas>,
    ) -> Self {
        let ([tx, ty], [sx, sy]) = pos.into();

        let dx = data.width();
        let ymin = data.xmin();
        let ymax = data.ymax();
        let dy = ymin.abs().max(ymax.abs());

        let xform = Self {
            xf: TAU / dx.max(f32::EPSILON),
            yf: dy.max(f32::EPSILON).recip(),
            sx: sx * 0.5,
            sy: sy * 0.5,
            tx: tx + sx * 0.5,
            ty: ty + sy * 0.5,
        };

        xform
    }

    fn transform(&self, x: f32, y: f32) -> [f32; 2] {
        let (sin, cos) = (x * self.xf).sin_cos();

        [
            self.tx + self.sx * cos * y * self.yf,
            self.ty + self.sy * sin * y * self.yf,
        ]
    }
}

impl Transform<Data> for PolarTransform {
    #[inline]
    fn transform_point(&self, point: Point) -> Point {
        let Point(x, y) = point;

        self.transform(x, y).into()
    }

    #[inline]
    fn transform_tensor(&self, tensor: &Tensor) -> Tensor {
        tensor.map_row(|row| {
            self.transform(row[0], row[1])
        })
    }

    #[inline]
    fn transform_path(&self, path: &Path<Data>) -> Path<Canvas> {
        path.map(|Point(x, y)| self.transform(x, y).into())
    }
}
/*
pub struct FrameSizes {
    label_pad: f32,
}

impl FrameSizes {
    fn new(cfg: &Config) -> Self {
        Self {
            label_pad: match cfg.get_as_type("frame", "label_pad") {
                Some(pad) => pad,
                None => 0.,
            },
        }
    }
}

pub struct FrameTextOpt {
    view: View<PolarFrame>,
    artist: FrameArtist,
}

impl FrameTextOpt {
    pub(crate) fn new(view: View<PolarFrame>, artist: FrameArtist) -> Self {
        Self {
            view,
            artist,
        }
    }

    fn write(&mut self, fun: impl FnOnce(&mut TextCanvas)) {
        self.view.write(|frame| {
            fun(frame.get_text_mut(self.artist))
        })
    }

    pub fn label(&mut self, label: &str) -> &mut Self {
        self.write(|text| { text.label(label); });
        self
    }

    pub fn color(&mut self, color: impl Into<Color>) -> &mut Self {
        self.write(|text| { text.path_style_mut().color(color); });
        self
    }

    pub fn size(&mut self, size: f32) -> &mut Self {
        self.write(|text| { text.text_style_mut().size(size); });
        self
    }
}

struct FrameMargins {
    top: f32,
    bottom: f32,
    left: f32,
    right: f32,
}

impl FrameMargins {
    fn new(cfg: &Config) -> Self {
        let bottom = cfg.get_as_type("figure.subplot", "bottom")
            .unwrap_or(0.);

        let top = cfg.get_as_type("figure.subplot", "top")
            .unwrap_or(1.);

        let left = cfg.get_as_type("figure.subplot", "left")
            .unwrap_or(0.);

        let right = cfg.get_as_type("figure.subplot", "right")
            .unwrap_or(1.);

        Self {
            bottom,
            top, 
            left,
            right, 
        }
    }
}
*/
