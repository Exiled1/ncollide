extern crate nalgebra as na;
#[macro_use]
extern crate ncollide_entities;
extern crate ncollide;

use std::mem;
use std::any::TypeId;
use na::{Vector2, Point2, Isometry2, Translation};
use ncollide::inspection::{Shape, Shape2, ShapeDesc2};
use ncollide::geometry::{self, Proximity};
use ncollide::shape::{CompositeShape, Cuboid2};
use ncollide::partitioning::BVT;
use ncollide::bounding_volume::{HasBoundingVolume, AABB2};

struct CrossedCuboids {
    bvt: BVT<usize, AABB2<f32>>
}

impl CrossedCuboids {
    pub fn new() -> CrossedCuboids {
        // The shape indices paired with their corresponding AABBs.
        // Nedded to initialize the acceleration structure.
        let aabbs = vec! [
            (0, CrossedCuboids::generate_aabb(0)),
            (1, CrossedCuboids::generate_aabb(1))
        ];

        CrossedCuboids {
            bvt: BVT::new_balanced(aabbs)
        }
    }

    // Helper function to generate the AABB bounding the i-th cuboid.
    fn generate_aabb(i: usize) -> AABB2<f32> {
        if i == 0 {
            // The AABB for the horizontal cuboid.
            AABB2::new(Point2::new(-1.0, 0.0), Point2::new(3.0, 2.0))
        }
        else {
            // The AABB for the vertical cuboid.
            AABB2::new(Point2::new(0.0, -1.0), Point2::new(2.0, 3.0))
        }
    }

    // Helper function to generate the i-th cuboid.
    fn generate_cuboid(i: usize) -> Cuboid2<f32> {
        if i == 0 {
            // Create a 4x2 cuboid. Remember that we must provide the
            // half-lengths.
            Cuboid2::new(Vector2::new(2.0, 1.0))
        }
        else {
            // Create a 2x4 cuboid. Remember that we must provide the
            // half-lengths.
            Cuboid2::new(Vector2::new(1.0, 2.0))
        }
    }
}

impl CompositeShape<Point2<f32>, Isometry2<f32>> for CrossedCuboids {
    fn len(&self) -> usize {
        2 // There are only two parts.
    }

    fn map_part_at(&self, i: usize, f: &mut FnMut(&Isometry2<f32>, &Shape2<f32>)) {
        // The translation needed to center the cuboid at the point (1, 1).
        let transform = Isometry2::new(Vector2::new(1.0, 1.0), na::zero());

        // Create the cuboid on-the-fly.
        let cuboid = CrossedCuboids::generate_cuboid(i);

        // Call the function.
        f(&transform, &cuboid)
    }

    fn map_transformed_part_at(&self,
                               i: usize,
                               m: &Isometry2<f32>,
                               f: &mut FnMut(&Isometry2<f32>, &Shape2<f32>)) {
        // Prepend the translation needed to center the cuboid at the point (1, 1).
        let transform = m.prepend_translation(&Vector2::new(1.0, 1.0));

        // Create the cuboid on-the-fly.
        let cuboid = CrossedCuboids::generate_cuboid(i);

        // Call the function.
        f(&transform, &cuboid)
    }

    fn aabb_at(&self, i: usize) -> AABB2<f32> {
        // Compute the i-th AABB.
        CrossedCuboids::generate_aabb(i)
    }

    fn bvt(&self) -> &BVT<usize, AABB2<f32>> {
        // Reference to the acceleration structure.
        &self.bvt
    }
}


impl_composite_shape_desc!(CrossedCuboids, Point2<f32>, Isometry2<f32>);

impl HasBoundingVolume<Isometry2<f32>, AABB2<f32>> for CrossedCuboids {
    fn bounding_volume(&self, m: &Isometry2<f32>) -> AABB2<f32> {
        // This is far from an optimal AABB.
        AABB2::new(Point2::new(-10.0, -10.0) + m.translation(),
                   Point2::new(10.0, 10.0)   + m.translation())
    }
}

fn main() {
    let cross  = CrossedCuboids::new();
    let cuboid = Cuboid2::new(Vector2::new(1.0, 1.0));

    let cross_pos  = na::one();
    let cuboid_pos = Isometry2::new(Vector2::new(6.0, 0.0), na::zero());

    let dist = geometry::distance(&cross_pos, &cross, &cuboid_pos, &cuboid);
    let prox = geometry::proximity(&cross_pos, &cross, &cuboid_pos, &cuboid, 0.0);
    let ctct = geometry::contact(&cross_pos, &cross, &cuboid_pos, &cuboid, 0.0);

    assert!(na::approx_eq(&dist, &2.0));
    assert_eq!(prox, Proximity::Disjoint);
    assert!(ctct.is_none());
}
