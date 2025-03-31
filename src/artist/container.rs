use essay_graphics::api::{
    renderer::{Renderer, Result}, Bounds, Coord, PathOpt
};

use crate::{
    chart::{Data, LegendHandler}, 
    config::{ConfigArc, PathStyle, StyleCycle},
    data_artist_option_struct
};

use super::{Artist, ArtistDraw, ArtistView, ToCanvas};

pub struct Container<M: Coord> {
    artists: Vec<Box<dyn ArtistDraw<M>>>,
    style: PathStyle,
    cycle: StyleCycle,
}

impl<M: Coord> Container<M> {
    pub fn new() -> Self {
        Self {
            artists: Vec::new(),
            style: PathStyle::new(),
            cycle: StyleCycle::new(),
        }
    }

    pub fn push(&mut self, artist: impl ArtistDraw<M> + 'static) {
        self.artists.push(Box::new(artist));
    }
}

impl<M: Coord> ArtistDraw<M> for Container<M> {
    /*
    fn resize(&mut self, renderer: &mut dyn Renderer, pos: &Bounds<Canvas>) {
        for artist in &mut self.artists {
            artist.resize(renderer, pos);
        }
    }
    */

    fn bounds(&mut self) -> Bounds<M> {
        let mut bounds = Bounds::<M>::none();

        for artist in &mut self.artists {
            bounds = if bounds.is_none() {
                artist.bounds().clone()
            } else {
                bounds.union(&artist.bounds())
            };
        }

        bounds
    }

    fn draw(
        &mut self, 
        renderer: &mut dyn Renderer,
        to_canvas: &ToCanvas,
        style: &dyn PathOpt,
    ) -> Result<()> {
        let style = self.style.push(style);

        for (i, artist) in self.artists.iter_mut().enumerate() {
            let style = self.cycle.push(&style, i);

            artist.draw(renderer, to_canvas, &style)?;
        }

        Ok(())
    }
}

impl Artist<Data> for Container<Data> {
    type Opt = ContainerOpt;

    fn config(&mut self, cfg: &ConfigArc) {
        self.cycle = StyleCycle::from_config(cfg, "container.cycle");
    }

    fn opt(&mut self, view: ArtistView<Data, Container<Data>>) -> Self::Opt {
        ContainerOpt::new(view)
    }

    fn get_legend(&self) -> Option<LegendHandler> {
        None
    }
}

data_artist_option_struct!(ContainerOpt, Container<Data>);

//impl PathStyleArtist for Container<Data> {
//    fn style_mut(&mut self) -> &mut PathStyle {
//        &mut self.style
//    }
//}
