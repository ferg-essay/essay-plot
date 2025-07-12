use core::fmt;

use essay_graphics::api::{
    renderer::{Canvas, Renderer, Result}, 
    Affine2d, Bounds, Coord, PathOpt, Point
};

use crate::{
    artist::{Artist, ArtistContainer, ArtistDraw, Stale}, config::{ConfigArc, PathStyle}, palette::Palette, transform::ToCanvas
};

use super::LegendHandler;

pub(crate) struct DataFrame {
    pos_canvas: Bounds<Canvas>,

    data_bounds: Bounds<Data>,
    view_bounds: Bounds<Data>,
    pan_zoom_bounds: Option<Bounds<Data>>,

    x_min: Option<f32>,
    x_max: Option<f32>,
    y_min: Option<f32>,
    y_max: Option<f32>,

    x_margin: Option<f32>,
    y_margin: Option<f32>,

    scaling: Scaling,
    aspect: Option<f32>,
    aspect_mode: AspectMode,
    is_flip_y: bool,

    artist_items: ArtistContainer<Data>,

    to_canvas: Affine2d,
    style: PathStyle,

    _is_stale: bool,
    stale_data_bounds: Stale,
}

impl DataFrame {
    pub fn new(cfg: &ConfigArc, prefix: &str) -> Self {
        Self {
            pos_canvas: Bounds::none(),

            data_bounds: Bounds::<Data>::unit(),
            view_bounds: Bounds::<Data>::unit(),
            pan_zoom_bounds: None,

            x_min: None,
            x_max: None,
            y_min: None,
            y_max: None,
            x_margin: cfg.get_as_type(prefix, "x_margin"),
            y_margin: cfg.get_as_type(prefix, "y_margin"),
            scaling: Scaling::Auto,
            aspect: None,
            aspect_mode: AspectMode::BoundingBox,
            is_flip_y: false,

            // artists: PlotContainer::new(cfg),
            artist_items: ArtistContainer::from_config(cfg, prefix),

            style: PathStyle::default(),

            to_canvas: Affine2d::eye(),
            _is_stale: true,

            stale_data_bounds: Stale::new_for_update(),
        }
    }

    pub fn scaling(&mut self, scaling: Scaling) -> &mut Self {
        self.scaling = scaling;

        self
    }

    pub fn aspect(&mut self, aspect: f32) -> &mut Self {
        self.aspect = Some(aspect);

        self
    }

    pub fn aspect_mode(&mut self, mode: AspectMode) -> &mut Self {
        self.aspect_mode = mode;

        self
    }

    pub fn flip_y(&mut self, is_flip_y: bool) -> &mut Self {
        self.is_flip_y = is_flip_y;

        self
    }

    pub fn xlim(&mut self, x_min: Option<f32>, x_max: Option<f32>) -> &mut Self {
        if let Some(x_min) = x_min {
            if let Some(x_max) = x_max {
                assert!(x_min < x_max);
            }
        }

        self.x_min = x_min;
        self.x_max = x_max;

        self
    }

    pub fn ylim(&mut self, y_min: Option<f32>, y_max: Option<f32>) -> &mut Self {
        if let Some(y_min) = y_min {
            if let Some(y_max) = y_max {
                assert!(y_min < y_max);
            }
        }

        self.y_min = y_min;
        self.y_max = y_max;

        self
    }
    
    pub(crate) fn color_cycle(&mut self, cycle: impl Into<Palette>) {
        let cycle = cycle.into();

        self.artist_items.cycle(cycle);
    }

