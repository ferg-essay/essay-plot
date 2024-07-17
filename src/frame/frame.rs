use std::f32::consts::PI;

use essay_graphics::{api::{
    driver::Renderer, Affine2d, Bounds, Canvas, CanvasEvent, Clip, Color, PathOpt, Point, VertAlign 
}, layout::ViewTrait};

use crate::{
    artist::{
        patch::CanvasPatch, paths, Artist, Colorbar, PathStyle, TextCanvas, ToCanvas
    }, 
    graph::{Config, ConfigArc}
};

use super::{data_box::DataBox, axis::{Axis, AxisTicks, XAxis, YAxis}, layout::FrameId, LayoutArc, legend::Legend, Data, ArtistId};

pub struct Frame {
    id: FrameId,
    
    pos: Bounds<Canvas>,

    config: ConfigArc,

    to_canvas: Affine2d,

    margins: FrameMargins,

    _is_share_x: bool,
    _is_share_y: bool,

    path_style: PathStyle,
    // prop_cycle

    data: DataBox,

    title: TextCanvas,

    bottom: BottomFrame,
    left: LeftFrame,
    top: TopFrame,
    right: RightFrame,

    //is_frame_visible: bool,

    legend: Legend,

    //is_stale: bool, 
    //aspect_ratio: Option<f32>,
    //box_aspect_ratio: Option<f32>,
    // is_visible
    // axis_locator
    // is_axis_below
    // label
    // adjustable (Box vs Data)
    // is_snap
    // transform
    // xbound (min, max)
    // xmargin
    // xscale - linear, log, symlog, logit
    // xticks - sets ticks and labels
    // ybound
    // ylabel
    // ylim
    // ymargin
    // yscale
    // yticks
    // zorder
}

impl Frame {
    pub(crate) fn new(cfg: &ConfigArc) -> Self {
        let id = FrameId(0);

        Self {
            id,

            config: cfg.clone(),

            pos: Bounds::none(),

            data: DataBox::new(id, cfg),

            title: TextCanvas::new(),

            margins: FrameMargins::new(cfg),

            bottom: BottomFrame::new(cfg),
            left: LeftFrame::new(cfg),
            top: TopFrame::new(),
            right: RightFrame::new(),

            path_style: PathStyle::default(),

            to_canvas: Affine2d::eye(),

            legend: Legend::new(cfg),

            //is_stale: true,
            _is_share_x: false,
            _is_share_y: false,
            //is_frame_visible: true,
            //aspect_ratio: None,
            //box_aspect_ratio: None,
        }
    }

    #[inline]
    pub fn id(&self) -> FrameId {
        self.id
    }

    pub(crate) fn pos(&self) -> &Bounds<Canvas> {
        &self.pos
    }

    pub(crate) fn config(&self) -> &ConfigArc {
        &self.config
    }

    /*
    pub(crate) fn _update(&mut self, canvas: &Canvas) {
        self.title.update(pos, canvas);

        self.data.update(pos, canvas);

        self.bottom.update_axis(&self.data);
        self.left.update_axis(&self.data);

        self.bottom.update(pos, canvas);
        self.left.update(pos, canvas);
        self.top.update(pos, canvas);
        self.right.update(pos, canvas);

        self.legend.update_handlers(&self.data);
        self.legend.update(pos, canvas);
    }
    */

