use essay_graphics::api::{renderer::{Canvas, Renderer}, Bounds, Clip, Coord, PathOpt};

use crate::{
    artist::{Artist, PathStyle, ToCanvas}, 
    chart::{Data, LegendHandler, PlotArtist}, 
    data_artist_option_struct, 
    path_style_options
};

use super::ArtistView;

data_artist_option_struct!(PlotOpt, PlotOptHandle<Data>);

impl PlotOpt {
    path_style_options!(style);
}

pub struct PlotOptHandle<M: Coord> {
    artist: Box<dyn Artist<M>>,
    style: PathStyle,
}

impl<M: Coord> PlotOptHandle<M> {
    pub fn new<A>(artist: A) -> Self
    where
        A: Artist<M> + 'static
    {
        Self {
            artist: Box::new(artist),
            style: PathStyle::new(),
        }
    }
}

impl Artist<Data> for PlotOptHandle<Data> {
    fn resize(&mut self, renderer: &mut dyn Renderer, pos: &Bounds<Canvas>) {
        self.artist.resize(renderer, pos);
    }

    fn bounds(&mut self) -> Bounds<Data> {
        self.artist.bounds()
    }

    fn draw(
        &mut self, 
        renderer: &mut dyn Renderer,
        to_canvas: &ToCanvas,
        clip: &Clip,
        style: &dyn PathOpt,
    ) {
        let style = self.style.push(style);

        self.artist.draw(renderer, to_canvas, clip, &style);
    }
}

impl PlotArtist for PlotOptHandle<Data> {
    type Opt = PlotOpt;

    fn config(
        &mut self, 
        cfg: &super::ConfigArc, 
        artist: ArtistView<PlotOptHandle<Data>>,
    ) -> Self::Opt {
        self.style = PathStyle::from_config(cfg, "artist");

        PlotOpt::new(artist)
    }

    fn get_legend(&self) -> Option<LegendHandler> {
        None
    }
}
