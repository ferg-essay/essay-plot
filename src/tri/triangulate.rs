use essay_tensor::tensor::Tensor;

use super::triangulate;

pub struct Triangulation {
    xy: Tensor,
    triangles: Tensor<usize>,
}

impl Triangulation {
    pub fn new(xy: impl Into<Tensor>, triangles: impl Into<Tensor<usize>>) -> Self {
        let xy = xy.into();
        let triangles = triangles.into();
        assert!(xy.rank() == 2, "xy must be a 2D list (rank-2) {:?}", xy.shape());
        assert!(xy.cols() == 2, "xy must be a 2D list (rank-2) {:?}", xy.shape());

        assert!(triangles.rank() == 2, "triangles must be a list of triple indices (rank-2) {:?}", xy.shape());
        assert!(triangles.cols() == 3, "triangles must be a list of triple indices (rank-2) {:?}", xy.shape());

        Self {
            xy,
            triangles,
        }
    }

    pub fn vertices(&self) -> &Tensor {
        &self.xy
    }

    pub fn triangles(&self) -> &Tensor<usize> {
        &self.triangles
    }

    pub fn edges(&self) -> Tensor<usize> {
        let mut edges = Vec::<[usize; 2]>::new();

        for triangle in self.triangles.iter_row() {
            let (a, b, c) = (triangle[0], triangle[1], triangle[2]);

            if a < b {
                edges.push([a, b]);
            } else {
                edges.push([b, a]);
            }

            if b < c {
                edges.push([b, c]);
            } else {
                edges.push([c, b]);
            }

            if c < a {
                edges.push([c, a]);
            } else {
                edges.push([a, c]);
            }
        }

        Tensor::from(edges)
    }
}

impl From<Tensor> for Triangulation {
    fn from(value: Tensor) -> Self {
        triangulate(&value)
    }
}

impl From<&Tensor> for Triangulation {
    fn from(value: &Tensor) -> Self {
        triangulate(value)
    }
}