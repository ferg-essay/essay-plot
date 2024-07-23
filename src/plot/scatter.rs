use essay_graphics::api::{affine2d, renderer::Renderer, Bounds, Canvas, Clip, JoinStyle, Path, PathOpt};
use essay_tensor::Tensor;

use crate::{
    artist::{
        paths::{self}, 
        Artist, ArtistHandle, Markers, PathCollection, PathStyle, PlotArtist, ToCanvas
    }, 
    data_artist_option_struct, 
    path_style_options,
    chart::{Data, LegendHandler, ConfigArc, Chart}, 
};

pub fn scatter(
    graph: &mut Chart, 
    x: impl Into<Tensor>, 
    y: impl Into<Tensor>, 
) -> ScatterOpt {
    let x : Tensor = x.into();
    let y : Tensor = y.into();

    let plot = ScatterPlot::new(x.stack([y], -1));

    graph.artist(plot)
}

pub struct ScatterPlot {
    xy: Tensor,
    collection: PathCollection,
    is_stale: bool,

    style: PathStyle,

    size: f32,

    marker: Markers,
}

impl ScatterPlot {
    fn new(xy: Tensor) -> Self {
        let scale = 10.;
        let size = scale * scale;
        let path = paths::unit_pos().transform(
            &affine2d::scale(scale, scale)
        );

        let collection = PathCollection::new(path, xy.clone());
        let mut style = PathStyle::new();

        //style.linewidth(1.5);
        style.join_style(JoinStyle::Round);

        Self {
            xy,
            style,
            size,
            marker: Markers::Circle,
            collection,
            is_stale: true,
        }
    }

    /*
    fn size(&mut self, size: f32) -> &mut Self {
        assert!(size >= 0.);

        self.size = size;

        self
    }

    fn marker(&mut self, marker: impl Into<Markers>) -> &mut Self {
        self.marker = marker.into();

        self
    }
    */
}

impl Artist<Data> for ScatterPlot {
    fn resize(&mut self, renderer: &mut dyn Renderer, pos: &Bounds<Canvas>) {
        if self.is_stale {
            self.is_stale = false;

            // 0.5 because source is [-1, 1]
            let scale = 0.5 * self.size.sqrt() * renderer.scale_factor();

            let path: Path<Canvas> = self.marker.get_scaled_path(scale);

            self.collection = PathCollection::new(path, &self.xy);
        }

        self.collection.resize(renderer, pos);
    }

    fn bounds(&mut self) -> Bounds<Data> {
        self.collection.bounds()
    }

    fn draw(
        &mut self, 
        renderer: &mut dyn Renderer,
        to_canvas: &ToCanvas,
        clip: &Clip,
        style: &dyn PathOpt,
    ) {
        let style = self.style.push(style);

        self.collection.draw(renderer, to_canvas, clip, &style)
    }
}

impl PlotArtist for ScatterPlot {
    type Opt = ScatterOpt;

    fn config(
        &mut self, 
        cfg: &ConfigArc, 
        artist: ArtistHandle<ScatterPlot>,
    ) -> Self::Opt {
        self.style = PathStyle::from_config(cfg, "scatter");

        ScatterOpt::new(artist)
    }

    fn get_legend(&self) -> Option<LegendHandler> {
        None
    }
}

data_artist_option_struct!(ScatterOpt, ScatterPlot);

impl ScatterOpt {
    path_style_options!(style);

    pub fn marker(&mut self, marker: impl Into<Markers>) -> &mut Self {
        self.write(|plot| plot.marker = marker.into());

        self
    }
}
