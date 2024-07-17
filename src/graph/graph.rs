use core::fmt;

use essay_graphics::{api::Color, layout::ViewHandle};

use crate::{
    artist::{Artist, IntoArtist, PlotArtist, TextCanvas},
    frame::{AspectMode, AxisOpt, Data, Frame, FrameArtist}
};

use super::{style::PlotOptHandle, PlotOpt};

#[derive(Clone)]
pub struct Graph {
    //id: GraphId,
    //frame_id: FrameId,

    //layout: LayoutArc,
    view: ViewHandle<Frame>,
}

impl Graph {
    /*
    pub(crate) fn new(id: GraphId, frame_id: FrameId, layout: LayoutArc) -> Self {
        let mut graph = Self {
            id,
            frame_id, 
            layout,
        };

        graph.default_properties();

        graph
    }
    */
    pub(crate) fn new(view: ViewHandle<Frame>) -> Self {
        let mut graph = Self {
            view
        };

        graph.default_properties();

        graph
    }

    /*
    #[inline]
    pub fn id(&self) -> GraphId {
        self.id
    }

    #[inline]
    pub fn frame_id(&self) -> FrameId {
        self.frame_id
    }
    */

    fn text_opt(&self, artist: FrameArtist) -> FrameTextOpt {
        FrameTextOpt::new(self.view.clone(), artist)
        // let layout = self.layout.clone();
        // self.layout.read(|l| l.frame(self.frame_id).text_opt(layout, artist))
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

    /*
    pub fn artist<'a, A>(
        &mut self, 
        artist: A,
    ) -> A::Opt 
    where
        A: PlotArtist<Data> + 'static
    */

    pub fn artist<'a, A>(
        &mut self, 
        artist: A,
    ) -> <A::Artist as PlotArtist>::Opt 
    where
        A: IntoArtist + 'static
    {
        let artist = artist.into_artist();

        self.view.write(|f| {
            let config = f.config().clone();

            f.data_mut().add_artist(artist, &config, self.view.clone())
        })

        /*
        let plot_id = PlotId::new(
            self.view.clone(),
            id
        );

        self.view.write(|frame| {
            let config = frame.config().clone();

            frame.data_mut()
                .artist_mut::<A::Artist>(id)
                .config(&config, plot_id)
                */

                /*
            layout
                .frame_mut(id.frame())
                .data_mut()
                .artist_mut::<A::Artist>(id)
                .config(&config, plot_id)
                */
        // })
    }
}

impl fmt::Debug for Graph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // let pos = self.layout.read(|l| l.frame(self.frame_id).pos().clone());

        write!(f, "Graph[{:?}]",
            self.view,
        )
    }
}

struct FrameTextOpt {
    view: ViewHandle<Frame>,
    artist: FrameArtist,
}

impl FrameTextOpt {
    fn new(view: ViewHandle<Frame>, artist: FrameArtist) -> Self {
        Self {
            view,
            artist,
        }
    }

    fn write(&mut self, fun: impl FnOnce(&mut TextCanvas)) {
        self.view.write(|frame| {
            fun(frame.get_text_mut(self.artist))
        })
    }

    pub fn label(&mut self, label: &str) -> &mut Self {
        self.write(|text| { text.label(label); });
        self
    }

    pub fn color(&mut self, color: impl Into<Color>) -> &mut Self {
        self.write(|text| { text.path_style_mut().color(color); });
        self
    }

    pub fn size(&mut self, size: f32) -> &mut Self {
        self.write(|text| { text.text_style_mut().size(size); });
        self
    }
}