    ///
    /// Sets the device bounds and propagates to children
    /// 
    /// The position for a frame is the size of the data box. The frame,
    /// axes and titles are relative to the data box.
    /// 
    pub(crate) fn _set_pos(&mut self, pos: &Bounds<Canvas>) -> &mut Self {
        self.pos = pos.clone();

        let title = self.title.get_extent();

        // title exists outside the pos bounds
        self.title.set_pos([
            pos.xmin(), pos.ymax(), 
            pos.xmax(), pos.ymax() + title.height()
        ]); 

        let pos_data = Bounds::<Canvas>::new(
            Point(pos.xmin(), pos.ymin()), 
            Point(pos.xmax(), pos.ymax()),
        );

        self.data.set_pos(&pos_data);

        let pos_data = self.data.get_pos();

        let pos_top = Bounds::<Canvas>::new(
            Point(pos_data.xmin(), pos_data.ymax()),
            Point(pos_data.xmax(), pos_data.ymax()),
        );
        self.top.set_pos(pos_top);

        let pos_right = Bounds::<Canvas>::new(
            Point(pos_data.xmax(), pos_data.ymin()),
            Point(pos_data.xmax(), pos_data.ymax()),
        );
        self.right.set_pos(pos_right);

        let pos_canvas = Bounds::<Canvas>::new(
            Point(pos_data.xmin(), pos_data.ymax()),
            Point(pos_data.xmin(), pos_data.ymax()),
        );
        self.legend.set_pos(pos_canvas);

        self
    }

    pub(crate) fn _data(&self) -> &DataBox {
        &self.data
    }

    pub(crate) fn data_mut(&mut self) -> &mut DataBox {
        &mut self.data
    }

    pub(crate) fn text_opt(&self, layout: LayoutArc, artist: FrameArtist) -> FrameTextOpt {
        match artist {
            FrameArtist::Title => FrameTextOpt::new(layout, self.id, artist),
            FrameArtist::XLabel => FrameTextOpt::new(layout, self.id, artist),
            FrameArtist::YLabel => FrameTextOpt::new(layout, self.id, artist),

            _ => panic!("Invalid artist {:?}", artist)
        }
    }

    pub(crate) fn get_text_mut(&mut self, artist: FrameArtist) -> &mut TextCanvas {
        match artist {
            FrameArtist::Title => &mut self.title,
            FrameArtist::XLabel => &mut self.bottom.title,
            FrameArtist::YLabel => &mut self.left.title,

            _ => panic!("Invalid text {:?}", artist)
        }
    }

    pub(crate) fn get_axis_mut(&mut self, artist: FrameArtist) -> &mut Axis {
        match artist {
            FrameArtist::X => self.bottom.axis_mut(),
            FrameArtist::Y => self.left.axis_mut(),

            _ => panic!("Invalid axis {:?}", artist)
        }
    }

    pub(crate) fn get_ticks_mut(&mut self, artist: FrameArtist) -> &mut AxisTicks {
        match artist {
            FrameArtist::XMajor => self.bottom.axis_mut().major_mut(),
            FrameArtist::XMinor => self.bottom.axis_mut().minor_mut(),
            FrameArtist::YMajor => self.left.axis_mut().major_mut(),
            FrameArtist::YMinor => self.left.axis_mut().minor_mut(),

            _ => panic!("Invalid axis-texts {:?}", artist)
        }
    }

    pub(crate) fn get_data_artist_mut<A>(&mut self, id: ArtistId) -> &mut A
    where
        A: Artist<Data> + 'static
    {
        self.data_mut().artist_mut(id)
    }

    pub(crate) fn colorbar(&mut self) {
        self.right.colorbar();
    }

    pub(crate) fn event(&mut self, renderer: &mut dyn Renderer, event: &CanvasEvent) {
        if self.data.get_pos().contains(event.point()) {
            if self.data.event(renderer, event) {
                self.left.update_axis(&self.data);
                self.bottom.update_axis(&self.data);

                renderer.request_redraw(&self.pos);
            };
        }
    }

    pub(crate) fn _draw(&mut self, renderer: &mut dyn Renderer) {
        let clip = Clip::from(&self.pos);

        let frame_to_canvas = ToCanvas::new(
            self.pos.clone(), 
            self.to_canvas.clone()
        );

        let to_canvas = ToCanvas::new(
            self.pos.clone(), 
            self.data.get_canvas_transform().clone()
        );

        self.title.draw(renderer, &to_canvas, &clip, &self.path_style);

        self.bottom.draw(renderer, &self.data, &frame_to_canvas, &clip, &self.path_style);
        self.left.draw(renderer, &self.data, &frame_to_canvas, &clip, &self.path_style);

        self.top.draw(renderer, &frame_to_canvas, &clip, &self.path_style);
        self.right.draw(renderer,  &frame_to_canvas, &clip, &self.path_style);

        // TODO: grid order
        self.data.draw(renderer, &to_canvas, &clip, &self.path_style);

        self.legend.draw(renderer, &frame_to_canvas, &clip, &self.path_style);
    }

