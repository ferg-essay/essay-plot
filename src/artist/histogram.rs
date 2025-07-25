use essay_graphics::api::{renderer::{Canvas, Renderer, Result}, Bounds, Path, PathOpt};
use essay_tensor::{stats::HistArgs, tensor::Tensor};

use crate::{
    artist::Stale, chart::{Data, LegendHandler}, config::{ConfigArc, PathStyle}, data_artist_option_struct, path_style_options, transform::ToCanvas
};

use super::{paths, Artist, ArtistDraw, ArtistView};

pub struct Histogram {
    data: Tensor,
    style: PathStyle,

    n_bins: Option<usize>,
    bins: Tensor,
    count: Tensor,
    extent: Bounds<Data>,
    paths: Vec<Path<Data>>,

    stale: Stale,
}

impl Histogram {
    pub fn new(data: impl Into<Tensor>) -> Self {
        let data : Tensor = data.into();

        assert!(data.rank() == 1, "histogram requires 1D value {:?}", data.shape());

        let mut histogram = Self {
            data,
            style: PathStyle::new(),
            n_bins: None,
            bins: Tensor::zeros([1]),
            count: Tensor::zeros([1]),
            extent: Bounds::<Data>::none(),
            paths: Vec::new(),
            stale: Stale::stale(),
        };

        histogram.update_bounds();

        histogram
    }

    pub(crate) fn set_data(&mut self, data: Tensor) {
        assert!(data.rank() == 1, "histogram requires 1D value {:?}", data.shape());

        self.data = data;
        self.stale = Stale::stale();
    }

    fn update_bounds(&mut self) {
        if self.stale.is_stale() {
            self.stale = Stale::new_for_update();

            let hist_args: HistArgs = if let Some(n_bins) = self.n_bins {
                n_bins.into()
            } else {
                ().into()
            };

            let (count, bins) = essay_tensor::stats::histogram(&self.data, hist_args);

            self.count = count;
            self.bins = bins;

            let (min, max) = (self.bins[0], self.bins[self.bins.len() - 1]);
            let c_max = self.count.reduce_max()[0];

            self.extent = Bounds::new([min, 0.], [max, c_max]);

            let mut paths = Vec::<Path<Data>>::new();

            for i in 0..self.count.len() {
                paths.push(paths::rect(
                    [self.bins[i], 0.], 
                    [self.bins[i + 1], self.count[i]]
                ));
            }

            self.paths = paths;
        }
    }
    
}

impl ArtistDraw<Data> for Histogram {
    fn bounds(&mut self) -> Bounds<Data> {
        self.update_bounds();

        self.extent.clone()
    }

    fn draw(
        &mut self, 
        renderer: &mut dyn Renderer,
        to_canvas: &ToCanvas<Data>,
        style: &dyn PathOpt,
    ) -> Result<()> {
        self.update_bounds();

        let style = self.style.push(style);

        for path in &self.paths {
            let c_path: Path<Canvas> = to_canvas.transform_path(path);
            renderer.draw_path(&c_path, &style)?;
        }

        Ok(())
    }
}

impl Artist<Data> for Histogram {
    type Opt = HistogramOpt;

    fn config(&mut self, cfg: &ConfigArc) {
        self.style = PathStyle::from_config(cfg, "histogram");
    }

    fn opt(&mut self, view: ArtistView<Data, Histogram>) -> Self::Opt {
        HistogramOpt::new(view)
    }

    fn get_legend(&self) -> Option<LegendHandler> {
        None
    }
}

data_artist_option_struct!(HistogramOpt, Histogram);

impl HistogramOpt {
    path_style_options!(style);

    pub fn n_bins(&mut self, n_bins: usize) -> &mut Self {
        assert!(n_bins > 0);

        self.write(|artist| {
            artist.n_bins = Some(n_bins);
            artist.stale = Stale::stale();
        });

        self
    }

    pub fn data(&mut self, data: impl Into<Tensor>) -> &mut Self {
        let data = data.into();
        assert!(data.rank() == 2, "Histogram data must be 1D. Shape={:?}", data.shape());

        self.write(|artist| {
            artist.set_data(data);
        });

        self
    }
}
