use essay_graphics::api::{renderer::{Renderer, Result}, Bounds, PathOpt};
use essay_tensor::{array::{stack, stack_axis}, signal::rfft_norm, tensor::Tensor};

use crate::{
    artist::{Artist, ArtistDraw, ArtistView, GridColor, Norm, Norms, Shading, ToCanvas}, 
    chart::{Chart, Data, LegendHandler}, 
    palette::{ColorMap, EssayColors},
    config::ConfigArc,
    data_artist_option_struct
};

pub fn specgram(
    graph: &mut Chart, 
    y: impl Into<Tensor>, 
) -> SpecGramOpt {
    let y = y.into();

    let specgram = SpecGram::new(y);
    
    graph.artist(specgram)
}

pub struct SpecGram {
    grid_color: GridColor,
    data: Tensor,
    nfft: usize,
    overlap: usize,
    norm: Norm,
}

impl SpecGram {
    pub fn new(data: impl Into<Tensor>) -> Self {
        let data : Tensor = data.into();
        assert!(data.rank() == 1, "specgram requires 1d value {:?}", data.shape());

        let nfft = 256;
        let overlap = 128;

        let spectrum = calculate_spectrum(&data, nfft, overlap);

        let norms = Norms::Ln;

        let mut grid_color = GridColor::new(spectrum);
        //grid_color.color_map(ColorMaps::BlueWhite2);
        grid_color.color_map(EssayColors::BlueOrange);
        grid_color.norm(Norm::from(norms.clone()));

        let mut spec_gram = Self {
            data: data,
            nfft,
            overlap,
            grid_color,
            norm: Norm::from(norms),
        };

        spec_gram.update_bounds();

        spec_gram
    }

    fn update_bounds(&mut self) {
        let spectrum = calculate_spectrum(
            &self.data, 
            self.nfft, 
            self.overlap
        );

        self.norm.set_bounds(&spectrum);

        let max = self.norm.max();
        let min = self.norm.min().max(max - 4.);
        self.grid_color.set_data(spectrum);
        self.grid_color.set_norm(min, max);
    }
}

fn calculate_spectrum(data: &Tensor, nfft: usize, overlap: usize) -> Tensor {
    let delta = nfft - overlap;

    let mut i : usize = 0;
    let mut values : Vec<Tensor> = Vec::new();
    while i < data.len() {
        let sublen = nfft.min(data.len() - i);
        let mut slice = data.subslice(i, sublen);

        if slice.len() < nfft {
            let mut vec = Vec::from(slice.as_slice());
            vec.resize(nfft, 0.);
            slice = Tensor::from(vec);
        }

        let value = rfft_norm(slice, ());
        let value = value.subslice(1, value.len() - 1);

        values.push(value);

        i += delta;
    }

    stack_axis(1, values)
}

impl ArtistDraw<Data> for SpecGram {
    fn bounds(&mut self) -> Bounds<Data> {
        self.grid_color.bounds()
    }

    fn draw(
        &mut self, 
        renderer: &mut dyn Renderer,
        to_canvas: &ToCanvas,
        style: &dyn PathOpt,
    ) -> Result<()> {
        self.grid_color.draw(renderer, to_canvas, style)
    }
}

impl Artist<Data> for SpecGram {
    type Opt = SpecGramOpt;

    fn config(&mut self, _cfg: &ConfigArc) {
    }

    fn opt(&mut self, view: ArtistView<Data, SpecGram>) -> Self::Opt {
        SpecGramOpt::new(view)
    }

    fn get_legend(&self) -> Option<LegendHandler> {
        None
    }
}

data_artist_option_struct!(SpecGramOpt, SpecGram);

impl SpecGramOpt {
    pub fn data(&mut self, data: impl Into<Tensor>) -> &mut Self {
        let data = data.into();
        assert!(data.rank() == 1, "SpecGram data must be rank 1. Shape={:?}", data.shape());

        self.write(|artist| {
            artist.data = data;
            artist.update_bounds();
        });

        self
    }

    pub fn nfft(&mut self, nfft: usize) -> &mut Self {
        assert!(nfft > 0);

        self.write(|artist| {
            assert!(artist.overlap < artist.nfft);

            artist.nfft = nfft;
            artist.update_bounds();
        });

        self
    }

    pub fn overlap(&mut self, overlap: usize) -> &mut Self {
        self.write(|artist| {
            assert!(overlap < artist.nfft);

            artist.overlap = overlap;
            artist.update_bounds();
        });

        self
    }

    pub fn norm(&mut self, norm: impl Into<Norm>) -> &mut Self {
            self.write(|artist| {
                artist.grid_color.norm(norm);
                artist.update_bounds();
            });

        self
    }

    pub fn color_map(&mut self, cmap: impl Into<ColorMap>) -> &mut Self {
        self.write(|artist| {
            artist.grid_color.color_map(cmap);
        });

        self
    }

    pub fn shading(&mut self, shading: Shading) -> &mut Self {
        self.write(|artist| {
        artist.grid_color.shading(shading);
    });

    self
}
}
