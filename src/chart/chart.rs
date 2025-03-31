use essay_graphics::{
    api::renderer::{self, Drawable, Renderer},
    layout::View, 
};

use crate::{
    artist::{Artist, ArtistDraw},
    chart::{AspectMode, AxisOpt, ChartFrame, Data, FrameArtist, FrameTextOpt}
};

use super::{style::PlotOptHandle, ConfigArc, PlotOpt, Scaling};

#[derive(Clone)]
pub struct Chart {
    view: View<ChartFrame>,
}

impl Chart {
    pub fn new(config: &ConfigArc) -> Self {
        let view = View::from(ChartFrame::new(config));
        
        // chart.default_properties();

        Self {
            view
        }
    }

    pub fn view(&self) -> &View<ChartFrame> {
        &self.view
    }

    fn text_opt(&self, artist: FrameArtist) -> FrameTextOpt {
        FrameTextOpt::new(self.view.clone(), artist)
    }

    pub fn title(&mut self, label: &str) -> FrameTextOpt {
        let mut opt = self.text_opt(FrameArtist::Title);
        opt.label(label);
        opt
    }

    pub fn x(&mut self) -> AxisOpt {
        AxisOpt::new(&self.view, FrameArtist::X)
    }

    pub fn y(&mut self) -> AxisOpt {
        AxisOpt::new(&self.view, FrameArtist::Y)
    }

    pub fn x_label(&mut self, label: &str) -> FrameTextOpt {
        let mut opt = self.text_opt(FrameArtist::XLabel);
        opt.label(label);
        opt
    }

    pub fn y_label(&mut self, label: &str) -> FrameTextOpt {
        let mut opt = self.text_opt(FrameArtist::YLabel);
        opt.label(label);
        opt
    }

    pub fn scaling(&mut self, scaling: Scaling) -> &mut Self {
        self.view.write(|f| { 
            f.data_mut().scaling(scaling); 
        });

        self
    }

    pub fn aspect(&mut self, aspect: f32) -> &mut Self {
        self.view.write(|f| { 
            f.data_mut().aspect(aspect); 
        });

        self
    }

    pub fn aspect_mode(&mut self, mode: AspectMode) -> &mut Self {
        self.view.write(|f| { 
            f.data_mut().aspect_mode(mode); 
        });

        self
    }

    pub fn flip_y(&mut self, is_flip_y: bool) -> &mut Self {
        self.view.write(|f| { 
            f.data_mut().flip_y(is_flip_y); 
        });

        self
    }

    pub fn xlim(&mut self, x_min: f32, x_max: f32) -> &mut Self {
        self.view.write(|f| { 
            f.data_mut().xlim(x_min, x_max); 
        });

        self
    }

    pub fn ylim(&mut self, y_min: f32, y_max: f32) -> &mut Self {
        self.view.write(|f| { 
            f.data_mut().ylim(y_min, y_max); 
        });

        self
    }

    pub fn colorbar(&mut self) -> &mut Self {
        self.view.write(|f| { 
            f.colorbar();
        });

        self
    }

    fn _default_properties(&mut self) {
        // self.title.font().size(12.);
    }

    // TODO: should there be a plain add_artist that doesn't wrap PlotStyle?

    pub fn add_simple_artist<'a, A>(
        &mut self, 
        artist: A,
    ) -> PlotOpt
    where
        A: ArtistDraw<Data> + 'static
    {
        self.artist(PlotOptHandle::new(artist))
    }

    pub fn artist<'a, A>(
        &mut self, 
        artist: A,
    ) -> <A::Artist as Artist<Data>>::Opt 
    where
        A: IntoArtist + 'static
    {
        let artist = artist.into_artist();

        let view_clone = self.view.clone();

        self.view.write(|f| {
            let config = f.config().clone();

            f.data_mut().add_artist(artist, &config, view_clone)
        })
    }
}

impl From<Chart> for View<ChartFrame> {
    fn from(chart: Chart) -> Self {
        chart.view.clone()
    }
}

impl From<&Chart> for View<ChartFrame> {
    fn from(chart: &Chart) -> Self {
        chart.view.clone()
    }
}

impl Default for Chart {
    fn default() -> Self {
        Chart::new(&ConfigArc::default())
    }
}

impl Drawable for Chart {
    fn draw(&mut self, renderer: &mut dyn Renderer) -> renderer::Result<()> {
        self.view.drawable().draw(renderer)
    }
}

pub trait IntoArtist {
    type Artist : Artist<Data>;

    fn into_artist(self) -> Self::Artist;
}

impl<A: Artist<Data>> IntoArtist for A {
    type Artist = Self;

    fn into_artist(self) -> Self::Artist {
        self
    }
}
