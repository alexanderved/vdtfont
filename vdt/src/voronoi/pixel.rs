use super::site::{Site, SiteId};

pub struct Pixel {
    x: usize,
    y: usize,

    nearest_site: Site,
    nearest_site_id: SiteId,
}

impl Pixel {
    pub(crate) fn new(x: usize, y: usize, raw_data: [i64; 3]) -> Self {
        Self {
            x,
            y,
            nearest_site: Site::new(raw_data[0] as f32, raw_data[1] as f32),
            nearest_site_id: raw_data[2],
        }
    }

    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
    }

    pub fn nearest_site(&self) -> &Site {
        &self.nearest_site
    }

    pub fn nearest_site_id(&self) -> SiteId {
        self.nearest_site_id
    }
}
