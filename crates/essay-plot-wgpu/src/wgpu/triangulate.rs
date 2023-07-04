use std::{ops::Index, cmp::Ordering, collections::VecDeque};

use essay_plot_base::{Point, Canvas, Path, PathCode};

use super::bezier::intersection;

pub fn tri_new(path: &Path<Canvas>) -> Vec<Triangle> {
    let mut tri = Tri::new(path);

    tri.triangles()
}

// Seidel's algorithm

struct Tri {
    points: Vec<Point>,
    edges: Vec<Edge>,

    sort_edges: Vec<EdgeId>,
}

impl Tri {
    fn new(path: &Path<Canvas>) -> Self {
        let mut tri = Self {
            points: Vec::<Point>::new(),
            edges: Vec::new(),
            sort_edges: Vec::new(),
        };

        let codes = path.codes();
        assert!(matches!(codes[0], PathCode::MoveTo(_)));

        let mut prev = Point(f32::MAX, f32::MAX);
        let mut first = prev;
        let mut prev_edge = EdgeId(usize::MAX);
        let mut first_edge = EdgeId(usize::MAX);
        let mut n_poly = 0;

        for code in codes {
            match code {
                PathCode::MoveTo(p) => {
                    prev = *p;
                    first = prev;
                    prev_edge = EdgeId(tri.edges.len());
                    first_edge = EdgeId(tri.edges.len());
                },
                PathCode::LineTo(p) => {
                    // let point = tri.add_point(p);

                    prev_edge = tri.add_edge(prev, *p, prev_edge);

                    prev = *p;
                },
                PathCode::Bezier2(_, p) => {
                    // let point = tri.add_point(p);

                    prev_edge = tri.add_edge(prev, *p, prev_edge);

                    prev = *p;
                },
                PathCode::Bezier3(_, _, p) => {
                    // let point = tri.add_point(p);

                    prev_edge = tri.add_edge(prev, *p, prev_edge);

                    prev = *p;
                }
                PathCode::ClosePoly(p) => {
                    // let point = tri.add_point(p);

                    prev_edge = tri.add_edge(prev, *p, prev_edge);
                    prev_edge = tri.add_edge(*p, first, prev_edge);
                    tri.set_edge_pair(prev_edge, first_edge);
                    first_edge = EdgeId(usize::MAX);
                    prev_edge = EdgeId(usize::MAX);
                    prev = Point(f32::MAX, f32::MAX);
                },
            }
        }

        for edge in &tri.edges {
            println!("Edge {:?} fwd={:?} rev={:?}", edge.id, edge.forward, edge.reverse);

        }

        //let mut sorted_edges : Vec<EdgeId> = tri.edges.iter().map(|e| e.id).collect();

        //tri.sort_x(sorted_edges.as_mut_slice());

        //tri.sort_edges = sorted_edges;

        tri
    }

    fn triangles(&mut self) -> Vec<Triangle> {
        let mut tri = Vec::<Triangle>::new();

        let mut edges : Vec<EdgeId> = self.edges.iter().map(|e| e.id).collect();
        self.sort_x(edges.as_mut_slice());

        //let mut edges = VecDeque::from_iter(self.sort_edges.iter().map(|s| *s));

        while let Some(mut overlap) = self.get_overlap(&mut edges) {
            //let edge_id = edge_id;
            //let overlap = self.get_overlap(&edges, edge_id);

            println!("Overlap {:?}", overlap);

            if overlap.len() > 10 || self.edges.len() > 20 {
                break;
            }

            if overlap.len() < 2 {
                break;
            } else if overlap.len() == 2 {
                self.pop_triangle(&mut tri, &mut edges, overlap[0], overlap[1]); 
                // also add triangle
            } else if self.cut_cross(&mut edges, &mut overlap) {
                // cut and reloop
            //} else if overlap.len() == 3 {
            //    self.pop_triangle3(&mut tri, &mut edges, &overlap);
            } else {
                self.pop_triangle_multi(&mut tri, &mut edges, &mut overlap);
            }
        }

        tri
    }

