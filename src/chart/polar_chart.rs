use essay_graphics::{
    api::renderer::{self, Drawable, Renderer},
    layout::View, 
};

use crate::{
    artist::{Artist, ArtistDraw, IntoArtist}, 
    chart::{AspectMode, Data, FrameArtist, FrameTextOpt}, 
    config::ConfigArc, 
    palette::Palette, transform::AngleCoord
};

use super::{polar_frame::PolarFrame, style::PlotOptHandle, PlotOpt, PolarAxisOpt, Scaling};

#[derive(Clone)]
pub struct PolarChart {
    view: View<PolarFrame>,
}

impl PolarChart {
    pub fn new(config: &ConfigArc) -> Self {
        let view = View::from(PolarFrame::new(config));
        
        // chart.default_properties();

        Self {
            view
        }
    }

    pub fn view(&self) -> &View<PolarFrame> {
        &self.view
    }

    fn text_opt(&self, artist: FrameArtist) -> FrameTextOpt<PolarFrame> {
        FrameTextOpt::new(self.view.clone(), artist)
    }

    pub fn title(&mut self, label: &str) -> FrameTextOpt<PolarFrame> {
        let mut opt = self.text_opt(FrameArtist::Title);
        opt.label(label);
        opt
    }

    pub fn x(&mut self) -> PolarAxisOpt {
        PolarAxisOpt::new(&self.view, FrameArtist::X)
    }

    pub fn y(&mut self) -> PolarAxisOpt {
        PolarAxisOpt::new(&self.view, FrameArtist::Y)
    }

    pub fn x_label(&mut self, label: &str) -> FrameTextOpt<PolarFrame> {
        let mut opt = self.text_opt(FrameArtist::XLabel);
        opt.label(label);
        opt
    }

    pub fn y_label(&mut self, label: &str) -> FrameTextOpt<PolarFrame> {
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

    pub fn angle_coord(&mut self, angle_coord: AngleCoord) -> &mut Self {
        self.view.write(|f| { 
            f.angle_coord(angle_coord);
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

    pub fn xlim(&mut self, x_min: Option<f32>, x_max: Option<f32>) -> &mut Self {
        self.view.write(|f| { 
            f.data_mut().xlim(x_min, x_max); 
        });

        self
    }

    pub fn ylim(&mut self, y_min: Option<f32>, y_max: Option<f32>) -> &mut Self {
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

    pub fn color_cycle(&mut self, cycle: impl Into<Palette>) -> &mut Self {
        self.view.write(|f| { 
            f.color_cycle(cycle);
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
        A: IntoArtist<Data> + 'static
    {
        let artist = artist.into_artist();

        self.view.write(|f| {
            let config = f.config().clone();

            f.data_mut().add_artist(artist, &config)
        })
    }
}

impl From<PolarChart> for View<PolarFrame> {
    fn from(chart: PolarChart) -> Self {
        chart.view.clone()
    }
}

impl From<&PolarChart> for View<PolarFrame> {
    fn from(chart: &PolarChart) -> Self {
        chart.view.clone()
    }
}

impl Default for PolarChart {
    fn default() -> Self {
        PolarChart::new(&ConfigArc::default())
    }
}

impl Drawable for PolarChart {
    fn draw(&mut self, renderer: &mut dyn Renderer) -> renderer::Result<()> {
        self.view.drawable().draw(renderer)
    }
}
