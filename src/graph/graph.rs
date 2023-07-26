use core::fmt;

use crate::{
    artist::{Artist, PlotArtist, PlotId},
    frame::{Data, LayoutArc, FrameId, FrameArtist, FrameTextOpt, AxisOpt, AspectMode}
};

use super::{style::{PlotOptArtist, PlotOpt}, GraphId};

#[derive(Clone)]
pub struct Graph {
    id: GraphId,
    frame_id: FrameId,

    layout: LayoutArc,
}

impl Graph {
    pub(crate) fn new(id: GraphId, frame_id: FrameId, layout: LayoutArc) -> Self {
        let mut graph = Self {
            id,
            frame_id, 
            layout,
        };

        graph.default_properties();

        graph
    }

    #[inline]
    pub fn id(&self) -> GraphId {
        self.id
    }

    #[inline]
    pub fn frame_id(&self) -> FrameId {
        self.frame_id
    }

    fn text_opt(&self, artist: FrameArtist) -> FrameTextOpt {
        let layout = self.layout.clone();
        self.layout.read(|l| l.frame(self.frame_id).text_opt(layout, artist))
    }

    pub fn title(&mut self, label: &str) -> FrameTextOpt {
        let mut opt = self.text_opt(FrameArtist::Title);
        opt.label(label);
        opt
    }

    pub fn x(&mut self) -> AxisOpt {
        let layout = self.layout.clone();

        AxisOpt::new(layout, self.frame_id(), FrameArtist::X)
    }

    pub fn y(&mut self) -> AxisOpt {
        let layout = self.layout.clone();

        AxisOpt::new(layout, self.frame_id(), FrameArtist::Y)
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

    pub fn aspect(&mut self, aspect: f32) -> &mut Self {
        self.layout.write(|l| {
            l.frame_mut(self.frame_id)
            .data_mut()
            .aspect(aspect);
        });

        self
    }

    pub fn aspect_mode(&mut self, mode: AspectMode) -> &mut Self {
        self.layout.write(|l| {
            l.frame_mut(self.frame_id)
            .data_mut()
            .aspect_mode(mode);
        });

        self
    }

    pub fn flip_y(&mut self, is_flip_y: bool) -> &mut Self {
        self.layout.write(|l| {
            l.frame_mut(self.frame_id)
            .data_mut()
            .flip_y(is_flip_y);
        });

        self
    }

    pub fn colorbar(&mut self) -> &mut Self {
        let _id = self.layout.write(|l|
            l.frame_mut(self.frame_id)
            .colorbar()
        );

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
        self.add_plot_artist(PlotOptArtist::new(artist))
    }

    pub fn add_plot_artist<'a, A>(
        &mut self, 
        artist: A,
    ) -> A::Opt 
    where
        A: PlotArtist<Data> + 'static
    {
        let id = self.layout.write(|l|
            l.frame_mut(self.frame_id)
            .data_mut()
            .add_artist(artist)
        );

        let plot_id = PlotId::new(
            self.layout.clone(),
            id
        );

        self.layout.write(move |layout| {
            let config = layout.config().clone();

            layout
                .frame_mut(id.frame())
                .data_mut()
                .artist_mut::<A>(id)
                .config(&config, plot_id)
        })
    }
}

impl fmt::Debug for Graph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pos = self.layout.read(|l| l.frame(self.frame_id).pos().clone());

        write!(f, "Graph[{}]({},{}; {}x{})",
            self.frame_id.index(),
            pos.xmin(),
            pos.ymin(),
            pos.width(),
            pos.height(),
        )
    }
}