    pub fn title(&mut self, text: &str) -> &mut TextCanvas {
        self.title.label(text);

        &mut self.title
    }

    pub fn xlabel(&mut self, text: &str) -> &mut TextCanvas {
        self.bottom.title(text)
    }

    pub fn ylabel(&mut self, text: &str) -> &mut TextCanvas {
        self.left.label(text)
    }
}

impl ViewTrait for Frame {
    fn update(&mut self, pos: &Bounds<Canvas>, canvas: &Canvas) {
        let pos = Bounds::from([
            pos.xmin() + pos.width() * self.margins.left,
            pos.ymin() + pos.height() * self.margins.top,
            pos.xmin() + pos.width() * self.margins.right,
            pos.ymin() + pos.height() * self.margins.bottom,
        ]);

        self.pos = pos.clone();

        let title = self.title.get_extent();

        // title exists outside the pos bounds
        self.title.update(&Bounds::from([
            pos.xmin(), pos.ymax(), 
            pos.xmax(), pos.ymax() + title.height()
        ]), canvas); 

        let pos_data = Bounds::<Canvas>::new(
            Point(pos.xmin(), pos.ymin()), 
            Point(pos.xmax(), pos.ymax()),
        );

        self.data.update(&pos_data, canvas);

        let pos_data = self.data.get_pos();

        let pos_top = Bounds::<Canvas>::new(
            Point(pos_data.xmin(), pos_data.ymax()),
            Point(pos_data.xmax(), pos_data.ymax()),
        );
        self.top.update(&pos_top, canvas);

        let pos_right = Bounds::<Canvas>::new(
            Point(pos_data.xmax(), pos_data.ymin()),
            Point(pos_data.xmax(), pos_data.ymax()),
        );
        self.right.update(&pos_right, canvas);

        let pos_canvas = Bounds::<Canvas>::new(
            Point(pos_data.xmin(), pos_data.ymax()),
            Point(pos_data.xmin(), pos_data.ymax()),
        );
        self.legend.update(&pos_canvas, canvas);

        // TODO:
        self.bottom.update_axis(&self.data);
        self.bottom.update(&pos, canvas);

        self.left.update_axis(&self.data);
        self.left.update(&pos, canvas);

        self.top.update(&pos, canvas);
        self.right.update(&pos, canvas);

        self.legend.update_handlers(&self.data);
        self.legend.update(&pos, canvas);
    }

    fn draw(&mut self, renderer: &mut dyn Renderer) {
        let clip = Clip::from(&self.pos);

        let frame_to_canvas = ToCanvas::new(
            self.pos.clone(), 
            self.to_canvas.clone()
        );

        let to_canvas = ToCanvas::new(
            self.pos.clone(), 
            self.data.get_canvas_transform().clone()
        );

        self.title.draw(renderer, &to_canvas, &clip, &self.path_style);

        self.bottom.draw(renderer, &self.data, &frame_to_canvas, &clip, &self.path_style);
        self.left.draw(renderer, &self.data, &frame_to_canvas, &clip, &self.path_style);

        self.top.draw(renderer, &frame_to_canvas, &clip, &self.path_style);
        self.right.draw(renderer,  &frame_to_canvas, &clip, &self.path_style);

        // TODO: grid order
        self.data.draw(renderer, &to_canvas, &clip, &self.path_style);

        self.legend.draw(renderer, &frame_to_canvas, &clip, &self.path_style);
    }
}
//impl Coord for Figure {}


#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FrameArtist {
    Title,
    X,
    Y,
    XMajor,
    XMinor,
    YMajor,
    YMinor,
    XLabel,
    YLabel,
}

pub struct FrameSizes {
    label_pad: f32,
}

