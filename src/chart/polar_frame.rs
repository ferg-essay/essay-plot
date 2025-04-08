use std::f32::consts::{PI, TAU};

use essay_graphics::api::{
        renderer::{Canvas, Drawable, Renderer, Result}, Affine2d, Bounds, Path, Point, Size 
    };
use essay_tensor::tensor::Tensor;

use crate::{
    artist::{
        ArtistDraw, Stale, TextCanvas
    }, 
    config::{Config, ConfigArc, PathStyle},
    palette::Palette, transform::{ToCanvas, Transform, TransformAffine}, 
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

    title: TextCanvas,

    legend: Legend,
}

impl PolarFrame {
    pub(crate) fn new(cfg: &ConfigArc) -> Self {
        let mut frame = Self {
            config: cfg.clone(),

            pos: Bounds::none(),

            data: DataFrame::new(cfg, "frame"),

            x_axis: XAxis::new(cfg, "t_axis"),
            y_axis: YAxis::new(cfg, "r_axis"),

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
    
        self.data.update_pos(renderer, &pos_data);
    
        let pos_data = self.data.get_pos().clone();
    
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

        let polar_transform = PolarTransform::new(self.data.data_bounds(), self.pos);
        let draw_to_canvas = ToCanvas::<Data>::new(
            stale,
            self.data.data_bounds(),
            &polar_transform,
        );

        self.title.draw(renderer, &frame_to_canvas, &self.path_style)?;

        renderer.draw_with_closure(self.data.get_pos(), Box::new(|r| {
            self.data.draw(r, &draw_to_canvas, &self.path_style)
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
struct PolarTransform {
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
        let pos = pos.with_aspect(1.);

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
