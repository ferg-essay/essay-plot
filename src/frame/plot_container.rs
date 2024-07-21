use std::{any::Any, marker::PhantomData};

use essay_graphics::api::{Coord, Bounds, driver::Renderer, Canvas, PathOpt, Clip, Point};

use crate::{artist::{Artist, StyleCycle, PlotArtist, ToCanvas}, graph::Config};

use super::{legend::LegendHandler, ArtistId, Data};

pub(crate) struct PlotContainer {
    artist_any: Vec<Box<dyn Any + Send>>,
    artist_handles: Vec<Box<dyn ArtistHandleTrait<Data>>>,
    cycle: StyleCycle,
}

impl PlotContainer {
    pub(crate) fn new(cfg: &Config) -> Self {
        let container = Self {
            artist_any: Vec::new(),
            artist_handles: Vec::new(),
            cycle: StyleCycle::from_config(cfg, "frame.cycle"),
        };

        container
    }

    pub(crate) fn add_artist<A>(&mut self, artist: A) -> ArtistId
    where
        A: PlotArtist + 'static
    {
        let id = ArtistId::new_data(self.artist_any.len());
        //let id = ArtistId::new_data(self.ptrs.len());

        //let plot = PlotPtr::new(id, artist);
        // self.ptrs.push(plot);
        self.artist_any.push(Box::new(artist));

        self.artist_handles.push(Box::new(ArtistHandle::<Data, A>::new()));

        id
    }

    pub(crate) fn _cycle(&self) -> &StyleCycle {
        &self.cycle
    }

    pub(crate) fn _cycle_mut(&mut self) -> &mut StyleCycle {
        &mut self.cycle
    }

    fn _deref<A: Artist<Data> + 'static>(&self, id: ArtistId) -> &A {
        // unsafe { self.ptrs[id.index()]._deref() }
        self.artist_any[id.index()].downcast_ref().unwrap()
    }

    fn deref_mut<A: Artist<Data> + 'static>(&mut self, id: ArtistId) -> &mut A {
        self.artist_any[id.index()].downcast_mut().unwrap()
    }

    pub(crate) fn artist_mut<A>(&mut self, id: ArtistId) -> &mut A
    where
        A: Artist<Data> + 'static
    {
        self.deref_mut(id)
    }

    pub(crate) fn get_handlers(&mut self) -> Vec<LegendHandler> {
        let mut vec = Vec::<LegendHandler>::new();

        for (i, handle) in self.artist_handles.iter().enumerate() {
            let artist_any = &mut self.artist_any[i];

            match handle.get_legend(artist_any) {
                Some(handler) => vec.push(handler),
                None => {},
            };
        }

        vec
    }
}

impl Artist<Data> for PlotContainer {
    fn resize(&mut self, renderer: &mut dyn Renderer, pos: &Bounds<Canvas>) {
        for (i, handle) in self.artist_handles.iter().enumerate() {
            let artist_any = &mut self.artist_any[i];

            handle.resize(artist_any, renderer, pos);
        }
    }
    
    fn bounds(&mut self) -> Bounds<Data> {
        let mut bounds = Bounds::none();

        for (i, handle) in self.artist_handles.iter().enumerate() {
            let artist_any = &mut self.artist_any[i];

            bounds = if bounds.is_none() {
                handle.get_extent(artist_any)
            } else {
                let extent = handle.get_extent(artist_any);
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
        for (i, handle) in self.artist_handles.iter().enumerate() {
            let style = self.cycle.push(style, i);

            handle.draw(&mut self.artist_any[i], renderer, to_canvas, clip, &style);
        }
    }
}

trait ArtistHandleTrait<M: Coord> : Send {
    fn resize(&self, any: &mut Box<dyn Any + Send>, renderer: &mut dyn Renderer, pos: &Bounds<Canvas>);
    fn get_extent(&self, any: &mut Box<dyn Any + Send>) -> Bounds<M>;
    fn get_legend(&self, any: &mut Box<dyn Any + Send>) -> Option<LegendHandler>;

    fn draw(
        &self, 
        artist_any: &mut Box<dyn Any + Send>,
        renderer: &mut dyn Renderer,
        to_canvas: &ToCanvas,
        clip: &Clip,
        style: &dyn PathOpt,
    );
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
    fn resize(&self, any: &mut Box<dyn Any + Send>, renderer: &mut dyn Renderer, pos: &Bounds<Canvas>) {
        any.downcast_mut::<A>().unwrap().resize(renderer, pos);
    }

    fn get_extent(&self, any: &mut Box<dyn Any + Send>) -> Bounds<Data> {
        any.downcast_mut::<A>().unwrap().bounds()
    }

    fn draw(
        &self, 
        artist_any: &mut Box<dyn Any + Send + 'static>,
        renderer: &mut dyn Renderer,
        to_canvas: &ToCanvas,
        clip: &Clip,
        style: &dyn PathOpt,
    ) {
        let artist = artist_any.downcast_mut::<A>().unwrap();
        artist.draw(renderer, to_canvas, clip, style)
    }

    fn get_legend(&self, any: &mut Box<dyn Any + Send>) -> Option<LegendHandler> {
        let artist = any.downcast_mut::<A>().unwrap();
        artist.get_legend()
    }
}
