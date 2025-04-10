use std::f32::consts::TAU;

use essay_graphics::api::{
        renderer::{Canvas, Drawable, Renderer, Result}, 
        Affine2d, Bounds, Path, Point 
};

use crate::{
    artist::{
        paths, ArtistDraw, Stale, TextCanvas
    }, 
    config::{ConfigArc, PathStyle}, 
    palette::Palette, 
    transform::{AngleCoord, CartesianTransform, PolarTransform, ToCanvas, Transform, TransformAffine} 
};

use super::{
    axis::{Axis, AxisTicks}, 
    cartesian_frame::{FrameMargins, FrameWithTextArtist}, 
    data_frame::DataFrame, 
    legend::Legend, 
    polar_axis::{PolarXAxis, PolarYAxis}, 
    Data, FrameArtist, Scaling
};

pub struct PolarFrame {
    pos: Bounds<Canvas>,

    config: ConfigArc,

    to_canvas: Affine2d,

    margins: FrameMargins,

    path_style: PathStyle,

    data: DataFrame,

    x_axis: PolarXAxis,
    y_axis: PolarYAxis,

    angle_coord: AngleCoord,

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

            x_axis: PolarXAxis::new(cfg, "axis"),
            y_axis: PolarYAxis::new(cfg, "axis"),

            angle_coord: AngleCoord::Degrees,

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

    pub(crate) fn _get_text_mut(&mut self, artist: FrameArtist) -> &mut TextCanvas {
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

    pub(crate) fn angle_coord(&mut self, angle_coord: AngleCoord) {
        self.angle_coord = angle_coord;
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
    
        let pos_data = Bounds::<Canvas>::new(
            Point(pos.xmin(), pos.ymin()), 
            Point(pos.xmax(), pos.ymax()),
        );
        let pos_data = pos_data.with_aspect(1.);

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
    
        self.data.update_pos(renderer, &pos_data);

        self.update_axis();
    
        let pos_data = self.data.pos();
    
        let pos_legend = Bounds::<Canvas>::new(
            Point(pos_data.xmin(), pos_data.ymax()),
            Point(pos_data.xmin(), pos_data.ymax()),
        );
        self.legend.resize(renderer, &pos_legend);
    
        // TODO:
        self.x_axis.resize(&self.data, self.angle_coord);
        self.y_axis.resize(&self.data, self.angle_coord);
    
        self.legend.update_handlers(&mut self.data);
    }

    fn update_axis(&mut self) {
        let mut major = Vec::new();

        let bounds: Bounds<Data> = ([-1., -1.], [2., 2.]).into();
        let transform = CartesianTransform::bounds_to(bounds, self.data.pos());

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

        let polar_transform = PolarTransform::new(
            self.data.data_bounds(), 
            self.data.pos(),
            self.angle_coord,
        );

        let draw_to_canvas = ToCanvas::<Data>::new(
            stale,
            self.data.data_bounds(),
            &polar_transform,
        );

        self.title.draw(renderer, &frame_to_canvas, &self.path_style)?;

        self.y_axis.draw(renderer, &self.path_style)?;
        self.x_axis.draw(renderer, &self.path_style)?;

        renderer.draw_with_closure(self.data.pos(), Box::new(|ui| {
            // self.draw_axis(ui)?;
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
