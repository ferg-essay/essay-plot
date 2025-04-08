use std::{any::Any, marker::PhantomData, sync::{Arc, Mutex}};

use essay_graphics::api::{
    renderer::{self, Canvas, Renderer, Result}, Affine2d, Bounds, Coord, Path, PathOpt, Point
};
use essay_tensor::tensor::Tensor;

use crate::chart::LegendHandler;
use crate::config::{ConfigArc, StyleCycle};

pub trait ArtistDraw<M: Coord> : Send {
    fn bounds(&mut self) -> Bounds<M>;

    fn draw(
        &mut self, 
        renderer: &mut dyn Renderer,
        to_canvas: &ToCanvas,
        style: &dyn PathOpt,
    ) -> Result<()>;
}

pub trait Artist<M: Coord> : ArtistDraw<M> + Send + Sized {
    type Opt : Clone;
    
    fn config(&mut self, cfg: &ConfigArc);

    fn opt(&mut self, view: ArtistView<M, Self>) -> Self::Opt;

    fn get_legend(&self) -> Option<LegendHandler> {
        None
    }
}

pub trait IntoArtist<M: Coord> {
    type Artist : Artist<M>;

    fn into_artist(self) -> Self::Artist;
}

impl<M: Coord, A: Artist<M>> IntoArtist<M> for A {
    type Artist = Self;

    fn into_artist(self) -> Self::Artist {
        self
    }
}

pub struct ArtistContainer<M: Coord> {
    artists: ContainerArc<M>,

    config: ConfigArc,
    _prefix: String,
    
    cycle: StyleCycle,
}

impl<M: Coord> ArtistContainer<M> {
    pub fn from_config(config: &ConfigArc, prefix: &str) -> Self {
        Self {
            artists: ContainerArc(Arc::new(Mutex::new(Vec::new()))),

            cycle: StyleCycle::from_config(config, &format!("{}.cycle", prefix)),

            config: config.clone(),
            _prefix: String::from(prefix),
        }
    }

    pub fn cycle(&mut self, cycle: impl Into<StyleCycle>) {
        self.cycle = cycle.into();
    }

    pub fn add<A: Artist<M> + 'static>(&mut self, mut artist: A) -> A::Opt {
        artist.config(&self.config); // todo: prefix

        let index = self.artists.0.lock().unwrap().len();

        let view = ArtistView::<M, A> {
            artists: self.artists.clone(),
            index,
            marker: Default::default(),
        };

        let opt = artist.opt(view);

        let item = ArtistItem::new(artist);

        self.artists.0.lock().unwrap().push(item);

        opt
    }
    
    pub fn bounds(&mut self, bounds: Bounds<M>) -> Bounds<M> {
        let mut artists = self.artists.0.lock().unwrap();

        let mut bounds = bounds;

        for item in artists.iter_mut() {
            let sub_bounds = item.get_bounds();

            bounds = if bounds.is_none() {
                sub_bounds
            } else if sub_bounds.is_none() { 
                bounds
            } else { 
                bounds.union(&sub_bounds) 
            }
        }

        bounds
    }

    pub fn draw(
        &mut self, 
        renderer: &mut dyn Renderer,
        to_canvas: &ToCanvas,
        style: &dyn PathOpt
    ) -> renderer::Result<()> {
        let mut vec = self.artists.0.lock().unwrap();

        let len = vec.len();

        for (i, item) in vec.iter_mut().enumerate() {
            let style = self.cycle.push(style, i, len);

            item.draw(renderer, to_canvas, &style)?;
        }

        Ok(())
    }

    pub fn get_handlers(&mut self) -> Vec<LegendHandler> {
        let mut vec = self.artists.0.lock().unwrap();

        vec.iter_mut().filter_map(|item| item.get_legend()).collect()

        /*
        let mut vec = Vec::<LegendHandler>::new();

        for item in &mut self.artist_items {
            match item.get_legend() {
                Some(handler) => vec.push(handler),
                None => {},
            };
        }

        vec
        */
    }
}

struct ContainerArc<M: Coord>(Arc<Mutex<Vec<ArtistItem<M>>>>);

impl<M: Coord> Clone for ContainerArc<M> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

pub struct ArtistView<M: Coord, A: Artist<M>> {
    artists: ContainerArc<M>,
    index: usize,
    marker: PhantomData<(fn(M), fn(A))>,
}

impl<M: Coord, A: Artist<M> + 'static> ArtistView<M, A> {
    pub fn read<R>(&self, fun: impl FnOnce(&A) -> R) -> R {
        let artist = &self.artists.0.lock().unwrap()[self.index];

        fun(artist.deref())
    }

    pub fn write<R>(&mut self, fun: impl FnOnce(&mut A) -> R) -> R {
        let artist = &mut self.artists.0.lock().unwrap()[self.index];

        fun(artist.deref_mut())
    }
}

impl<M: Coord, A: Artist<M>> Clone for ArtistView<M, A> {
    fn clone(&self) -> Self {
        Self { 
            artists: self.artists.clone(), 
            index: self.index.clone(), 
            marker: Default::default(),
        }
    }
}

struct ArtistItem<M: Coord> {
    any: Box<dyn Any + Send>,
    handle: Box<dyn ArtistHandleTrait<M>>,
}

