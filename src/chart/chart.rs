use essay_graphics::{
    api::Bounds,
    layout::{Layout, View}, 
};

use crate::{
    artist::Artist,
    chart::{AspectMode, AxisOpt, Data, ChartFrame, FrameArtist, FrameTextOpt, PlotArtist}
};

use super::{config::read_config, style::PlotOptHandle, ConfigArc, PlotOpt, Scaling};

pub struct ChartBuilder {
    config: ConfigArc,
    layout: Layout,
}

impl ChartBuilder {
    pub fn new(layout: Layout) -> Self {
        Self {
            config: read_config().into_arc(),

            layout,
        }
    }

    pub fn chart(&mut self, pos: impl Into<Bounds<Layout>>) -> Chart {
        Chart::new(self.layout.view(pos, ChartFrame::new(&self.config)))
    }

    pub fn get_layout_mut(&mut self) -> &mut Layout {
        &mut self.layout
    }

    pub fn into_layout(self) -> Layout {
        self.layout
    }
}

#[derive(Clone)]
pub struct Chart {
    view: View<ChartFrame>,
}

impl Chart {
    pub(crate) fn new(view: View<ChartFrame>) -> Self {
        let mut chart = Self {
            view
        };

        chart.default_properties();

        chart
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

    fn default_properties(&mut self) {
        //self.title.font().size(12.);
    }

    // TODO: should there be a plain add_artist that doesn't wrap PlotStyle?

    pub fn add_simple_artist<'a, A>(
        &mut self, 
        artist: A,
    ) -> PlotOpt
    where
        A: Artist<Data> + 'static
    {
        self.artist(PlotOptHandle::new(artist))
    }

    pub fn artist<'a, A>(
        &mut self, 
        artist: A,
    ) -> <A::Artist as PlotArtist>::Opt 
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

pub trait IntoArtist {
    type Artist : PlotArtist;

    fn into_artist(self) -> Self::Artist;
}

impl<A: PlotArtist> IntoArtist for A {
    type Artist = Self;

    fn into_artist(self) -> Self::Artist {
        self
    }
}
