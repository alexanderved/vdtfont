pub mod curve;
pub mod glyph;
pub mod outliner;

pub use glyph::{Glyph, OutlinedGlyph, TriangulatedGlyph};

use crate::delaunay::{Delaunay, DelaunayFactory, DelaunayTriangleHandle, DelaunayTriangle};
use crate::point::{Point, PointHandle, PointId};
use crate::voronoi::VoronoiImageFactory;

use arena_system::{Arena, Handle};
use ocl::prm::Float2;
use ttfp::AsFaceRef;

const MIN_GLYPH_HEIGHT: usize = 64;
const MAX_GLYPH_HEIGHT: usize = 2048;
const MIN_POINT_DISTANCE: f32 = 2.0;

pub struct Font {
    subtables: ttfp::PreParsedSubtables<'static, ttfp::OwnedFace>,

    voronoi_image_factory: VoronoiImageFactory,
    delaunay_factory: DelaunayFactory,
}

impl Font {
    #[inline]
    pub fn from_vec(data: Vec<u8>) -> anyhow::Result<Self> {
        Self::from_vec_and_index(data, 0)
    }

    #[inline]
    pub fn from_vec_and_index(data: Vec<u8>, index: u32) -> anyhow::Result<Self> {
        let platform = ocl::Platform::default();
        let device = ocl::Device::list(platform, Some(ocl::DeviceType::GPU))?[0];
        let context = ocl::Context::builder().platform(platform).devices(device).build()?;
        let queue =
            ocl::Queue::new(&context, device, Some(ocl::CommandQueueProperties::PROFILING_ENABLE))?;

        Ok(Self {
            subtables: ttfp::PreParsedSubtables::from(ttfp::OwnedFace::from_vec(data, index)?),
            voronoi_image_factory: VoronoiImageFactory::new(queue.clone(), MAX_GLYPH_HEIGHT)?,
            delaunay_factory: DelaunayFactory::new(queue)?,
        })
    }

    #[inline]
    pub fn units_per_em(&self) -> Option<f32> {
        Some(self.subtables.as_face_ref().units_per_em().into())
    }

    #[inline]
    pub fn ascender(&self) -> f32 {
        self.subtables.as_face_ref().ascender().into()
    }

    #[inline]
    pub fn descender(&self) -> f32 {
        self.subtables.as_face_ref().descender().into()
    }

    #[inline]
    pub fn line_gap(&self) -> f32 {
        self.subtables.as_face_ref().line_gap().into()
    }

    #[inline]
    pub fn glyph(&self, c: char) -> Glyph {
        let index = self.subtables.glyph_index(c).map(|id| id.0).unwrap_or(0);
        Glyph(index)
    }

    #[inline]
    pub fn hor_advance(&self, glyph: Glyph) -> f32 {
        self.subtables
            .as_face_ref()
            .glyph_hor_advance(glyph.into())
            .expect("Invalid glyph_hor_advance")
            .into()
    }

    #[inline]
    pub fn hor_side_bearing(&self, glyph: Glyph) -> f32 {
        self.subtables
            .as_face_ref()
            .glyph_hor_side_bearing(glyph.into())
            .expect("Invalid glyph_hor_side_bearing")
            .into()
    }

    #[inline]
    pub fn ver_advance(&self, glyph: Glyph) -> f32 {
        self.subtables
            .as_face_ref()
            .glyph_ver_advance(glyph.into())
            .expect("Invalid glyph_ver_advance")
            .into()
    }

    #[inline]
    pub fn ver_side_bearing(&self, glyph: Glyph) -> f32 {
        self.subtables
            .as_face_ref()
            .glyph_ver_side_bearing(glyph.into())
            .expect("Invalid glyph_ver_side_bearing")
            .into()
    }

    #[inline]
    pub fn hor_kerning(&self, first: Glyph, second: Glyph) -> f32 {
        self.subtables
            .glyphs_hor_kerning(first.into(), second.into())
            .map(f32::from)
            .unwrap_or_default()
    }

    #[inline]
    pub fn glyph_count(&self) -> usize {
        self.subtables.as_face_ref().number_of_glyphs() as _
    }

