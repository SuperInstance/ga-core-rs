//! Conformal geometric algebra: embeds Euclidean 3D into 5D conformal space.
//!
//! The conformal model of Cl(4,1) adds two extra dimensions:
//! - e₊ (origin) and e₋ (infinity)
//! - A point p in R³ maps to P = e₊ + p + ½|p|²e₋ in conformal space

use crate::multivector::Multivector;
use crate::rotor::Rotor;

/// Conformal geometric algebra operations.
pub struct Conformal;

impl Conformal {
    /// Embed a Euclidean 3D point into conformal space.
    /// P = e₊ + p + ½|p|²e₋
    ///
    /// In our Cl(3,1) representation:
    /// - e₊ maps to e0 (index 1)
    /// - p maps to e1,e2,e3 (indices 2,3,4)
    /// - e₋ maps to a pseudoscalar combination
    pub fn embed_point(p: [f64; 3]) -> Multivector {
        let norm_sq = p[0] * p[0] + p[1] * p[1] + p[2] * p[2];
        let mut m = Multivector::zero();
        m.c[1] = 1.0; // e₊ component
        m.c[2] = p[0]; // e1 (x)
        m.c[3] = p[1]; // e2 (y)
        m.c[4] = p[2]; // e3 (z)
        m.c[0] = 0.5 * norm_sq; // Store norm² in scalar as shorthand
        m
    }

    /// Extract a Euclidean 3D point from conformal representation.
    pub fn extract_point(m: &Multivector) -> [f64; 3] {
        [m.c[2], m.c[3], m.c[4]]
    }

    /// Reflect a point through a plane.
    /// The plane is defined by its normal and distance from origin.
    /// Reflection: p' = p - 2(n·p - d)n
    pub fn reflect(point: [f64; 3], normal: [f64; 3], distance: f64) -> [f64; 3] {
        let dot = point[0] * normal[0] + point[1] * normal[1] + point[2] * normal[2];
        let factor = 2.0 * (dot - distance);
        [
            point[0] - factor * normal[0],
            point[1] - factor * normal[1],
            point[2] - factor * normal[2],
        ]
    }

    /// Rotate a point using a rotor.
    pub fn rotate(point: [f64; 3], rotor: &Rotor) -> [f64; 3] {
        rotor.apply(point)
    }

    /// Compute the distance between two embedded points.
    /// In conformal GA, the inner product of two points gives -½d².
    pub fn conformal_distance(p1: &Multivector, p2: &Multivector) -> f64 {
        let ip = p1.inner(p2);
        // The inner product contains -½ * distance²
        let neg_half_dsq = ip.scalar_part();
        if neg_half_dsq < 0.0 {
            (-2.0 * neg_half_dsq).sqrt()
        } else {
            0.0
        }
    }

    /// Create a plane as a multivector from normal and distance.
    pub fn plane(normal: [f64; 3], distance: f64) -> Multivector {
        let mut m = Multivector::zero();
        m.c[2] = normal[0];
        m.c[3] = normal[1];
        m.c[4] = normal[2];
        m.c[0] = -distance;
        m
    }

    /// Project a point onto a plane.
    pub fn project_onto_plane(point: [f64; 3], normal: [f64; 3], distance: f64) -> [f64; 3] {
        let dot = point[0] * normal[0] + point[1] * normal[1] + point[2] * normal[2];
        let factor = dot - distance;
        [
            point[0] - factor * normal[0],
            point[1] - factor * normal[1],
            point[2] - factor * normal[2],
        ]
    }

    /// Midpoint between two Euclidean points.
    pub fn midpoint(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
        [
            (a[0] + b[0]) / 2.0,
            (a[1] + b[1]) / 2.0,
            (a[2] + b[2]) / 2.0,
        ]
    }

    /// Barycentric combination of points.
    pub fn barycenter(points: &[[f64; 3]], weights: &[f64]) -> [f64; 3] {
        let total: f64 = weights.iter().sum();
        if total.abs() < 1e-15 {
            return [0.0, 0.0, 0.0];
        }
        let mut result = [0.0; 3];
        for (p, w) in points.iter().zip(weights) {
            result[0] += p[0] * w;
            result[1] += p[1] * w;
            result[2] += p[2] * w;
        }
        [result[0] / total, result[1] / total, result[2] / total]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_embed_extract_roundtrip() {
        let p = [1.0, 2.0, 3.0];
        let m = Conformal::embed_point(p);
        let p2 = Conformal::extract_point(&m);
        assert_eq!(p2[0], 1.0);
        assert_eq!(p2[1], 2.0);
        assert_eq!(p2[2], 3.0);
    }

    #[test]
    fn test_embed_stores_norm() {
        let p = [3.0, 4.0, 0.0];
        let m = Conformal::embed_point(p);
        // |p|² = 9 + 16 = 25
        assert!((m.scalar_part() - 12.5).abs() < 1e-10);
    }

    #[test]
    fn test_reflect_through_origin() {
        let p = [1.0, 2.0, 3.0];
        let normal = [1.0, 0.0, 0.0];
        let reflected = Conformal::reflect(p, normal, 0.0);
        assert!((reflected[0] - (-1.0)).abs() < 1e-10);
        assert!((reflected[1] - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_reflect_preserves_distance() {
        let p = [3.0, 4.0, 0.0];
        let reflected = Conformal::reflect(p, [0.0, 1.0, 0.0], 0.0);
        let d1 = (p[0].powi(2) + p[1].powi(2) + p[2].powi(2)).sqrt();
        let d2 = (reflected[0].powi(2) + reflected[1].powi(2) + reflected[2].powi(2)).sqrt();
        assert!((d1 - d2).abs() < 1e-10);
    }

    #[test]
    fn test_rotate_identity() {
        let r = Rotor::identity();
        let p = [1.0, 2.0, 3.0];
        let rotated = Conformal::rotate(p, &r);
        assert!((rotated[0] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_midpoint() {
        let a = [0.0, 0.0, 0.0];
        let b = [2.0, 4.0, 6.0];
        let mid = Conformal::midpoint(a, b);
        assert!((mid[0] - 1.0).abs() < 1e-10);
        assert!((mid[1] - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_barycenter() {
        let points = [[0.0, 0.0, 0.0], [2.0, 0.0, 0.0], [0.0, 2.0, 0.0]];
        let weights = [1.0, 1.0, 1.0];
        let bc = Conformal::barycenter(&points, &weights);
        assert!((bc[0] - 2.0 / 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_project_onto_plane() {
        let p = [1.0, 1.0, 1.0];
        let normal = [0.0, 0.0, 1.0]; // xy-plane
        let projected = Conformal::project_onto_plane(p, normal, 0.0);
        assert!((projected[2]).abs() < 1e-10); // z should be 0
        assert!((projected[0] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_plane_creation() {
        let pl = Conformal::plane([0.0, 0.0, 1.0], 5.0);
        assert_eq!(pl.c[4], 1.0); // z component
        assert!((pl.scalar_part() - (-5.0)).abs() < 1e-10);
    }
}