impl FrameSizes {
    fn new(cfg: &Config) -> Self {
        Self {
            label_pad: match cfg.get_as_type("frame", "label_pad") {
                Some(pad) => pad,
                None => 0.,
            },
        }
    }
}

//
// Top Frame
//

pub struct TopFrame {
    bounds: Bounds<Canvas>,
    pos: Bounds<Canvas>,
    spine: Option<CanvasPatch>,
}

impl TopFrame {
    pub fn new() -> Self {
        Self {
            bounds: Bounds::new(Point(0., 0.), Point(0., 20.)),
            pos: Bounds::none(),
            spine: Some(CanvasPatch::new(paths::line(Point(0., 0.), Point(1., 0.)))),
        }
    }

    pub fn set_pos(&mut self, pos: Bounds<Canvas>) {
        self.pos = pos.clone();

        if let Some(spine) = &mut self.spine {
            spine.set_pos(Bounds::new(
                Point(pos.xmin(), pos.ymin()),
                Point(pos.xmax(), pos.ymin() + 1.),
            ))
        }
    }
}

impl Artist<Canvas> for TopFrame {
    fn update(&mut self, _pos: &Bounds<Canvas>, _canvas: &Canvas) {
    }
    
    fn get_extent(&mut self) -> Bounds<Canvas> {
        self.bounds.clone()
    }

    fn draw(
        &mut self, 
        renderer: &mut dyn Renderer,
        to_canvas: &ToCanvas,
        clip: &Clip,
        style: &dyn PathOpt,
    ) {
        if let Some(patch) = &mut self.spine {
            patch.draw(renderer, to_canvas, clip, style);
        }
        
    }
}

//
// Bottom frame
//

pub struct BottomFrame {
    sizes: FrameSizes,

    x_axis: XAxis,

    title: TextCanvas,
}

impl BottomFrame {
    pub fn new(cfg: &Config) -> Self {
        let mut frame = Self {
            sizes: FrameSizes::new(cfg),

            x_axis: XAxis::new(cfg, "x_axis"),

            title: TextCanvas::new(),
        };

        frame.title.text_style_mut().valign(VertAlign::Top);

        frame
    }

    pub fn update_axis(&mut self, data: &DataBox) {
        self.x_axis.update_axis(data); 
    }

    fn draw(
        &mut self, 
        renderer: &mut dyn Renderer,
        data: &DataBox,
        to_canvas: &ToCanvas,
        clip: &Clip,
        style: &dyn PathOpt,
    ) {
        let mut y = self.x_axis.draw(renderer, data, to_canvas, clip, style);
        y -= renderer.to_px(self.sizes.label_pad);

        self.title.set_pos(Bounds::new(
            Point(data.get_pos().xmin(), y),
            Point(data.get_pos().xmax(), y),
        ));
    
        self.title.draw(renderer, to_canvas, clip, style);
    }

    fn title(&mut self, text: &str) -> &mut TextCanvas {
        self.title.label(text)
    }

    fn update(&mut self, pos: &Bounds<Canvas>, canvas: &Canvas) {
        self.title.update(pos, canvas);
        self.x_axis.update(pos, canvas);
    }

    fn axis_mut(&mut self) -> &mut Axis {
        self.x_axis.axis_mut()
    }
}

//
// Left frame
//

pub struct LeftFrame {
    sizes: FrameSizes,

    y_axis: YAxis,

    title: TextCanvas,
}

impl LeftFrame {
    pub fn new(cfg: &Config) -> Self {
        let mut label = TextCanvas::new();
        label.angle(PI / 2.);

        let mut frame = Self {
            sizes: FrameSizes::new(cfg),

            y_axis: YAxis::new(cfg, "y_axis"),

            title: label,
        };

        frame.title.text_style_mut().valign(VertAlign::BaselineBottom);

        frame
    }

    pub fn update_axis(&mut self, data: &DataBox) {
        self.y_axis.update_axis(data);
    }

