use crate::point::PointId;

///
#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[repr(C)]
pub(super) struct TriangleFan {
    pub(super) center: PointId,
    pub(super) triangle_offset: i32,
    pub(super) triangle_number: i32,
    pub(super) last_triangle_index: i32,
}

impl TriangleFan {
    pub(super) fn new(center: PointId) -> Self {
        TriangleFan { center, triangle_offset: -1, triangle_number: 0, last_triangle_index: 0 }
    }
}

unsafe impl ocl::OclPrm for TriangleFan {}
