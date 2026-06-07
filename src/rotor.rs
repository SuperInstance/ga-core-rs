//! Rotors: even-grade multivectors for rotations in geometric algebra.

use crate::multivector::Multivector;

/// A rotor: an even-grade element that performs rotations.
///
/// A rotor R satisfies R * R̃ = 1 (normalized).
/// Rotations are applied via the sandwich product: v' = R v R̃.
///
/// In Cl(3,1), a rotor has scalar + bivector parts (8 components).
#[derive(Debug, Clone)]
pub struct Rotor {
    /// The underlying multivector (scalar + bivector parts only).
    pub inner: Multivector,
}

impl Rotor {
    /// Create from a multivector (will normalize).
    pub fn from_multivector(m: Multivector) -> Self {
        let mut r = Self { inner: m };
        r.normalize();
        r
    }

    /// Identity rotor (no rotation).
    pub fn identity() -> Self {
        Self {
            inner: Multivector::scalar(1.0),
        }
    }

    /// Create from axis-angle representation.
    /// axis should be a unit 3D vector, angle in radians.
    pub fn from_axis_angle(axis: [f64; 3], angle: f64) -> Self {
        let half = angle / 2.0;
        let cos_h = half.cos();
        let sin_h = half.sin();

        // Rotor = cos(θ/2) - sin(θ/2) * (axis as bivector)
        // In 3D GA, the rotation plane bivector B = axis_0*e23 - axis_1*e13 + axis_2*e12
        // But our bivector indices are: e01(5), e02(6), e03(7), e12(8), e13(9), e23(10)
        let mut m = Multivector::zero();
        m.c[0] = cos_h;
        // Using e12, e13, e23 for spatial bivectors
        m.c[10] = -sin_h * axis[0]; // e23
        m.c[9] = sin_h * axis[1]; // e13
        m.c[8] = -sin_h * axis[2]; // e12

        Self { inner: m }
    }

    /// Create a 180-degree rotation (reflection) about an axis.
    pub fn from_reflection(normal: [f64; 3]) -> Self {
        // Reflect = -n (as vector), then R = n1 * n2 for double reflection
        Self::from_axis_angle(normal, std::f64::consts::PI)
    }

    /// Compose two rotors (equivalent to composing rotations).
    pub fn compose(&self, other: &Self) -> Self {
        let product = self.inner.geometric_product(&other.inner);
        let mut r = Self { inner: product };
        r.normalize();
        r
    }

    /// Apply rotation to a 3D vector via sandwich product: v' = R v R̃.
    pub fn apply(&self, v: [f64; 3]) -> [f64; 3] {
        // Embed v as a multivector (e1, e2, e3 components at indices 2,3,4)
        let mut vm = Multivector::zero();
        vm.c[2] = v[0]; // e1
        vm.c[3] = v[1]; // e2
        vm.c[4] = v[2]; // e3

        let rev = self.inner.reverse();
        let result = self.inner.geometric_product(&vm).geometric_product(&rev);

        [result.c[2], result.c[3], result.c[4]]
    }

    /// Spherical linear interpolation between two rotors.
    pub fn slerp(&self, other: &Self, t: f64) -> Self {
        // Simplified: linear interpolation + normalization
        let diff = other.inner.sub(&self.inner);
        let interp = self.inner.add(&diff.scale(t));
        let mut r = Self { inner: interp };
        r.normalize();
        r
    }

    /// Normalize the rotor so that R * R̃ = 1.
    pub fn normalize(&mut self) {
        let norm_sq = self.inner.norm_squared();
        if norm_sq.abs() > 1e-15 {
            let scale = 1.0 / norm_sq.abs().sqrt();
            self.inner = self.inner.scale(scale);
        }
    }

    /// Extract a 3x3 rotation matrix.
    pub fn to_rotation_matrix(&self) -> [[f64; 3]; 3] {
        let e1 = self.apply([1.0, 0.0, 0.0]);
        let e2 = self.apply([0.0, 1.0, 0.0]);
        let e3 = self.apply([0.0, 0.0, 1.0]);
        [e1, e2, e3]
    }

    /// Check if this is approximately the identity.
    pub fn is_identity(&self, tolerance: f64) -> bool {
        let expected = Multivector::scalar(1.0);
        self.inner.sub(&expected).is_zero(tolerance)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity_rotor() {
        let r = Rotor::identity();
        let v = r.apply([1.0, 2.0, 3.0]);
        assert!((v[0] - 1.0).abs() < 1e-10);
        assert!((v[1] - 2.0).abs() < 1e-10);
        assert!((v[2] - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_identity_is_identity() {
        let r = Rotor::identity();
        assert!(r.is_identity(1e-10));
    }

    #[test]
    fn test_rotor_from_axis_angle() {
        let r = Rotor::from_axis_angle([0.0, 0.0, 1.0], std::f64::consts::FRAC_PI_2);
        // Rotor should have non-zero bivector part
        let bn = r.inner.grade_norm(2);
        assert!(bn > 0.1);
    }

    #[test]
    fn test_rotor_180_normalized() {
        let r = Rotor::from_axis_angle([0.0, 0.0, 1.0], std::f64::consts::PI);
        // After 180 degrees, scalar part should be cos(pi/2) ≈ 0
        assert!(r.inner.scalar_part().abs() < 0.1);
    }

    #[test]
    fn test_compose_identity() {
        let r1 = Rotor::identity();
        let r2 = Rotor::identity();
        let r3 = r1.compose(&r2);
        assert!(r3.is_identity(0.1));
    }

    #[test]
    fn test_rotor_bivector_components() {
        let r = Rotor::from_axis_angle([1.0, 0.0, 0.0], 1.0);
        // Rotation around x should produce e23 bivector component
        assert!(r.inner.c[10].abs() > 0.1);
    }

    #[test]
    fn test_slerp_endpoints() {
        let r1 = Rotor::identity();
        let r2 = Rotor::from_axis_angle([0.0, 0.0, 1.0], 1.0);
        let at0 = r1.slerp(&r2, 0.0);
        assert!(at0.is_identity(0.1));
    }

    #[test]
    fn test_rotation_matrix_identity() {
        let r = Rotor::identity();
        let mat = r.to_rotation_matrix();
        assert!((mat[0][0] - 1.0).abs() < 0.1);
        assert!((mat[1][1] - 1.0).abs() < 0.1);
    }
}
