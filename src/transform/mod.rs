use std::{f32::consts::TAU, marker::PhantomData, ops::Deref};

use essay_graphics::api::{renderer::Canvas, Affine2d, Bounds, Coord, Path, Point};
use essay_tensor::tensor::Tensor;

use crate::{artist::Stale, chart::Data};

pub trait Transform<M: Coord> {
    fn transform_point(&self, point: Point) -> Point;

    fn transform_tensor(&self, tensor: &Tensor) -> Tensor;

    fn transform_path(&self, path: &Path<M>) -> Path<Canvas>;
}

pub struct TransformAffine<M: Coord> {
    affine: Affine2d,
    marker: PhantomData<fn(M)>,
}

impl<M: Coord> TransformAffine<M> {
    pub fn new(affine: Affine2d) -> Self {
        Self {
            affine,
            marker: Default::default(),
        }
    }
}

impl<M: Coord> Transform<M> for TransformAffine<M> {
    fn transform_point(&self, point: Point) -> Point {
        self.affine.transform_point(point)
    }

    fn transform_tensor(&self, tensor: &Tensor) -> Tensor {
        self.affine.transform(tensor)
    }

    fn transform_path(&self, path: &Path<M>) -> Path<Canvas> {
        self.affine.transform_path(path)
    }
}

#[derive(Debug)]
pub struct PolarTransform {
    xf: f32,
    yf: f32,

    sx: f32,
    sy: f32,
    tx: f32,
    ty: f32,

    angle_coord: AngleCoord,
}

impl PolarTransform {
    pub fn new(
        data: Bounds<Data>,
        pos: Bounds<Canvas>,
        angle_coord: AngleCoord,
    ) -> Self {
        let ([tx, ty], [sx, sy]) = pos.into();

        let dx = data.width();
        let ymin = data.xmin();
        let ymax = data.ymax();
        let dy = ymin.abs().max(ymax.abs());

        let xform = Self {
            xf: angle_coord.max() / dx.max(f32::EPSILON),
            yf: dy.max(f32::EPSILON).recip(),
            sx: sx * 0.5,
            sy: sy * 0.5,
            tx: tx + sx * 0.5,
            ty: ty + sy * 0.5,

            angle_coord,
        };

        xform
    }

    fn transform(&self, x: f32, y: f32) -> [f32; 2] {
        let (sin, cos) = self.angle_coord.to_radians(x * self.xf).sin_cos();

        [
            self.tx + self.sx * cos * y * self.yf,
            self.ty + self.sy * sin * y * self.yf,
        ]
    }
}

impl Transform<Data> for PolarTransform {
    #[inline]
    fn transform_point(&self, point: Point) -> Point {
        let Point(x, y) = point;

        self.transform(x, y).into()
    }

    #[inline]
    fn transform_tensor(&self, tensor: &Tensor) -> Tensor {
        tensor.map_row(|row| {
            self.transform(row[0], row[1])
        })
    }

    #[inline]
    fn transform_path(&self, path: &Path<Data>) -> Path<Canvas> {
        path.map(|Point(x, y)| self.transform(x, y).into())
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum AngleCoord {
    Radians,
    Degrees,
}

impl AngleCoord {
    #[inline]
    pub fn to_radians(&self, angle: f32) -> f32 {
        match self {
            AngleCoord::Radians => angle,
            AngleCoord::Degrees => (- angle + 90.).to_radians()
        }
    }

    #[inline]
    pub fn max(&self) -> f32 {
        match self {
            AngleCoord::Radians => TAU,
            AngleCoord::Degrees => 360.,
        }
    }
}

pub struct ToCanvas<'a, M: Coord> {
    id: Stale,
    bounds: Bounds<M>,
    transform: &'a dyn Transform<M>,
}

impl<'a, M: Coord> ToCanvas<'a, M> {
    pub fn new(
        stale: Stale, 
        bounds: Bounds<M>,
        transform: &'a dyn Transform<M>
    ) -> Self {
        Self {
            id: stale,
            bounds,
            transform,
        }
    }

    #[inline]
    pub fn stale(&self) -> Stale {
        self.id
    }

    #[inline]
    pub fn bounds(&self) -> Bounds<M> {
        self.bounds
    }

    #[inline]
    pub fn get_transform(&self) -> &dyn Transform<M> {
        self.transform
    }

    #[inline]
    pub fn transform_point(&self, point: Point) -> Point {
        self.transform.transform_point(point)
    }

    #[inline]
    pub fn transform_path(&self, path: &Path<M>) -> Path<Canvas> {
        self.transform.transform_path(&path)
    }

    #[inline]
    pub fn transform_tensor(&self, tensor: &Tensor) -> Tensor {
        self.transform.transform_tensor(tensor)
    }

    /*
    pub(crate) fn matmul(&self, transform: &Affine2d) -> Self {
        Self {
            id: self.id, // todo: update id
            pos_frame: self.pos_frame,
            to_canvas: self.to_canvas.matmul(transform),
        }
    }
    */
}
