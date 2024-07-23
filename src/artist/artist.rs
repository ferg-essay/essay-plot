use std::{marker::PhantomData, ops::Deref};

use essay_graphics::{api::{
    renderer::Renderer, Affine2d, Bounds, Canvas, Event, Clip, Coord, PathOpt
}, layout::View};

use crate::chart::{ArtistId, ArtistView, ConfigArc, Data, Frame, LegendHandler};

pub trait Artist<M: Coord> : Send {
    fn resize(&mut self, renderer: &mut dyn Renderer, pos: &Bounds<Canvas>);

    fn bounds(&mut self) -> Bounds<M>;
    
    fn draw(
        &mut self, 
        renderer: &mut dyn Renderer,
        to_canvas: &ToCanvas,
        clip: &Clip,
        style: &dyn PathOpt,
    );

    #[allow(unused_variables)]
    fn event(&mut self, renderer: &mut dyn Renderer, event: &Event) -> bool {
        false
    }
}

pub trait PlotArtist : Artist<Data> + Sized {
    type Opt;
    
    fn config(
        &mut self, 
        cfg: &ConfigArc, 
        view: ArtistView<Self>,
    ) -> Self::Opt;

    fn get_legend(&self) -> Option<LegendHandler>;
}

pub trait IntoArtist {
    type Artist : PlotArtist;

    fn into_artist(self) -> Self::Artist;
}

impl<A: PlotArtist> IntoArtist for A {
    type Artist = Self;

    fn into_artist(self) -> Self::Artist {
        self
    }
}

pub struct ToCanvas {
    pos_frame: Bounds<Canvas>,
    to_canvas: Affine2d,
}

impl ToCanvas {
    pub fn new(pos_frame: Bounds<Canvas>, to_canvas: Affine2d) -> Self {
        Self {
            pos_frame,
            to_canvas
        }
    }

    pub fn pos(&self) -> &Bounds<Canvas> {
        &self.pos_frame
    }

    pub fn to_canvas(&self) -> &Affine2d {
        &self.to_canvas
    }
}

impl Deref for ToCanvas {
    type Target = Affine2d;

    fn deref(&self) -> &Self::Target {
        self.to_canvas()
    }
}