    pub fn outline_glyph(&self, glyph: Glyph) -> OutlinedGlyph {
        let mut outliner = outliner::Outliner::new();
        let rect = self
            .subtables
            .as_face_ref()
            .outline_glyph(glyph.into(), &mut outliner)
            .unwrap();

        let dim = nearest_power_of_two(
            (MAX_GLYPH_HEIGHT as f32 * MIN_POINT_DISTANCE / outliner.shortest_distance) as usize,
        )
        .clamp(MIN_GLYPH_HEIGHT, MAX_GLYPH_HEIGHT);

        let height: f32 = self.ascender() - self.descender();
        let h_factor = dim as f32 / height;
        let v_factor = dim as f32 / height;

        let bounds = ttfp::Rect {
            x_min: (rect.x_min as f32 * h_factor) as i16,
            x_max: (rect.x_max as f32 * h_factor) as i16,
            y_min: (rect.y_min as f32 * v_factor) as i16,
            y_max: (rect.y_max as f32 * v_factor) as i16,
        };

        outliner.points.iter_mut().for_each(|p| {
            let new_x = p.x() * h_factor - bounds.x_min as f32;
            let new_y = bounds.height() as f32 - p.y() * v_factor + bounds.y_min as f32;

            p.set_coords(Float2::new(new_x, new_y));
        });

        let points: Arena<Point> = outliner.points.into();

        OutlinedGlyph::new(glyph, dim, bounds, points)
    }

    pub fn triangulate_glyph(
        &mut self,
        outlined_glyph: OutlinedGlyph
    ) -> anyhow::Result<TriangulatedGlyph> {
        let (glyph, dim, _, points) = outlined_glyph.into_raw_parts();
        let voronoi_image = self.voronoi_image_factory.construct_borrowed(points, dim)?;
        let mut delaunay = self.delaunay_factory.construct(&voronoi_image)?;

        self.insert_constraint_edges(&mut delaunay);

        let bounding_point_ids: [PointId; 4] = delaunay.bounds().into();
        let bounding_triangle = delaunay
            .points()
            .handle::<PointHandle>(bounding_point_ids[0].into(), Some(delaunay.triangles()))
            .triangle_fan()[0];
        self.remove_excess_triangles(bounding_triangle, false);

        let (dim, points, triangles, _) = delaunay.into_raw_parts();
        let triangles = triangles
            .handle_iter::<DelaunayTriangleHandle>(&points)
            .filter(|t| t.get().is_ok())
            .filter(|t| t.is_visible())
            .map(|t| *t.get().unwrap())
            .collect::<Arena<DelaunayTriangle>>();

        Ok(TriangulatedGlyph::new(glyph, dim, points, triangles))
    }

    fn insert_constraint_edges(&self, delaunay: &mut Delaunay) {
        let mut edges: Vec<[i64; 2]> = vec![];
        delaunay
            .points()
            .handle_iter::<PointHandle>(Some(delaunay.triangles()))
            .for_each(|p| {
                let pp = p.previous_in_outline();
                if !p.is_connected_to(pp)
                    && !p.index().is_invalid()
                    && !pp.index().is_invalid()
                    && !p.triangle_fan().is_empty()
                    && !pp.triangle_fan().is_empty()
                {
                    edges.push([p.index().into(), pp.index().into()]);
                }
            });

        edges
            .into_iter()
            .for_each(|e| {
                delaunay.insert_edge(e);
            });
    }

    #[allow(clippy::only_used_in_recursion)]
    fn remove_excess_triangles(
        &self,
        starting_triangle: DelaunayTriangleHandle,
        is_visible: bool,
    ) {
        if starting_triangle.is_finalized() { return; }

        starting_triangle.set_is_visible(is_visible);
        starting_triangle.set_is_finalized(true);

        starting_triangle.neighbours().into_iter().for_each(|n| {
            let has_contour_edge = starting_triangle.shared_edge_with(&n).is_contour();
            let is_visible = if has_contour_edge { !is_visible } else { is_visible };

            self.remove_excess_triangles(n, is_visible);
        });
    }
}

fn is_power_of_two(n: usize) -> bool {
    2usize.pow(n.ilog2()) == n
}

fn nearest_power_of_two(n: usize) -> usize {
    if is_power_of_two(n) {
        return n;
    }

    2usize.pow(n.ilog2() + 1)
}
