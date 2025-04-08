use essay_graphics::api::{renderer::{Renderer, Result}, Bounds, Coord, PathOpt};

use crate::{
    artist::{Artist, ArtistDraw, ArtistView}, 
    chart::{Data, LegendHandler}, 
    config::{ConfigArc, PathStyle}, 
    data_artist_option_struct, path_style_options, transform::ToCanvas
};

data_artist_option_struct!(PlotOpt, PlotOptHandle<Data>);

impl PlotOpt {
    path_style_options!(style);
}

pub struct PlotOptHandle<M: Coord> {
    artist: Box<dyn ArtistDraw<M>>,
    style: PathStyle,
}

impl<M: Coord> PlotOptHandle<M> {
    pub fn new<A>(artist: A) -> Self
    where
        A: ArtistDraw<M> + 'static
    {
        Self {
            artist: Box::new(artist),
            style: PathStyle::new(),
        }
    }
}

impl ArtistDraw<Data> for PlotOptHandle<Data> {
    fn bounds(&mut self) -> Bounds<Data> {
        self.artist.bounds()
    }

    fn draw(
        &mut self, 
        renderer: &mut dyn Renderer,
        to_canvas: &ToCanvas<Data>,
        style: &dyn PathOpt,
    ) -> Result<()> {
        let style = self.style.push(style);

        self.artist.draw(renderer, to_canvas, &style)
    }
}

impl Artist<Data> for PlotOptHandle<Data> {
    type Opt = PlotOpt;

    fn config(&mut self, cfg: &ConfigArc) {
        self.style = PathStyle::from_config(cfg, "artist");
    }

    fn opt(&mut self, view: ArtistView<Data, PlotOptHandle<Data>>) -> Self::Opt {
        PlotOpt::new(view)
    }

    fn get_legend(&self) -> Option<LegendHandler> {
        None
    }
}
