use super::point::PointId;

pub(crate) struct Bounds([PointId; 4]);

impl Bounds {
    pub(crate) fn new(point_ids: [PointId; 4]) -> Self {
        Self(point_ids)
    }

    pub(crate) fn contains(&self, point_id: PointId) -> bool {
        self.0.contains(&point_id)
    }
}
