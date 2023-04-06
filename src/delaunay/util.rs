use crate::point::PointHandle;

#[rustfmt::skip]
pub fn lines_intersect(line0: [PointHandle; 2], line1: [PointHandle; 2]) -> bool {
    let det = (line0[1].x() - line0[0].x()) * (line1[1].y() - line1[0].y())
        - (line1[1].x() - line1[0].x()) * (line0[1].y() - line0[0].y());

    if det == 0.0 {
        false
    } else {
        let lambda = ((line1[1].y() - line1[0].y()) * (line1[1].x() - line0[0].x())
            - (line1[1].x() - line1[0].x()) * (line1[1].y() - line0[0].y())) / det;
        let gamma = ((line0[0].y() - line0[1].y()) * (line1[1].x() - line0[0].x())
            + (line0[1].x() - line0[0].x()) * (line1[1].y() - line0[0].y())) / det;

        (0.0 < lambda && lambda < 1.0) && (0.0 < gamma && gamma < 1.0)
    }
}

pub fn are_lines_equal(line0: [PointHandle; 2], line1: [PointHandle; 2]) -> bool {
    line0[0] == line1[0] && line0[1] == line1[1] || line0[0] == line1[1] && line0[1] == line1[0]
}
