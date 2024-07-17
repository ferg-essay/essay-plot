use essay_graphics::api::{Coord, Canvas, Bounds, driver::Renderer, Clip, PathOpt};

use crate::{
    artist::{Artist, ArtistHandle, PathStyle, PlotArtist, ToCanvas}, data_artist_option_struct, frame::{Data, LegendHandler}, path_style_options
};

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
    fn update(&mut self, pos: &Bounds<Canvas>, canvas: &Canvas) {
        self.artist.update(pos, canvas);
    }

    fn get_extent(&mut self) -> Bounds<Data> {
        self.artist.get_extent()
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
        artist: ArtistHandle<PlotOptHandle<Data>>,
    ) -> Self::Opt {
        self.style = PathStyle::from_config(cfg, "artist");

        unsafe { PlotOpt::new(artist) }
    }

    fn get_legend(&self) -> Option<LegendHandler> {
        None
    }
}
