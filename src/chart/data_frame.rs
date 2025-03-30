use core::fmt;
use std::{any::Any, marker::PhantomData};

use essay_graphics::{
    api::{
        renderer::{Canvas, Renderer, Result}, 
        Affine2d, Bounds, Coord, PathOpt, Point
    }, 
    layout::View
};

use crate::{
    artist::{Artist, PathStyle, StyleCycle, ToCanvas}, 
    chart::{Config, ConfigArc}
};

use super::{ChartFrame, LegendHandler};

pub(crate) struct DataFrame {
    pos_canvas: Bounds<Canvas>,

    data_bounds: Bounds<Data>,
    view_bounds: Bounds<Data>,
    pan_zoom_bounds: Option<Bounds<Data>>,

    x_lim: Option<(f32, f32)>,
    y_lim: Option<(f32, f32)>,

    x_margin: Option<f32>,
    y_margin: Option<f32>,

    scaling: Scaling,
    aspect: Option<f32>,
    aspect_mode: AspectMode,
    is_flip_y: bool,

    // artists: PlotContainer,
    artist_items: Vec<ArtistItem>,

    to_canvas: Affine2d,
    style: PathStyle,
    cycle: StyleCycle,

    _is_stale: bool,
}

impl DataFrame {
    pub fn new(cfg: &Config) -> Self {
        Self {
            pos_canvas: Bounds::none(),

            data_bounds: Bounds::<Data>::unit(),
            view_bounds: Bounds::<Data>::unit(),
            pan_zoom_bounds: None,

            x_lim: None,
            y_lim: None,
            x_margin: cfg.get_as_type("frame", "x_margin"),
            y_margin: cfg.get_as_type("frame", "y_margin"),
            scaling: Scaling::Auto,
            aspect: None,
            aspect_mode: AspectMode::BoundingBox,
            is_flip_y: false,

            // artists: PlotContainer::new(cfg),
            artist_items: Vec::new(),

            style: PathStyle::default(),
            cycle: StyleCycle::from_config(cfg, "frame.cycle"),

            to_canvas: Affine2d::eye(),
            _is_stale: true,
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

    pub fn xlim(&mut self, x_min: f32, x_max: f32) -> &mut Self {
        assert!(x_min < x_max);

        self.x_lim = Some((x_min, x_max));

        self
    }

    pub fn ylim(&mut self, y_min: f32, y_max: f32) -> &mut Self {
        assert!(y_min < y_max);

        self.y_lim = Some((y_min, y_max));

        self
    }

    pub(crate) fn add_artist<A: PlotArtist + 'static>(
        &mut self, 
        artist: A,
        config: &ConfigArc,
        view: View<ChartFrame>,
    ) -> A::Opt {
        let id = self.artist_items.len();
        let view = ArtistView::new(view.clone(), id);

        let mut artist = artist;

        let opt = artist.config(config, view);

        self.artist_items.push(ArtistItem {
            any: Box::new(artist),
            handle: Box::new(ArtistHandle::<Data, A>::new()),
        });

        opt
    }

    pub(super) fn update_pos(&mut self, renderer: &mut dyn Renderer, pos: &Bounds<Canvas>) {
        for item in &mut self.artist_items {
            item.resize(renderer, pos);
        }

        self.data_bounds = self.bounds();
    
        self.update_view();

        self.pos_canvas = pos.clone();

        self.update_aspect();

        self.to_canvas = self.get_view_bounds().affine_to(&self.pos_canvas);

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

        if let Some(xlim) = self.x_lim {
            xmin = xlim.0;
            xmax = xlim.1;
        }

        if let Some(ylim) = self.y_lim {
            ymin = ylim.0;
            ymax = ylim.1;
        }

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

    pub(crate) fn get_pos(&self) -> &Bounds<Canvas> {
        &self.pos_canvas
    }

    pub(crate) fn get_view_bounds(&self) -> &Bounds<Data> {
        if let Some(bounds) = &self.pan_zoom_bounds {
            bounds
        } else {
            &self.view_bounds
        }
    }

    pub(crate) fn get_canvas_transform(&self) -> &Affine2d {
        &self.to_canvas
    }

    /*
    pub(crate) fn artist<A>(&self, id: ArtistId) -> &A
    where
        A: Artist<Data> + 'static
    {
        self.artists.artist(id)
    }

    pub(crate) fn artist_mut<A>(&mut self, id: ArtistId) -> &mut A
    where
        A: Artist<Data> + 'static
    {
        self.artists.artist_mut(id)
    }
    */

    pub(crate) fn get_handlers(&mut self) -> Vec<LegendHandler> {
        let mut vec = Vec::<LegendHandler>::new();

        for item in &mut self.artist_items {
            match item.get_legend() {
                Some(handler) => vec.push(handler),
                None => {},
            };
        }

        vec
    }

    // pub(crate) fn get_handlers(&mut self) -> Vec<LegendHandler> {
    //     self.artists.get_legend_handlers()
    // }
    
    fn bounds(&mut self) -> Bounds<Data> {
        let mut bounds = Bounds::none();

        for item in &mut self.artist_items {
            bounds = if bounds.is_none() {
                item.get_extent()
            } else {
                let extent = item.get_extent();
                if extent.is_none() { bounds } else { bounds.union(&extent) }
            }
        }

        if bounds.is_none() {
            Bounds::new(Point(0., 0.), Point(1., 1.))
        } else {
            bounds
        }
    }

    /*
    // true if request redraw
    fn event(&mut self, _renderer: &mut dyn Renderer, event: &Event) -> bool {
        match event {
            Event::ResetView(_) => {
                //self.update_view();
                self._is_stale = true;
                self.pan_zoom_bounds = None;
                true
            }
            Event::Pan(_p_start, p_last, p_now) => {
                let to_data = self.pos_canvas.affine_to(&self.get_view_bounds());
                let p0 = to_data.transform_point(*p_last);
                let p1 = to_data.transform_point(*p_now);

                let dx = p0.x() - p1.x();
                let dy = p0.y() - p1.y();

                let view = &self.get_view_bounds();
                self.pan_zoom_bounds = Some(Bounds::new(
                    Point(
                        view.x0() + dx,
                        view.y0() + dy,
                    ),
                    Point(
                        view.x1() + dx,
                        view.y1() + dy,
                    )
                ));

                true
            },
            Event::ZoomBounds(p_start, p_now) => {
                if self.pos_canvas.contains(*p_now) {
                    let to_data = self.pos_canvas.affine_to(&self.get_view_bounds());
                    let p0 = to_data.transform_point(*p_start);
                    let p1 = to_data.transform_point(*p_now);

                    // let view = &self.view_bounds;
                    // TODO: check min size?
                    self.pan_zoom_bounds = Some(Bounds::new(p0, p1));
                }

                true
            },
            _ => { false }
        }
    }
    */
}

impl Artist<Canvas> for DataFrame {
    fn bounds(&mut self) -> Bounds<Canvas> {
        self.pos_canvas.clone()
    }

    fn draw(
        &mut self, 
        renderer: &mut dyn Renderer, 
        to_canvas: &ToCanvas,
        style: &dyn PathOpt,
    ) -> Result<()> {
        // self.resize(renderer);
        // TODO: 
        renderer.flush();
        //let to_canvas = to_canvas.matmul(&self.to_canvas);
        //let to_canvas = &self.to_canvas;
        let style = self.style.push(style);
        // let clip = Clip::Bounds(self.pos_canvas.p0(), self.pos_canvas.p1());

        for (i, item) in self.artist_items.iter_mut().enumerate() {
            let style = self.cycle.push(&style, i);

            item.draw(renderer, to_canvas, &style)?;
        }

        //renderer.flush(&clip);
        renderer.flush();

        Ok(())

        // TODO: intersect clip
        //for artist in &mut self.artists {
        //    artist.draw(renderer, &to_canvas, &self.pos_canvas, &style);
        //}
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

struct ArtistItem {
    any: Box<dyn Any + Send>,
    handle: Box<dyn ArtistHandleTrait<Data>>,
}

impl ArtistItem {
    #[inline]
    fn deref<A: PlotArtist + 'static>(&self) -> &A {
        self.any.downcast_ref().unwrap()
    }

    #[inline]
    fn deref_mut<A: PlotArtist + 'static>(&mut self) -> &mut A {
        self.any.downcast_mut().unwrap()
    }

    #[inline]
    fn get_extent(&mut self) -> Bounds<Data> {
        self.handle.get_extent(&mut self.any)
    }

    #[inline]
    fn get_legend(&mut self) -> Option<LegendHandler> {
        self.handle.get_legend(&mut self.any)
    }

    #[inline]
    fn draw(
        &mut self, 
        renderer: &mut dyn Renderer, 
        to_canvas: &ToCanvas,
        style: &dyn PathOpt) -> Result<()> {
        self.handle.draw(&mut self.any, renderer, to_canvas, style)
    }

    #[inline]
    fn resize(
        &mut self, 
        _renderer: &mut dyn Renderer, 
        _pos: &Bounds<Canvas>) {
        // self.handle.resize(&mut self.any, renderer, pos);
    }
}

pub struct ArtistView<A: PlotArtist> {
    view: View<ChartFrame>,
    id: usize,
    marker: PhantomData<fn(A)>
}

impl<A: PlotArtist + 'static> ArtistView<A> {
    pub(crate) fn new(
        view: View<ChartFrame>, 
        id: usize,
    ) -> Self {
        Self {
            view,
            id,
            marker: Default::default(),
        }
    }

    pub fn read<R>(&self, fun: impl FnOnce(&A) -> R) -> R {
        self.view.read(|f| {
            fun(f.data().artist_items[self.id].deref())
        })
    }

    pub fn write<R>(&mut self, fun: impl FnOnce(&mut A) -> R) -> R {
        self.view.write(|f| {
            fun(f.data_mut().artist_items[self.id].deref_mut())
        })
    }
}

impl<A: PlotArtist> Clone for ArtistView<A> {
    fn clone(&self) -> Self {
        Self { 
            view: self.view.clone(), 
            id: self.id.clone(), 
            marker: self.marker.clone() 
        }
    }
}

trait ArtistHandleTrait<M: Coord> : Send {
    // fn resize(&self, any: &mut Box<dyn Any + Send>, renderer: &mut dyn Renderer, pos: &Bounds<Canvas>);
    fn get_extent(&self, any: &mut Box<dyn Any + Send>) -> Bounds<M>;
    fn get_legend(&self, any: &mut Box<dyn Any + Send>) -> Option<LegendHandler>;