    fn draw(
        &mut self, 
        renderer: &mut dyn Renderer,
        data: &DataBox,
        to_canvas: &ToCanvas,
        clip: &Clip,
        style: &dyn PathOpt,
    ) {
        let mut x = self.y_axis.draw(renderer, data, to_canvas, clip, style);
        x -= renderer.to_px(self.sizes.label_pad);

        self.title.set_pos(Bounds::new(
            Point(x, data.get_pos().ymid()),
            Point(x, data.get_pos().ymid()),
        ));

        self.title.draw(renderer, to_canvas, clip, style);
    }

    fn label(&mut self, text: &str) -> &mut TextCanvas {
        self.title.label(text)
    }

    fn update(&mut self, pos: &Bounds<Canvas>, canvas: &Canvas) {
        self.title.update(pos, canvas);
        self.y_axis.update(pos, canvas);
    }

    fn axis_mut(&mut self) -> &mut Axis {
        self.y_axis.axis_mut()
    }
}

//
// Right frame
//

pub struct RightFrame {
    bounds: Bounds<Canvas>,
    pos: Bounds<Canvas>,
    spine: Option<CanvasPatch>,
    colorbar: Option<Colorbar>,
}

impl RightFrame {
    pub fn new() -> Self {
        Self {
            bounds: Bounds::new(Point(0., 0.), Point(20., 0.)),
            pos: Bounds::none(),
            spine: Some(CanvasPatch::new(paths::line(Point(0., 0.), Point(0., 1.)))),
            colorbar: None,
        }
    }

    pub fn set_pos(&mut self, pos: Bounds<Canvas>) {
        self.pos = pos.clone();

        if let Some(spine) = &mut self.spine {
            spine.set_pos(Bounds::new(
                Point(pos.xmin(), pos.ymin()),
                Point(pos.xmin() + 1., pos.ymax()),
            ))
        }

        if let Some(colorbar) = &mut self.colorbar {
            colorbar.set_pos(Bounds::new(
                Point(pos.xmin() + 40., pos.ymin()),
                Point(pos.xmin() + 80., pos.ymax()),
            ))
        }
    }

    pub fn colorbar(&mut self) {
        self.colorbar = Some(Colorbar::new());
    }
}

impl Artist<Canvas> for RightFrame {
    fn update(&mut self, pos: &Bounds<Canvas>, canvas: &Canvas) {
        if let Some(colorbar) = &mut self.colorbar {
            colorbar.update(pos, canvas);
        }
    }
    
    fn get_extent(&mut self) -> Bounds<Canvas> {
        self.bounds.clone()
    }

    fn draw(
        &mut self, 
        renderer: &mut dyn Renderer,
        to_canvas: &ToCanvas,
        clip: &Clip,
        style: &dyn PathOpt,
    ) {
        if let Some(patch) = &mut self.spine {
            patch.draw(renderer, to_canvas, clip, style);
        }

        if let Some(colorbar) = &mut self.colorbar {
            colorbar.draw(renderer, to_canvas, clip, style);
        }
    }
}

pub struct FrameTextOpt {
    layout: LayoutArc,
    id: FrameId,
    artist: FrameArtist,
}

impl FrameTextOpt {
    fn new(layout: LayoutArc, id: FrameId, artist: FrameArtist) -> Self {
        Self {
            layout,
            id,
            artist,
        }
    }

    fn write(&mut self, fun: impl FnOnce(&mut TextCanvas)) {
        self.layout.write(|l| {
            fun(l.frame_mut(self.id).get_text_mut(self.artist))
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

struct FrameMargins {
    top: f32,
    bottom: f32,
    left: f32,
    right: f32,
}

impl FrameMargins {
    fn new(cfg: &Config) -> Self {
        let bottom = match cfg.get_as_type("figure.subplot", "bottom") {
            Some(value) => value,
            None => 0.
        };

        let top = match cfg.get_as_type("figure.subplot", "top") {
            Some(value) => value,
            None => 1.
        };

        let left = match cfg.get_as_type("figure.subplot", "left") {
            Some(value) => value,
            None => 0.
        };

        let right = match cfg.get_as_type("figure.subplot", "right") {
            Some(value) => value,
            None => 1.
        };

        Self {
            bottom,
            top, 
            left,
            right, 
        }
    }
}