    fn get_overlap(&self, edges: &mut Vec<EdgeId>) -> Option<Vec<EdgeId>> {
        if edges.len() < 2 {
            return None;
        }

        let len = edges.len();

        let e0 = edges[len - 1];
        let fwd = self.edges[e0.i()].forward;
        let rev = self.edges[e0.i()].reverse;
        //let e1 = edges[len - 2];

        let [p0, p1] = self.edge_points(e0);
        let [fwd0, fwd1] = self.edge_points(fwd);
        let [rev0, rev1] = self.edge_points(rev);

        let x_min = p0.x().min(p1.x());
        let fwd_min = fwd0.x().min(fwd1.x());
        let rev_min = rev0.x().min(rev1.x());

        // select neighbor closest to right to minimize overlap
        let x_min = if fwd_min < rev_min { 
            rev_min.min(x_min) 
        } else { 
            fwd_min.min(x_min)
        };
        
        let mut overlap = Vec::<EdgeId>::new();
        overlap.push(e0);
        // overlap.push(e1); // avoid double counting
        
        for i in (0..len - 1).rev() {
            let edge_id = edges[i];
            let [r0, r1] = self.edge_points(edge_id);

            let r_max = r0.x().max(r1.x());
        
            //let qx_min = q0.x();
            // let qx_max = q0.x().max(q1.x());
            println!("R1 xmin={:?} r1={:?}", x_min, r_max);
        
            if x_min < r_max {
                overlap.push(edge_id);
            } else {
                return Some(overlap);
            }
        }

        Some(overlap)
    }

    fn pop_triangle(
        &mut self, 
        tri: &mut Vec<Triangle>,
        edges: &mut Vec<EdgeId>, 
        e0: EdgeId, 
        e1: EdgeId
    ) {
        edges.pop();
        edges.pop();

        // TODO: fwd, rev
        let [p0, p1] = self.edges[e0.i()].points;
        let [q0, q1] = self.edges[e1.i()].points;

        let fwd = self.edges[e0.i()].forward;
        let rev = self.edges[e0.i()].reverse;

        if fwd == e1 {
            let fwd_fwd = self.edges[fwd.i()].forward;

            self.remove_edge(e0);
            self.remove_edge(fwd);

            if fwd_fwd != rev {
                let edge = self.add_edge(p0, q1, rev);
                edges.push(edge);

                self.set_edge_pair(edge, fwd);
            }

            tri.push(Triangle(p0, p1, q1));
        } else {
            let rev_rev = self.edges[rev.i()].reverse;

            self.remove_edge(e0);
            self.remove_edge(rev);

            if rev_rev != fwd {
                let edge = self.add_edge(q0, p1, rev_rev);
                edges.push(edge);

                self.set_edge_pair(edge, fwd);
            }

            tri.push(Triangle(q0, p0, p1));
        }

    }

    fn x_pop_triangle3(
        &mut self, 
        tri: &mut Vec<Triangle>,
        edges: &mut Vec<EdgeId>, 
        overlap: &Vec<EdgeId>,
    ) {
        let e0 = overlap[0];
        let e1 = overlap[1];
        let e2 = overlap[2];

        // TODO: fwd, rev
        let [p0, p1] = self.edges[e0.i()].points;
        let [q0, q1] = self.edges[e1.i()].points;
        let [r0, r1] = self.edges[e2.i()].points;

        edges.pop();
        edges.pop();

        if p1 == q1 || p1 == r1 {
            if p1 == r0 && q1 == r1 || p1 == r1 && q1 == r0 {
                edges.pop();
            } else {
                let edge = if p1.x() < q1.x() {
                // TODO: fix prev
                    self.add_edge(p1, q1, e0)
                } else {
                    self.add_edge(q1, p1, e0)
                };
                edges.push(edge);
            }

            tri.push(Triangle(p0, p1, q0));
        } else {
            println!("Unknown {:?},{:?} {:?},{:?} {:?},{:?}", p0, p1, q0, q1, r0, r1);
        }

    }

