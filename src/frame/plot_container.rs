use std::{alloc, any::TypeId, marker::PhantomData, ptr::{NonNull, self}, mem::{ManuallyDrop, self}};

use essay_graphics::api::{Coord, Bounds, driver::Renderer, Canvas, PathOpt, Clip, Point};

use crate::{artist::{Artist, StyleCycle, PlotArtist, ToCanvas}, graph::Config};

use super::{legend::LegendHandler, ArtistId, Data};

pub(crate) struct PlotContainer {
    ptrs: Vec<PlotPtr<Data>>,
    artists: Vec<Box<dyn ArtistHandleTrait<Data>>>,
    cycle: StyleCycle,
}

impl PlotContainer {
    pub(crate) fn new(cfg: &Config) -> Self {
        let container = Self {
            ptrs: Vec::new(),
            artists: Vec::new(),
            cycle: StyleCycle::from_config(cfg, "frame.cycle"),
        };

        container
    }

    pub(crate) fn add_artist<A>(&mut self, artist: A) -> ArtistId
    where
        A: PlotArtist + 'static
    {
        let id = ArtistId::new_data(self.ptrs.len());

        let plot = PlotPtr::new(id, artist);
        self.ptrs.push(plot);

        self.artists.push(Box::new(ArtistHandle::<Data, A>::new(id)));

        id
    }

    pub(crate) fn _cycle(&self) -> &StyleCycle {
        &self.cycle
    }

    pub(crate) fn _cycle_mut(&mut self) -> &mut StyleCycle {
        &mut self.cycle
    }

    fn _deref<A: Artist<Data> + 'static>(&self, id: ArtistId) -> &A {
        unsafe { self.ptrs[id.index()]._deref() }
    }

    fn deref_mut<A: Artist<Data> + 'static>(&self, id: ArtistId) -> &mut A {
        unsafe { self.ptrs[id.index()].deref_mut() }
    }

    //pub(crate) fn style_mut(&mut self, id: ArtistId) -> &mut PathStyle {
    //    self.artists[id.index()].style_mut()
    //}

    pub(crate) fn _artist<A>(&self, id: ArtistId) -> &A
    where
        A: Artist<Data> + 'static
    {
        unsafe { self.ptrs[id.index()]._deref() }
    }

    pub(crate) fn artist_mut<A>(&mut self, id: ArtistId) -> &mut A
    where
        A: Artist<Data> + 'static
    {
        unsafe { self.ptrs[id.index()].deref_mut() }
    }

    pub(crate) fn get_handlers(&self) -> Vec<LegendHandler> {
        let mut vec = Vec::<LegendHandler>::new();

        for artist in &self.artists {
            match artist.get_legend(self) {
                Some(handler) => vec.push(handler),
                None => {},
            };
        }

        vec
    }
}

impl Artist<Data> for PlotContainer {
    fn update(&mut self, pos: &Bounds<Canvas>, canvas: &Canvas) {
        for artist in &self.artists {
            artist.update(self, pos, canvas);
        }
    }
    
    fn get_extent(&mut self) -> Bounds<Data> {
        let mut bounds = Bounds::none();

        for artist in &self.artists {
            bounds = if bounds.is_none() {
                artist.get_extent(self)
            } else {
                let extent = artist.get_extent(self);
                if extent.is_none() { bounds } else { bounds.union(&extent) }
            }
        }

        if bounds.is_none() {
            Bounds::new(Point(0., 0.), Point(1., 1.))
        } else {
            bounds
        }
    }

    fn draw(
        &mut self, 
        renderer: &mut dyn Renderer,
        to_canvas: &ToCanvas,
        clip: &Clip,
        style: &dyn PathOpt,
    ) {
        for (i, artist) in self.artists.iter().enumerate() {
            let style = self.cycle.push(style, i);

            artist.draw(self, renderer, to_canvas, clip, &style);
        }
    }
}

trait ArtistHandleTrait<M: Coord> : Send {
    // fn id(&self) -> ArtistId;

    //fn style_mut(&mut self) -> &mut PathStyle;

    fn update(&self, container: &PlotContainer, pos: &Bounds<Canvas>, canvas: &Canvas);
    fn get_extent(&self, container: &PlotContainer) -> Bounds<M>;
    fn get_legend(&self, container: &PlotContainer) -> Option<LegendHandler>;

    fn draw(
        &self, 
        container: &PlotContainer,
        renderer: &mut dyn Renderer,
        to_canvas: &ToCanvas,
        clip: &Clip,
        style: &dyn PathOpt,
    );
}

struct ArtistHandle<M: Coord, A: Artist<M>> {
    id: ArtistId,
    marker: PhantomData<(M, A)>,
}

impl<M: Coord, A: Artist<M>> ArtistHandle<M, A> {
    fn new(id: ArtistId) -> Self {
        Self {
            id,
            marker: PhantomData,
        }
    }
}

impl<A: Artist<Data>> ArtistHandleTrait<Data> for ArtistHandle<Data, A>
where
    A: PlotArtist + 'static,
{
    //fn id(&self) -> ArtistId {
    //    self.id
    //}

    //fn style_mut(&mut self) -> &mut PathStyle {
    //    &mut self.style
    //}

    fn update(&self, container: &PlotContainer, pos: &Bounds<Canvas>, canvas: &Canvas) {
        container.deref_mut::<A>(self.id).update(pos, canvas);
    }

    fn get_extent(&self, container: &PlotContainer) -> Bounds<Data> {
        container.deref_mut::<A>(self.id).get_extent()
    }

    fn draw(
        &self, 
        container: &PlotContainer,
        renderer: &mut dyn Renderer,
        to_canvas: &ToCanvas,
        clip: &Clip,
        style: &dyn PathOpt,
    ) {
        container.deref_mut::<A>(self.id).draw(renderer, to_canvas, clip, style)
    }

    fn get_legend(&self, container: &PlotContainer) -> Option<LegendHandler> {
        container.deref_mut::<A>(self.id).get_legend()
    }
}

// TODO: replace with downcast crate

pub(crate) struct PlotPtr<M: Coord> {
    type_id: TypeId, 
    marker: PhantomData<M>,
    data: NonNull<u8>,
}

impl<M: Coord> PlotPtr<M> {
    pub(crate) fn new<A>(_id: ArtistId, artist: A) -> Self
    where
        A: Artist<M> + 'static
    {
        let layout = alloc::Layout::new::<A>();
        let data = unsafe { alloc::alloc(layout) };
        let mut value = ManuallyDrop::new(artist);
        let source: NonNull<u8> = NonNull::from(&mut *value).cast();

        let src = source.as_ptr();
        let count = mem::size_of::<A>();

        // TODO: drop

        unsafe {
            ptr::copy_nonoverlapping::<u8>(src, data, count);
        }

        Self {
            type_id: TypeId::of::<A>(),
            data: NonNull::new(data).unwrap(),
            marker: PhantomData,
        }
    }

    pub unsafe fn _deref<A>(&self) -> &A
    where
        A: Artist<M> + 'static
    {
        assert_eq!(self.type_id, TypeId::of::<A>());

        &*self.data.as_ptr().cast::<A>()
    }

    pub unsafe fn deref_mut<A>(&self) -> &mut A 
    where
        A: Artist<M> + 'static
    {
        assert_eq!(self.type_id, TypeId::of::<A>());

        &mut *self.data.as_ptr().cast::<A>()
    }
}

// TODO: replace with downcast
unsafe impl<M: Coord> Send for PlotPtr<M> {}
