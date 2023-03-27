use crate::point::{Point, PointId};

pub struct Pixel {
    x: usize,
    y: usize,

    nearest_site: Point,
    nearest_site_id: PointId,
}

impl Pixel {
    pub(crate) fn new(x: usize, y: usize, raw_data: [i64; 3]) -> Self {
        Self {
            x,
            y,
            nearest_site: Point::new(raw_data[0] as f32, raw_data[1] as f32, false, -1),
            nearest_site_id: raw_data[2],
        }
    }

    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
    }

    pub fn nearest_site(&self) -> &Point {
        &self.nearest_site
    }

    pub fn nearest_site_id(&self) -> PointId {
        self.nearest_site_id
    }
}