    fn pop_triangle_multi(
        &mut self, 
        tri: &mut Vec<Triangle>,
        edges: &mut Vec<EdgeId>, 
        overlap: &mut Vec<EdgeId>,
    ) {
        //self.sort_y(overlap.as_mut_slice());

        let e0 = overlap[0];
        //let e1 = overlap[1];
        //let e2 = overlap[2];
        //let e3 = overlap[3];

        let fwd = self.edges[e0.i()].forward;
        let rev = self.edges[e0.i()].reverse;

        let [p0, p1] = self.edges[e0.i()].points;

        let [fwd0, fwd1] = self.edges[fwd.i()].points;
        let [rev0, rev1] = self.edges[fwd.i()].points;

        let fwd_min = fwd0.x().min(fwd1.x());
        let rev_min = rev0.x().min(rev1.x());

        let (e1, q) = if fwd_min < rev_min {
            (rev, rev0)
        } else {
            (fwd, fwd1)
        };

        println!("Ovl {:?} {:?} {:?} {:?}", e0, fwd, rev, overlap);
        // TODO: fwd, rev
        // let [q0, q1] = self.edges[e1.i()].points;

        // TODO: add reverse method
        //let [s0, s1] = self.edges[e3.i()].points;

        println!("TriMult:\n  p=({:?},{:?})\n  q={:?}",
            p0, p1, q);

        for _ in 0..overlap.len() {
            edges.pop();
        }

        overlap.remove(0);

        if let Some((mp_e, mp)) = self.find_inner(overlap, p0, p1, q) {
            println!("Inner {:?} {:?} {:?} {:?}", p0, p1, q, mp);

            self.remove_edge(e0);

            let mp_fwd = self.edges[mp_e.i()].forward;
            let mp_rev = self.edges[mp_e.i()].reverse;
            let [mp_p0, mp_p1] = self.edges[mp_e.i()].points;

            if mp == mp_p0 {
                let e_a = self.add_edge(p0, mp, rev);
                let e_b = self.add_edge(mp, p1, mp_rev);

                self.set_edge_pair(e_a, mp_e);
                self.set_edge_pair(e_b, fwd);

                overlap.push(e_a);
                overlap.push(e_b);

                tri.push(Triangle(p0, p1, mp));
            } else {
                let e_a = self.add_edge(p0, mp, rev);
                let e_b = self.add_edge(mp, p1, mp_e);

                self.set_edge_pair(e_a, mp_fwd);
                self.set_edge_pair(e_b, fwd);

                overlap.push(e_a);
                overlap.push(e_b);

                tri.push(Triangle(p0, p1, mp));
            }
        } else if e1 == fwd {
            println!("TR-FWD {:?} {:?} {:?}", p0, p1, q);
            overlap.retain(|id| *id != fwd);

            let fwd_fwd = self.edges[fwd.i()].forward;

            self.remove_edge(e0);
            self.remove_edge(fwd);

            assert_eq!(p1, fwd0);
            tri.push(Triangle(p0, p1, fwd1));

            if fwd_fwd == rev {
                self.remove_edge(rev);
                overlap.retain(|id| *id != rev);
            } else {
                let edge = self.add_edge(p0, fwd1, rev);
                self.set_edge_pair(edge, fwd_fwd);
                overlap.push(edge);
            }
            println!("Ret_ovl {:?}", overlap);
        } else {
            println!("TR-ReV {:?} {:?} {:?}", p0, p1, q);
            overlap.retain(|id| *id != rev);

            let rev_rev = self.edges[rev.i()].reverse;

            self.remove_edge(e0);
            self.remove_edge(rev);

            tri.push(Triangle(rev0, p0, p1));

            if rev_rev == fwd {
                self.remove_edge(fwd);
                overlap.retain(|id| *id != fwd);
            } else {
                let edge = self.add_edge(rev0, p1, rev_rev);
                self.set_edge_pair(edge, fwd);
                overlap.push(edge);
            }
        }

        self.sort_x(overlap);

        for edge in overlap.drain(..) {
            edges.push(edge);
        }

        for edge in edges {
            let p = self.edges[edge.i()].points;
            println!("  Ed {:?}, {:?}", p[0], p[1]);
        }

        // TODO: sorting, etc.
        //println!("Tri p0={:?} p1={:?} q0={:?} q1={:?} r0={:?}", p0, p1, q0, q1, r0);
        //panic!("tri");
    }

    fn find_inner(
        &self, 
        overlap: &Vec<EdgeId>,
        p0: Point, 
        p1: Point, 
        q1: Point
    ) -> Option<(EdgeId, Point)> {
        for edge in overlap.iter().rev() {
            let mp = self.edges[edge.i()].points[1];

            if in_triangle(mp, p0, p1, q1) {
                return Some((*edge, mp));
            }
        }

        None
    }

