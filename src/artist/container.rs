use essay_graphics::api::{
    Bounds, Coord, Canvas, PathOpt,
    renderer::Renderer, Clip
};

use crate::{chart::{ArtistView, ConfigArc, Data, LegendHandler}, data_artist_option_struct};

use super::{Artist, PathStyle, PlotArtist, StyleCycle, ToCanvas};

pub struct Container<M: Coord> {
    artists: Vec<Box<dyn Artist<M>>>,
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

    pub fn push(&mut self, artist: impl Artist<M> + 'static) {
        self.artists.push(Box::new(artist));
    }
}

impl<M: Coord> Artist<M> for Container<M> {
    fn resize(&mut self, renderer: &mut dyn Renderer, pos: &Bounds<Canvas>) {
        for artist in &mut self.artists {
            artist.resize(renderer, pos);
        }
    }

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
        clip: &Clip,
        style: &dyn PathOpt,
    ) {
        let style = self.style.push(style);

        for (i, artist) in self.artists.iter_mut().enumerate() {
            let style = self.cycle.push(&style, i);

            artist.draw(renderer, to_canvas, clip, &style);
        }
    }
}

impl PlotArtist for Container<Data> {
    type Opt = ContainerOpt;

    fn config(&mut self, cfg: &ConfigArc, artist: ArtistView<Container<Data>>) -> Self::Opt {
        self.cycle = StyleCycle::from_config(cfg, "container.cycle");

        ContainerOpt::new(artist)
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