    fn draw(
        &self, 
        artist_any: &mut Box<dyn Any + Send>,
        renderer: &mut dyn Renderer,
        to_canvas: &ToCanvas,
        style: &dyn PathOpt,
    ) -> Result<()>;
}

struct ArtistHandle<M: Coord, A: Artist<M>> {
    marker: PhantomData<fn(M, A)>,
}

impl<M: Coord, A: Artist<M>> ArtistHandle<M, A> {
    fn new() -> Self {
        Self {
            marker: PhantomData,
        }
    }
}

impl<A: Artist<Data>> ArtistHandleTrait<Data> for ArtistHandle<Data, A>
where
    A: PlotArtist + 'static,
{
    /*
    fn resize(&self, any: &mut Box<dyn Any + Send>, renderer: &mut dyn Renderer, pos: &Bounds<Canvas>) {
        // any.downcast_mut::<A>().unwrap().update_pos(renderer, pos);
    }
    */

    fn get_extent(&self, any: &mut Box<dyn Any + Send>) -> Bounds<Data> {
        any.downcast_mut::<A>().unwrap().bounds()
    }

    fn draw(
        &self, 
        artist_any: &mut Box<dyn Any + Send + 'static>,
        renderer: &mut dyn Renderer,
        to_canvas: &ToCanvas,
        style: &dyn PathOpt,
    ) -> Result<()> {
        let artist = artist_any.downcast_mut::<A>().unwrap();
        artist.draw(renderer, to_canvas, style)
    }

    fn get_legend(&self, any: &mut Box<dyn Any + Send>) -> Option<LegendHandler> {
        let artist = any.downcast_mut::<A>().unwrap();
        artist.get_legend()
    }
}

pub trait PlotArtist : Artist<Data> + Sized {
    type Opt : Clone;
    
    fn config(
        &mut self, 
        cfg: &ConfigArc, 
        view: ArtistView<Self>,
    ) -> Self::Opt;

    fn get_legend(&self) -> Option<LegendHandler> {
        None
    }
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