    ///
    /// Two edges cross. Cut them and insert the new edges
    /// 
    fn cut_cross(&mut self, edges: &mut Vec<EdgeId>, overlap: &mut Vec<EdgeId>) -> bool {
        for i in 0..overlap.len() {
            for j in 0..i {
                let e_i = overlap[i];
                let e_j = overlap[j];

                if let Some(mp) = self.cross(e_i, e_j) {
                    let [p0, p1] = self.edge_points(e_i);
                    let [q0, q1] = self.edge_points(e_j);

                    println!("Cx-p {} {:?} {:?} {:?}", i, mp, p0, p1);
                    println!("Cx-q {} {:?} {:?} {:?}", j, mp, q0, q1);

                    for _ in 0..overlap.len() {
                        edges.pop();
                    }

                    overlap.remove(i);
                    overlap.remove(j);
            
                    let e_p1 = self.add_edge(mp, p1, EdgeId(0));
                    let e_p0 = self.add_edge(p0, mp, EdgeId(0));
                    let e_q1 = self.add_edge(mp, q1, EdgeId(0));
                    let e_q0 = self.add_edge(q0, mp, EdgeId(0));

                    overlap.push(e_p0);
                    overlap.push(e_q0);

                    overlap.push(e_p1);
                    overlap.push(e_q1);

                    self.sort_x(overlap.as_mut_slice());

                    for edge in overlap.drain(..).rev() {
                        edges.push(edge);
                    }
            
                    return true;
                }
            }
        }

        false
    }

    fn cross(&self, e0: EdgeId, e1: EdgeId) -> Option<Point> {
        let [p0, p1] = self.edges[e0.i()].points;
        let [q0, q1] = self.edges[e1.i()].points;

        let p_ymin = p0.y().min(p1.y());
        let p_ymax = p0.y().max(p1.y());

        let q_ymin = q0.y().min(q1.y());
        let q_ymax = q0.y().max(q1.y());

        if p0 == q0 || p0 == q1 || p1 == q0 || p1 == q1 {
            return None;
        }

        if q_ymax < p_ymin || p_ymax < q_ymin {
            return None;
        }
        let mp = intersection(p0, p1, q0, q1);

        if p0.x() < mp.x() && mp.x() < p1.x()
            && q0.x() < mp.x() && mp.x() < q1.x() {
            Some(mp)
        } else {
            None
        }
    }

