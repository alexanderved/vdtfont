use crate::point::PointId;

use std::convert;

/// The ids of the points which bound the [`Delaunay`].
#[derive(Debug, Clone, Copy)]
pub struct Bounds([PointId; 4]);

impl Bounds {
    /// Creates a new [`Bounds`] from the given `point_ids`.
    pub fn new(point_ids: [PointId; 4]) -> Self {
        Self(point_ids)
    }

    /// Checks if [`Bounds`] contains the given `point_id`.
    pub fn contains(&self, point_id: PointId) -> bool {
        self.0.contains(&point_id)
    }
}

impl convert::From<[PointId; 4]> for Bounds {
    fn from(point_ids: [PointId; 4]) -> Self {
        Self::new(point_ids)
    }
}

impl convert::From<Bounds> for [PointId; 4] {
    fn from(bounds: Bounds) -> Self {
        bounds.0
    }
}