    pub(crate) fn add_artist<A: Artist<Data> + 'static>(
        &mut self, 
        artist: A,
        config: &ConfigArc,
    ) -> A::Opt {
        let mut artist = artist;

        artist.config(config);

        self.artist_items.add(artist)
    }

    pub(super) fn update_pos(&mut self, _renderer: &mut dyn Renderer, pos: &Bounds<Canvas>) {
        self.update_data_bounds();
    
        self.update_view();

        self.pos_canvas = pos.clone();

        self.update_aspect();

        self.to_canvas = self.data_bounds().affine_to(&self.pos_canvas);

        if self.is_flip_y {
            self.to_canvas = self.to_canvas
                .translate(0., - self.pos_canvas.ymin())
                .scale(1., -1.)
                .translate(0., self.pos_canvas.ymax());
        }
    }

    fn update_view(&mut self) {
        let data = &self.data_bounds;

        let (height, width) = (data.height(), data.width());

        let (mut xmin, mut xmax) = (data.xmin(), data.xmax());
        let (mut ymin, mut ymax) = (data.ymin(), data.ymax());

        match self.scaling {
            Scaling::Auto => {
                if self.aspect.is_none() {
                    if let Some(x_margin) = self.x_margin {
                        xmin -= x_margin * width;
                        xmax += x_margin * width;
                    }
        
                    if let Some(y_margin) = self.y_margin {
                        ymin -= y_margin * height;
                        ymax += y_margin * height;
                    }
                }
            }
            Scaling::Image => {
            }
        }

        // single point
        if xmin == xmax {
            xmin = xmin - 1.;
            xmax = xmax + 1.;
        }

        if ymin == ymax {
            ymin = ymin - 1.;
            ymax = ymax + 1.;
        }

        xmin = self.x_min.unwrap_or(xmin);
        xmax = self.x_max.unwrap_or(xmax);

        ymin = self.y_min.unwrap_or(ymin);
        ymax = self.y_max.unwrap_or(ymax);

        self.view_bounds = Bounds::new(Point(xmin, ymin), Point(xmax, ymax));
        // pos.clone()
    }

    fn update_aspect(&mut self) {
        match self.aspect_mode {
            AspectMode::BoundingBox => self.update_aspect_pos(),
            AspectMode::View => self.update_aspect_view(),
        }
    }

    fn update_aspect_view(&mut self) {
        if let Some(_aspect) = self.aspect {
            let mut bounds = self.view_bounds.clone();

            if bounds.height() < bounds.width() {
                let ymid = bounds.ymid();
                let h2 = bounds.width() * 0.5;

                bounds = Bounds::new(
                    Point(bounds.xmin(), ymid - h2),
                    Point(bounds.xmax(), ymid + h2),
                );
            } else {
                let w2 = bounds.height() * 0.5;

                bounds = Bounds::new(
                    Point(bounds.xmid() - w2, bounds.ymin()),
                    Point(bounds.xmid() + w2, bounds.ymax()),
                );
            }

            self.view_bounds = bounds;
        }
    }

    fn update_aspect_pos(&mut self) {
        if let Some(_aspect) = self.aspect {
            let view_ratio = self.view_bounds.width() / self.view_bounds.height().max(f32::EPSILON);
            let pos = &self.pos_canvas;
            let pos_ratio = pos.width() / pos.height().max(f32::EPSILON);

            let pos = if pos_ratio < view_ratio {
                let h2 = pos.width() * 0.5 / view_ratio;

                Bounds::new(
                    Point(pos.xmin(), pos.ymid() - h2),
                    Point(pos.xmax(), pos.ymid() + h2),
                )
            } else {
                let w2 = pos.height() * 0.5 * view_ratio;

                Bounds::new(
                    Point(pos.xmid() - w2, pos.ymin()),
                    Point(pos.xmid() + w2, pos.ymax()),
                )
            };

            self.pos_canvas = pos;
        }
    }

    pub(crate) fn pos(&self) -> Bounds<Canvas> {
        self.pos_canvas
    }

    pub(crate) fn data_bounds(&self) -> Bounds<Data> {
        self.pan_zoom_bounds.unwrap_or(self.view_bounds)
    }

    pub(crate) fn get_handlers(&mut self) -> Vec<LegendHandler> {
        self.artist_items.get_handlers()
    }
    
    fn update_data_bounds(&mut self) {
        let bounds = self.artist_items.bounds(Bounds::none())
            .or(Bounds::unit());

        if bounds != self.data_bounds {
            self.stale_data_bounds = self.stale_data_bounds.update();
        }

        self.data_bounds = bounds;
    }
}

impl ArtistDraw<Data> for DataFrame {
    fn bounds(&mut self) -> Bounds<Data> {
        // self.pos_canvas.clone()
        todo!()
    }

    fn draw(
        &mut self, 
        renderer: &mut dyn Renderer, 
        to_canvas: &ToCanvas<Data>,
        style: &dyn PathOpt,
    ) -> Result<()> {
        // self.resize(renderer);
        // TODO: 
        renderer.flush();
        //let to_canvas = to_canvas.matmul(&self.to_canvas);
        //let to_canvas = &self.to_canvas;
        let style = self.style.push(style);
        // let clip = Clip::Bounds(self.pos_canvas.p0(), self.pos_canvas.p1());

        self.artist_items.draw(renderer, to_canvas, &style)?;

        renderer.flush();

        Ok(())
    }
}


impl fmt::Debug for DataFrame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DataBox({},{}; {}x{})",
            self.view_bounds.xmin(),
            self.view_bounds.ymin(),
            self.view_bounds.width(),
            self.view_bounds.height())
    }
}

pub enum AspectMode {
    BoundingBox,
    View
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Scaling {
    Auto,
    Image
}
///
/// Data coordinates
///
#[derive(Clone, Copy, Debug)]
pub struct Data;

impl Coord for Data {
}