    fn sort_x(&mut self, edges: &mut [EdgeId]) {
        // self.sort_edges = self.edges.iter().map(|e| e.id).collect();

        edges.sort_by(|a, b| {
            let [a0, a1] = &self.edges[a.i()].points;
            let [b0, b1] = &self.edges[b.i()].points;

            let a_min = a0.x().min(a1.x());
            let a_max = a0.x().max(a1.x());

            let b_min = b0.x().min(b1.x());
            let b_max = b0.x().max(b1.x());


            if a_max < b_max {
                Ordering::Less
            } else if b_max < a_max {
                Ordering::Greater
            } else if a_min < b_min {
                Ordering::Less
            } else if b_min < a_min {
                Ordering::Greater
            } else if a0.y() < b0.y() {
                Ordering::Less
            } else if b0.y() < a0.y() {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });
    }

    fn sort_y(&mut self, edges: &mut [EdgeId]) {
        // self.sort_edges = self.edges.iter().map(|e| e.id).collect();

        edges.sort_by(|a, b| {
            let [a0, a1] = &self.edges[a.i()].points;
            let [b0, b1] = &self.edges[b.i()].points;

            let a_min = a0.y().min(a1.y());
            let a_max = a0.y().max(a1.y());

            let b_min = b0.y().min(b1.y());
            let b_max = b0.y().max(b1.y());


            if a_min < b_min {
                Ordering::Less
            } else if b_min < a_min {
                Ordering::Greater
            } else if a_max < b_max {
                Ordering::Less
            } else if b_max < a_max {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });
    }

    fn edge_points(&self, id: EdgeId) -> [Point; 2] {
        self.edges[id.i()].points
    }

    fn add_point(&mut self, p: &Point) -> PointId {
        let id = PointId(self.points.len());
    
        self.points.push(p.clone());
    
        id
    }

    fn add_edge(&mut self, p0: Point, p1: Point, prev: EdgeId) -> EdgeId {
        if p0 == p1 {
            return prev;
        }

        let id = EdgeId(self.edges.len());

        let edge = Edge::new(id, p0, p1);
        self.edges.push(edge);

        if prev != id {
            self.set_edge_pair(prev, id);
        }
    
        id
    }

    fn remove_edge(&mut self, id: EdgeId) -> [EdgeId; 2] {
        let fwd = self.edges[id.i()].forward;
        let rev = self.edges[id.i()].reverse;

        self.edges[fwd.i()].reverse = rev;
        self.edges[rev.i()].forward = fwd;

        [fwd, rev]
    }

    fn set_edge_pair(&mut self, prev: EdgeId, next: EdgeId) {
        self.edges[prev.i()].forward = next;
        self.edges[next.i()].reverse = prev;
        println!("SEP prev={:?} next={:?}", prev, next);
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct PointId(usize);

impl PointId {
    fn i(&self) -> usize {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct EdgeId(usize);

impl EdgeId {
    fn i(&self) -> usize {
        self.0
    }
}

struct Edge {
    id: EdgeId,
    points: [Point; 2],
    forward: EdgeId,
    reverse: EdgeId,
}

impl Edge {
    fn new(
        id: EdgeId,
        p0: Point,
        p1: Point,
    ) -> Self {
        Self {
            id,
            points: [p0, p1],
            forward: id,
            reverse: id
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Triangle(Point, Point, Point);

pub fn triangulate(points: Vec<Point>) -> Vec<Triangle> {
    let mut points = points;
    let mut triangles = Vec::<Triangle>::new();
    
    let mut index = 0;
    let mut index_start = index;
    while points.len() >= 3 {
        let len = points.len();

        let p0 = points[index];
        let p1 = points[(index + 1) % len];
        let p2 = points[(index + 2) % len];

        if p0 == p1 || p1 == p2 {
            points.remove((index + 1) % len);
            index = index % points.len();
            continue;
        }

        let triangle = Triangle(p0, p1, p2);

        if triangle.is_inside(&points) {
            triangles.push(triangle);

            points.remove((index + 1) % len);

            index_start = index;
        }

        // TODO: fix empty poly

        index = (index + 1) % points.len();
        assert_ne!(index, index_start, "remaining points {:?}", points);
    }

    triangles
}

#[inline]
fn in_triangle(p: Point, a: Point, b: Point, c: Point) -> bool {
    let d1 = edge_sign(p, a, b);
    let d2 = edge_sign(p, b, c);
    let d3 = edge_sign(p, c, a);

    (d1 < 0.) && (d2 < 0.) && (d3 < 0.)
    || (d1 > 0.) && (d2 > 0.) && (d3 > 0.)
}

/// sign of the half-plane for p in a, b
#[inline]
fn edge_sign(p: Point, a: Point, b: Point) -> f32 {
    (p.0 - b.0) * (a.1 - b.1) - (a.0 - b.0) * (p.1 - b.1)
}

impl Triangle {
    fn is_inside(&self, polygon: &Vec<Point>) -> bool {
        let center = Point(
            (self.0.x() + self.1.x() + self.2.x()) / 3.,
            (self.0.y() + self.1.y() + self.2.y()) / 3.,
        );

        let mut n_crosses = 0;
        for i in 0..polygon.len() - 1 {
            let p0 = polygon[i];
            let p1 = polygon[i + 1];
            if center.is_below(&p0, &p1) {
                n_crosses += 1;
            }
        }

        let p0 = polygon[polygon.len() - 1];
        let p1 = polygon[0];

        if center.is_below(&p0, &p1) {
            n_crosses += 1;
        }

        n_crosses % 2 == 1
    }
}

impl Index<usize> for Triangle {
    type Output = Point;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => panic!("Invalid index")
        }
    }
}

#[cfg(test)]
mod test {
    use essay_plot_base::{Path, PathCode, Point, Canvas};

    use crate::wgpu::triangulate::Triangle;

    use super::tri_new;

    #[test]
    fn test_3() {
        let path = Path::<Canvas>::new(vec![
            PathCode::MoveTo(Point(0., 0.)),
            PathCode::LineTo(Point(1., 0.)),
            PathCode::ClosePoly(Point(1., 1.)),
        ]);

        assert_eq!(
            tri_new(&path), vec![
                Triangle(Point(1., 0.), Point(1., 1.), Point(0., 0.))
        ]);

        let path = Path::<Canvas>::new(vec![
            PathCode::MoveTo(Point(0., 0.)),
            PathCode::LineTo(Point(0., 1.)),
            PathCode::ClosePoly(Point(1., 1.)),
        ]);
    
        assert_eq!(
            tri_new(&path), vec![
                Triangle(Point(0., 1.), Point(1., 1.), Point(0., 0.))
        ]);
    }

    #[test]
    fn test_4_square() {
        let path = Path::<Canvas>::new(vec![
            PathCode::MoveTo(Point(0., 0.)),
            PathCode::LineTo(Point(1., 0.)),
            PathCode::LineTo(Point(1., 1.)),
            PathCode::ClosePoly(Point(0., 1.)),
        ]);

        assert_eq!(
            tri_new(&path), vec![
                Triangle(Point(1., 0.), Point(1., 1.), Point(0., 1.)),
                Triangle(Point(0., 0.), Point(1., 0.), Point(0., 1.)),
        ]);
    }

    #[test]
    fn test_4_cross() {
        let path = Path::<Canvas>::new(vec![
            PathCode::MoveTo(Point(0., 0.)),
            PathCode::LineTo(Point(1., 1.)),
            PathCode::LineTo(Point(1., 0.)),
            PathCode::ClosePoly(Point(0., 1.)),
        ]);

        assert_eq!(
            tri_new(&path), vec![
                Triangle(Point(0., 0.), Point(0.5, 0.5), Point(0., 1.)),
                Triangle(Point(0., 1.), Point(0., 0.), Point(0.5, 0.5))
        ]);
    }

    #[test]
    fn test_wedge_right() {
        let path = Path::<Canvas>::new(vec![
            PathCode::MoveTo(Point(0., 0.)),
            PathCode::LineTo(Point(10., 10.)),
            PathCode::LineTo(Point(1., 20.)),
            PathCode::ClosePoly(Point(2., 5.)),
        ]);

        assert_eq!(
            tri_new(&path), vec![
                Triangle(Point(10., 10.), Point(1., 20.), Point(2., 5.)),
                Triangle(Point(10., 10.), Point(2., 5.), Point(0., 0.))
        ]);
    }

    #[test]
    fn test_wedge_left() {
        let path = Path::<Canvas>::new(vec![
            PathCode::MoveTo(Point(10., 0.)),
            PathCode::LineTo(Point(0., 10.)),
            PathCode::LineTo(Point(9., 20.)),
            PathCode::ClosePoly(Point(8., 5.)),
        ]);

        assert_eq!(
            tri_new(&path), vec![
                Triangle(Point(8., 5.), Point(10., 0.), Point(0., 10.)),
                Triangle(Point(9., 20.), Point(8., 5.), Point(0., 10.))
        ]);
    }

    #[test]
    fn test_inner_tri() {
        let path = Path::<Canvas>::new(vec![
            PathCode::MoveTo(Point(0., 0.)),
            PathCode::LineTo(Point(10., 0.)),
            PathCode::ClosePoly(Point(10., 10.)),
            PathCode::MoveTo(Point(5., 1.)),
            PathCode::LineTo(Point(6., 1.)),
            PathCode::ClosePoly(Point(6., 2.)),
        ]);

        assert_eq!(
            tri_new(&path), vec![
                Triangle(Point(10., 0.), Point(10., 10.), Point(6., 1.)),
                Triangle(Point(6., 1.), Point(10., 10.), Point(5., 1.)),
                Triangle(Point(10., 0.), Point(6., 1.), Point(6., 6.)),

                Triangle(Point(10., 0.), Point(6., 6.), Point(6., 1.)),
                Triangle(Point(10., 0.), Point(6., 1.), Point(5., 1.)),
                Triangle(Point(5., 1.), Point(10., 10.), Point(0., 0.)),

                Triangle(Point(10., 0.), Point(5., 1.), Point(6., 1.)),
                Triangle(Point(0., 0.), Point(10., 1.), Point(6., 1.)),
                Triangle(Point(0., 0.), Point(6., 1.), Point(5., 1.)),

                Triangle(Point(6., 1.), Point(6., 6.), Point(5., 1.)),
        ]);
    }
}