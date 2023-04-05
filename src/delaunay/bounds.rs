use crate::point::PointId;

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
