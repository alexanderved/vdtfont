use crate::point::{Point, PointId};

/// A pixel of [`VoronoiImage`].
pub struct Pixel {
    x: usize,
    y: usize,

    nearest_site: Point,
    nearest_site_id: PointId,
}

impl Pixel {
    // Creates a new [`Pixel`].
    pub(crate) fn new(x: usize, y: usize, raw_data: [i64; 3]) -> Self {
        Self {
            x,
            y,
            nearest_site: Point::new(raw_data[0] as f32, raw_data[1] as f32),
            nearest_site_id: raw_data[2],
        }
    }

    /// Returns the `x` coordinate of the pixel.
    pub fn x(&self) -> usize {
        self.x
    }

    /// Returns the `y` coordinate of the pixel.
    pub fn y(&self) -> usize {
        self.y
    }

    /// Returns a site which is nearest to the pixel.
    pub fn nearest_site(&self) -> &Point {
        &self.nearest_site
    }

    /// Returns an id of the site which is nearest to the pixel.
    pub fn nearest_site_id(&self) -> PointId {
        self.nearest_site_id
    }
}
