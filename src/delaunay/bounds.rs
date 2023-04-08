use crate::point::PointId;

use std::convert;

#[derive(Debug, Clone, Copy)]
pub struct Bounds([PointId; 4]);

impl Bounds {
    pub fn new(point_ids: [PointId; 4]) -> Self {
        Self(point_ids)
    }

    pub fn contains(&self, point_id: PointId) -> bool {
        self.0.contains(&point_id)
    }
}

impl convert::From<[PointId; 4]> for Bounds {
    fn from(point_ids: [PointId; 4]) -> Self {
        Self::new(point_ids)
    }
}

impl convert::Into<[PointId; 4]> for Bounds {
    fn into(self) -> [PointId; 4] {
        self.0
    }
}