impl<M: Coord> ArtistItem<M> {
    pub fn new<A>(artist: A) -> Self
    where
        A: Artist<M> + 'static
    {
        Self {
            any: Box::new(artist),
            handle: Box::new(ArtistHandle::<M, A>::new()),
        }
    }

    #[inline]
    pub fn deref<A: Artist<M> + 'static>(&self) -> &A {
        self.any.downcast_ref().unwrap()
    }

    #[inline]
    pub fn deref_mut<A: Artist<M> + 'static>(&mut self) -> &mut A {
        self.any.downcast_mut().unwrap()
    }

    #[inline]
    pub fn get_bounds(&mut self) -> Bounds<M> {
        self.handle.get_bounds(&mut self.any)
    }

    #[inline]
    pub fn get_legend(&mut self) -> Option<LegendHandler> {
        self.handle.get_legend(&mut self.any)
    }

    #[inline]
    pub fn draw(
        &mut self, 
        renderer: &mut dyn Renderer, 
        to_canvas: &ToCanvas,
        style: &dyn PathOpt
    ) -> renderer::Result<()> {
        self.handle.draw(&mut self.any, renderer, to_canvas, style)
    }
}

trait ArtistHandleTrait<M: Coord> : Send {
    fn get_bounds(&self, any: &mut Box<dyn Any + Send>) -> Bounds<M>;
    fn get_legend(&self, any: &mut Box<dyn Any + Send>) -> Option<LegendHandler>;

    fn draw(
        &self, 
        artist_any: &mut Box<dyn Any + Send>,
        renderer: &mut dyn Renderer,
        to_canvas: &ToCanvas,
        style: &dyn PathOpt,
    ) -> renderer::Result<()>;
}

struct ArtistHandle<M: Coord, A: ArtistDraw<M>> {
    marker: PhantomData<fn(M, A)>,
}

impl<M: Coord, A: ArtistDraw<M>> ArtistHandle<M, A> {
    fn new() -> Self {
        Self {
            marker: PhantomData,
        }
    }
}

impl<M: Coord, A> ArtistHandleTrait<M> for ArtistHandle<M, A>
where
    A: Artist<M> + 'static,
{
    fn get_bounds(&self, any: &mut Box<dyn Any + Send>) -> Bounds<M> {
        any.downcast_mut::<A>().unwrap().bounds()
    }

    fn draw(
        &self, 
        artist_any: &mut Box<dyn Any + Send + 'static>,
        renderer: &mut dyn Renderer,
        to_canvas: &ToCanvas,
        style: &dyn PathOpt,
    ) -> renderer::Result<()> {
        let artist = artist_any.downcast_mut::<A>().unwrap();
        artist.draw(renderer, to_canvas, style)
    }

    fn get_legend(&self, any: &mut Box<dyn Any + Send>) -> Option<LegendHandler> {
        let artist = any.downcast_mut::<A>().unwrap();
        artist.get_legend()
    }
}


pub struct ToCanvas {
    id: Stale,
    pos_frame: Bounds<Canvas>,
    to_canvas: Affine2d,
}

impl ToCanvas {
    pub fn new(pos_frame: Bounds<Canvas>, to_canvas: Affine2d) -> Self {
        Self {
            id: Stale::stale(),
            pos_frame,
            to_canvas
        }
    }

    #[inline]
    pub fn id(&self) -> Stale {
        self.id
    }

    #[inline]
    pub fn pos(&self) -> Bounds<Canvas> {
        self.pos_frame
    }

    // todo: fix Coord
    pub fn transform_path<M: Coord>(&self, path: &Path<M>) -> Path<Canvas> {
        path.map(|point| { self.transform_point(point) })
    }

    #[inline]
    pub fn transform_point(&self, point: Point) -> Point {
        self.to_canvas.transform_point(point)
    }

    #[inline]
    pub fn transform_tensor(&self, tensor: &Tensor) -> Tensor {
        tensor.map_row(|v: &[f32]| {
            let Point(x, y) = self.to_canvas.transform_point(Point(v[0], v[1]));

            [x, y]
        })
    }
    
    #[deprecated]
    pub(crate) fn affine2d(&self) -> &Affine2d{
        &self.to_canvas
    }
    
    pub(crate) fn matmul(&self, transform: &Affine2d) -> Self {
        Self {
            id: self.id, // todo: update id
            pos_frame: self.pos_frame,
            to_canvas: self.to_canvas.matmul(transform),
        }
    }
    
    /*
    pub fn to_canvas(&self) -> &Affine2d {
        &self.to_canvas
    }
    */
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Stale(u64);

impl Stale {
    const STALE: Stale = Stale(u64::MAX);

    #[inline]
    pub fn new_for_update() -> Self {
        Self(0)
    }

    #[inline]
    pub fn stale() -> Self {
        Self::STALE
    }

    #[inline]
    pub fn is_stale(&self) -> bool {
        *self == Self::STALE
    }

    #[inline]
    pub fn update(self) -> Self {
        // invalid to call update on a stale value, which should cause a
        // panic here
        Self(self.0 + 1)
    }

    #[inline]
    pub fn eq_or(&self, stale: Self, f: impl FnOnce()) -> Self {
        if *self != stale || self.is_stale() {
            (f)()
        }

        *self
    }
}

impl Default for Stale {
    fn default() -> Self {
        Self::stale()
    }
}
