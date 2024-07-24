use std::ops::Deref;

use essay_graphics::api::{
    renderer::{Canvas, Event, Renderer}, Affine2d, Bounds, Clip, Coord, PathOpt
};

pub trait Artist<M: Coord> : Send {
    fn resize(&mut self, renderer: &mut dyn Renderer, pos: &Bounds<Canvas>);

    fn bounds(&mut self) -> Bounds<M>;
    
    fn draw(
        &mut self, 
        renderer: &mut dyn Renderer,
        to_canvas: &ToCanvas,
        clip: &Clip,
        style: &dyn PathOpt,
    );

    #[allow(unused_variables)]
    fn event(&mut self, renderer: &mut dyn Renderer, event: &Event) -> bool {
        false
    }
}

pub struct ToCanvas {
    pos_frame: Bounds<Canvas>,
    to_canvas: Affine2d,
}

impl ToCanvas {
    pub fn new(pos_frame: Bounds<Canvas>, to_canvas: Affine2d) -> Self {
        Self {
            pos_frame,
            to_canvas
        }
    }

    pub fn pos(&self) -> &Bounds<Canvas> {
        &self.pos_frame
    }

    pub fn to_canvas(&self) -> &Affine2d {
        &self.to_canvas
    }
}

impl Deref for ToCanvas {
    type Target = Affine2d;

    fn deref(&self) -> &Self::Target {
        self.to_canvas()
    }
}
