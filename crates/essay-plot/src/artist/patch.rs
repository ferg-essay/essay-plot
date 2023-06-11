use core::fmt;

use crate::{graph::{Point, Data, Affine2d, Bounds, Display, Canvas}, driver::{Renderer}};

use super::{Path, path::Angle, ArtistTrait, Color, StyleOpt, Artist, Style};

pub trait PatchTrait {
    fn get_path(&mut self) -> &Path;
}

pub struct DataPatch {
    patch: Box<dyn PatchTrait>,
    bounds: Bounds<Data>,
    affine: Affine2d,
    style: Style,
}

impl DataPatch {
    pub fn new(patch: impl PatchTrait + 'static) -> Self {
        let mut patch = Box::new(patch);

        let bounds = patch.get_path().get_bounds();
        // TODO:
        let bounds = Bounds::none();
        Self {
            patch,
            bounds,
            affine: Affine2d::eye(),
            style: Style::new(),
        }
    }

    fn style_mut(&mut self) -> &mut Style {
        &mut self.style
    }
}

impl ArtistTrait<Data> for DataPatch {
    fn get_bounds(&mut self) -> Bounds<Data> {
        self.bounds.clone()
    }

    fn draw(
        &mut self, 
        renderer: &mut dyn Renderer,
        to_device: &Affine2d,
        clip: &Bounds<Canvas>,
        style: &dyn StyleOpt,
    ) {
        todo!()
    }
}

pub struct DisplayPatch {
    bounds: Bounds<Canvas>,
    pos: Bounds<Canvas>,

    patch: Box<dyn PatchTrait>,
    to_canvas: Affine2d,
    style: Style,
}

impl DisplayPatch {
    pub fn new(patch: impl PatchTrait + 'static) -> Self {
        Self {
            bounds: Bounds::unit(),
            pos: Bounds::none(),

            patch: Box::new(patch),
            to_canvas: Affine2d::eye(),
            style: Style::new(),
        }
    }

    pub fn set_pos(&mut self, pos: Bounds<Canvas>) {
        self.pos = pos.clone();
        self.to_canvas = self.bounds.affine_to(&pos);
    }

    fn style_mut(&mut self) -> &mut Style {
        &mut self.style
    }
}

impl ArtistTrait<Canvas> for DisplayPatch {
    fn get_bounds(&mut self) -> Bounds<Canvas> {
        self.bounds.clone()
    }

    fn draw(
        &mut self, 
        renderer: &mut dyn Renderer,
        to_canvas: &Affine2d,
        clip: &Bounds<Canvas>,
        style: &dyn StyleOpt,
    ) {
        let to_canvas = to_canvas.matmul(&self.to_canvas);
        let path = self.patch.get_path().transform(&to_canvas);
        renderer.draw_path(
            &self.style, 
            &path,
            &to_canvas, 
            clip
        ).unwrap();
    }
}

pub struct PathPatch {
    path: Path,
}

impl PathPatch {
    pub fn new(path: Path) -> Self {
        Self {
            path
        }
    }
}

impl ArtistTrait<Canvas> for PathPatch {
    fn get_bounds(&mut self) -> Bounds<Canvas> {
        todo!()
    }

    fn draw(
        &mut self, 
        renderer: &mut dyn Renderer,
        to_canvas: &Affine2d,
        clip: &Bounds<Canvas>,
        style: &dyn StyleOpt,
    ) {
        let path = self.path.transform(&to_canvas);

        renderer.draw_path(
            style, 
            &path,
            &to_canvas, 
            clip
        ).unwrap();
    }
}

pub struct Line {
    p0: Point,
    p1: Point,

    path: Option<Path<Data>>,
}

impl Line {
    pub fn new(
        p0: impl Into<Point>,
        p1: impl Into<Point>,
    ) -> Self {
        Self {
            p0: p0.into(),
            p1: p1.into(),
            path: None,
        }
    }

    //pub fn color(&mut self, color: Color) {
    //    self.color = Some(color);
    //}
}

impl PatchTrait for Line {
    fn get_path(&mut self) -> &Path {
        if self.path.is_none() {
            let path = Path::<Data>::from([
                self.p0, self.p1,
            ]);

            self.path = Some(path);
        }
            
        match &self.path {
            Some(path) => path,
            None => todo!(),
        }        
    }
}

impl ArtistTrait<Data> for Line {
    fn get_bounds(&mut self) -> Bounds<Data> {
        self.get_path().get_bounds()
    }

    fn draw(
        &mut self, 
        renderer: &mut dyn Renderer,
        to_canvas: &Affine2d,
        clip: &Bounds<Canvas>,
        style: &dyn StyleOpt,
    ) {
        if let Some(path) = &self.path {
            let path = path.transform(&to_canvas);

            renderer.draw_path(style, &path, to_canvas, clip).unwrap();
        }
    }
}

impl fmt::Debug for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Line({:?}, {:?})", self.p0, self.p1)
    }
}

pub struct Wedge {
    center: Point,
    radius: f32,
    angle: (Angle, Angle),

    path: Option<Path<Data>>,
}

impl Wedge {
    pub fn new(
        center: Point,
        radius: f32,
        angle: (Angle, Angle),
    ) -> Self {
        Self {
            center,
            radius,
            angle,
            path: None,
        }
    }

    //pub fn color(&mut self, color: Color) {
    //    self.color = Some(color);
    //}
}

impl PatchTrait for Wedge {
    fn get_path(&mut self) -> &Path {
        if self.path.is_none() {
            let wedge = Path::wedge(self.angle);
            
            //println!("Wedge {:?}", wedge.codes());
            let transform = Affine2d::eye()
                .scale(self.radius, self.radius)
                .translate(self.center.x(), self.center.y());

            let wedge = wedge.transform::<Data>(&transform);

            self.path = Some(wedge);
        }

        match &self.path {
            Some(path) => path,
            None => todo!(),
        }        
    }
}

impl ArtistTrait<Data> for Wedge {
    fn get_bounds(&mut self) -> Bounds<Data> {
        self.get_path().get_bounds()
    }

    fn draw(
        &mut self, 
        renderer: &mut dyn Renderer,
        to_canvas: &Affine2d,
        clip: &Bounds<Canvas>,
        style: &dyn StyleOpt,
    ) {
        if let Some(path) = &self.path {
            let path = path.transform(to_canvas);
            renderer.draw_path(style, &path, to_canvas, clip).unwrap();
        }
    }
}

impl fmt::Debug for Wedge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Wedge(({}, {}), {}, [{}, {}])",
            self.center.x(), self.center.y(),
            self.radius,
            self.angle.0.to_degrees(), self.angle.1.to_degrees())
    }
}
