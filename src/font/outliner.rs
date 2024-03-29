use super::curve::*;

use crate::point::{Point, PointId};

use owned_ttf_parser as ttfp;

// A glyph outliner.
pub(super) struct Outliner {
    pub(super) last: PointId,
    pub(super) last_move: PointId,
    pub(super) points: Vec<Point>,

    pub(super) shortest_distance: f32,
}

impl Outliner {
    pub(super) fn new() -> Self {
        Self { last: -1, last_move: -1, points: Vec::new(), shortest_distance: f32::MAX }
    }
}

impl Default for Outliner {
    fn default() -> Self {
        Self::new()
    }
}

impl ttfp::OutlineBuilder for Outliner {
    fn move_to(&mut self, x: f32, y: f32) {
        //eprintln!("M {x} {y}");

        let p = Point::new(x, y);
        self.points.push(p);

        self.last = self.points.len() as i64 - 1;
        self.last_move = self.points.len() as i64 - 1;
    }

    fn line_to(&mut self, x1: f32, y1: f32) {
        //eprintln!("L {x1} {y1}");

        let last = self.points.get(self.last as usize).unwrap().clone();
        let p1 = Point::with_previous(x1, y1, self.last);

        self.shortest_distance = self.shortest_distance.min(last.distance(&p1));

        self.points.push(p1);
        self.last = self.points.len() as i64 - 1;
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32) {
        //eprintln!("Q {x1} {y1}");

        let p1 = Point::new(x1, y1);
        let p2 = Point::new(x2, y2);

        let last = self.points.get(self.last as usize).unwrap().clone();
        tesselate_quadric_curve((last, p1, p2), &mut self.points);

        (self.last + 1..self.points.len() as i64).for_each(|i| {
            let p1 = self.points.get((i - 1) as usize).unwrap().clone();
            let p0 = self.points.get_mut(i as usize).unwrap();

            self.shortest_distance = self.shortest_distance.min(p0.distance(&p1));

            p0.set_previous_in_outline(i - 1);
        });

        self.last = self.points.len() as i64 - 1;
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32) {
        //eprintln!("C {x1} {y1} {x3} {y3}");

        let p1 = Point::new(x1, y1);
        let p2 = Point::new(x2, y2);
        let p3 = Point::new(x3, y3);

        let last = self.points.get(self.last as usize).unwrap().clone();
        tesselate_cubic_curve((last, p1, p2, p3), &mut self.points);

        (self.last + 1..self.points.len() as i64).for_each(|i| {
            let p1 = self.points.get((i - 1) as usize).unwrap().clone();
            let p0 = self.points.get_mut(i as usize).unwrap();

            self.shortest_distance = self.shortest_distance.min(p0.distance(&p1));

            p0.set_previous_in_outline(i - 1);
        });

        self.last = self.points.len() as i64 - 1;
    }

    fn close(&mut self) {
        //eprintln!("Z");

        self.points.pop();
        self.last -= 1;

        if let Some(m) = self.points.get_mut(self.last_move as usize) {
            m.set_previous_in_outline(self.last);
        }
    }
}
