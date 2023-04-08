use crate::point::Point;

const OBJSPACE_FLATNESS: f32 = 0.35;

pub fn tesselate_quadric_curve(p: (Point, Point, Point), points: &mut Vec<Point>) {
    let mp01 = p.0.midpoint(&p.1);
    let mp12 = p.1.midpoint(&p.2);
    let midpoint = mp01.midpoint(&mp12);
    let distance_squared = p.0.midpoint(&p.2).distance_squared(&midpoint);

    if distance_squared > OBJSPACE_FLATNESS * OBJSPACE_FLATNESS {
        let p0 = (p.0, mp01, midpoint.clone());
        let p1 = (midpoint, mp12, p.2);

        tesselate_quadric_curve(p0, points);
        tesselate_quadric_curve(p1, points);
    } else {
        points.push(p.2);
    }
}

pub fn tesselate_cubic_curve(p: (Point, Point, Point, Point), points: &mut Vec<Point>) {
    let longlen = p.0.distance(&p.1) + p.1.distance(&p.2) + p.2.distance(&p.3);
    let shortlen = p.0.distance(&p.3);
    let flatness_squared = longlen.powi(2) - shortlen.powi(2);

    let mp01 = p.0.midpoint(&p.1);
    let mp12 = p.1.midpoint(&p.2);
    let mp23 = p.2.midpoint(&p.3);

    let mp012 = mp01.midpoint(&mp12);
    let mp123 = mp12.midpoint(&mp23);

    let midpoint = mp012.midpoint(&mp123);

    if flatness_squared > OBJSPACE_FLATNESS * OBJSPACE_FLATNESS {
        let p0 = (p.0, mp01, mp012, midpoint.clone());
        let p1 = (midpoint, mp123, mp23, p.3);

        tesselate_cubic_curve(p0, points);
        tesselate_cubic_curve(p1, points);
    } else {
        points.push(p.3);
    }
}
