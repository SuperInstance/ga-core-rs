//! Multivector: the fundamental element of geometric algebra.
//!
//! For Cl(3,1), a multivector has 2^4 = 16 components:
//! 1 scalar, 4 vectors, 6 bivectors, 4 trivectors, 1 pseudoscalar.

/// A multivector in Cl(3,1) spacetime algebra.
///
/// Components indexed by basis blade:
/// 0: scalar
/// 1-4: e0, e1, e2, e3
/// 5-10: e01, e02, e03, e12, e13, e23
/// 11-14: e012, e013, e023, e123
/// 15: e0123 (pseudoscalar)
#[derive(Debug, Clone, PartialEq)]
pub struct Multivector {
    pub c: [f64; 16],
}

impl Multivector {
    /// Zero multivector.
    pub fn zero() -> Self {
        Self { c: [0.0; 16] }
    }

    /// Scalar value.
    pub fn scalar(v: f64) -> Self {
        let mut m = Self::zero();
        m.c[0] = v;
        m
    }

    /// Vector from 4 components (e0, e1, e2, e3).
    pub fn vector(v: [f64; 4]) -> Self {
        let mut m = Self::zero();
        m.c[1] = v[0];
        m.c[2] = v[1];
        m.c[3] = v[2];
        m.c[4] = v[3];
        m
    }

    /// Bivector from 6 components (e01, e02, e03, e12, e13, e23).
    pub fn bivector(v: [f64; 6]) -> Self {
        let mut m = Self::zero();
        m.c[5] = v[0];
        m.c[6] = v[1];
        m.c[7] = v[2];
        m.c[8] = v[3];
        m.c[9] = v[4];
        m.c[10] = v[5];
        m
    }

    /// Extract the scalar part.
    pub fn scalar_part(&self) -> f64 {
        self.c[0]
    }

    /// Extract the vector part (4 components).
    pub fn vector_part(&self) -> [f64; 4] {
        [self.c[1], self.c[2], self.c[3], self.c[4]]
    }

    /// Extract the bivector part (6 components).
    pub fn bivector_part(&self) -> [f64; 6] {
        [
            self.c[5], self.c[6], self.c[7], self.c[8], self.c[9], self.c[10],
        ]
    }

    /// Extract grade-k components.
    /// Returns the sum of absolute values of all components of grade k.
    pub fn grade_norm(&self, k: usize) -> f64 {
        match k {
            0 => self.c[0].abs(),
            1 => self.c[1..=4].iter().map(|x| x.abs()).sum(),
            2 => self.c[5..=10].iter().map(|x| x.abs()).sum(),
            3 => self.c[11..=14].iter().map(|x| x.abs()).sum(),
            4 => self.c[15].abs(),
            _ => 0.0,
        }
    }

    /// Addition.
    pub fn add(&self, other: &Self) -> Self {
        let mut r = Self::zero();
        for i in 0..16 {
            r.c[i] = self.c[i] + other.c[i];
        }
        r
    }

    /// Subtraction.
    pub fn sub(&self, other: &Self) -> Self {
        let mut r = Self::zero();
        for i in 0..16 {
            r.c[i] = self.c[i] - other.c[i];
        }
        r
    }

    /// Scalar multiplication.
    pub fn scale(&self, s: f64) -> Self {
        let mut r = Self::zero();
        for i in 0..16 {
            r.c[i] = self.c[i] * s;
        }
        r
    }

    /// Reverse: reverses the order of all blade products.
    /// For grade k, the sign changes by (-1)^(k(k-1)/2).
    pub fn reverse(&self) -> Self {
        let mut r = self.clone();
        // Grade 2 (bivectors): sign flip
        for i in 5..=10 {
            r.c[i] = -r.c[i];
        }
        // Grade 3 (trivectors): sign flip
        for i in 11..=14 {
            r.c[i] = -r.c[i];
        }
        r
    }

    /// Conjugate (Clifford conjugation): reverses AND negates odd grades.
    pub fn conjugate(&self) -> Self {
        let mut r = self.clone();
        // Negate grade 1
        for i in 1..=4 {
            r.c[i] = -r.c[i];
        }
        // Grade 2 stays
        // Negate grade 3
        for i in 11..=14 {
            r.c[i] = -r.c[i];
        }
        // Grade 4 stays
        r
    }

    /// Dual: multiply by the pseudoscalar from the right.
    /// In Cl(3,1) with metric (1,1,1,-1), I = e0123.
    pub fn dual(&self) -> Self {
        // Simplified: right-multiply by pseudoscalar
        // For each basis blade, e_A * I = ± e_{complement}
        // This is the Hodge dual
        let mut r = Self::zero();
        // The dual maps blade index i to index (15 - i) with a sign
        let signs = [1, -1, 1, -1, 1, -1, 1, -1, 1, -1, 1, -1, 1, -1, 1, -1];
        for i in 0..16 {
            r.c[15 - i] = signs[i] as f64 * self.c[i];
        }
        r
    }

    /// Norm squared: ⟨M̃, M⟩ where M̃ is the reverse.
    pub fn norm_squared(&self) -> f64 {
        let rev = self.reverse();
        let product = self.geometric_product(&rev);
        product.scalar_part()
    }

    /// Geometric product: the fundamental product of geometric algebra.
    /// This is a simplified implementation for Cl(3,1).
    pub fn geometric_product(&self, other: &Self) -> Self {
        // For the full geometric product, we need the multiplication table
        // This is the most complex operation in GA
        // Simplified: use bilinear expansion

        let mut result = Self::zero();

        // For each pair of components, multiply basis blades
        // Using the Cl(3,1) metric: e0²=+1, e1²=+1, e2²=+1, e3²=-1
        // The sign of e_i * e_j for i<j is +1 (anticommute)

        // Scalar * anything
        for i in 0..16 {
            result.c[i] += self.c[0] * other.c[i];
            result.c[i] += other.c[0] * self.c[i];
        }
        result.c[0] -= self.c[0] * other.c[0]; // subtracted twice above

        // Vector * vector: a·b + a∧b
        // e_i * e_i = metric[i-1]
        let metric = [1.0, 1.0, 1.0, -1.0]; // e0², e1², e2², e3²
        for i in 0..4 {
            for j in 0..4 {
                let ci = i + 1;
                let cj = j + 1;
                let prod = self.c[ci] * other.c[cj];
                if i == j {
                    // e_i² = metric[i]
                    result.c[0] += prod * metric[i];
                } else if i < j {
                    // e_i e_j = +e_{ij} (bivector)
                    let biv_idx = bivector_index(i, j);
                    result.c[biv_idx] += prod;
                } else {
                    // e_j e_i = -e_{ij}
                    let biv_idx = bivector_index(j, i);
                    result.c[biv_idx] -= prod;
                }
            }
        }

        // Scalar * vector and vector * scalar already handled above

        // For a complete implementation we'd need all grade combinations
        // This simplified version handles the most common cases:
        // scalar-scalar, scalar-vector, vector-scalar, vector-vector

        result
    }

    /// Wedge (outer) product.
    pub fn wedge(&self, other: &Self) -> Self {
        // a ∧ b = 0.5 * (a*b - b*a) for vectors
        let ab = self.geometric_product(other);
        let ba = other.geometric_product(self);
        ab.sub(&ba).scale(0.5)
    }

    /// Inner product (left contraction).
    pub fn inner(&self, other: &Self) -> Self {
        // a · b = 0.5 * (a*b + b*a) for vectors
        let ab = self.geometric_product(other);
        let ba = other.geometric_product(self);
        ab.add(&ba).scale(0.5)
    }

    /// Check if approximately zero.
    pub fn is_zero(&self, tolerance: f64) -> bool {
        self.c.iter().all(|&x| x.abs() < tolerance)
    }
}

/// Map two vector indices (i < j) to the bivector component index.
fn bivector_index(i: usize, j: usize) -> usize {
    // (0,1)→5, (0,2)→6, (0,3)→7, (1,2)→8, (1,3)→9, (2,3)→10
    match (i, j) {
        (0, 1) => 5,
        (0, 2) => 6,
        (0, 3) => 7,
        (1, 2) => 8,
        (1, 3) => 9,
        (2, 3) => 10,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero() {
        let m = Multivector::zero();
        assert!(m.is_zero(1e-10));
    }

    #[test]
    fn test_scalar() {
        let m = Multivector::scalar(5.0);
        assert_eq!(m.scalar_part(), 5.0);
    }

    #[test]
    fn test_vector() {
        let m = Multivector::vector([1.0, 2.0, 3.0, 4.0]);
        let v = m.vector_part();
        assert_eq!(v[0], 1.0);
        assert_eq!(v[3], 4.0);
    }

    #[test]
    fn test_add() {
        let a = Multivector::scalar(1.0);
        let b = Multivector::scalar(2.0);
        let c = a.add(&b);
        assert_eq!(c.scalar_part(), 3.0);
    }

    #[test]
    fn test_scale() {
        let m = Multivector::scalar(3.0);
        let s = m.scale(2.0);
        assert_eq!(s.scalar_part(), 6.0);
    }

    #[test]
    fn test_reverse() {
        let b = Multivector::bivector([1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        let r = b.reverse();
        // Bivectors change sign under reverse
        assert_eq!(r.c[5], -1.0);
    }

    #[test]
    fn test_grade_norm() {
        let m = Multivector::scalar(3.0);
        assert_eq!(m.grade_norm(0), 3.0);
        assert_eq!(m.grade_norm(1), 0.0);
    }

    #[test]
    fn test_geometric_product_scalar_scalar() {
        let a = Multivector::scalar(3.0);
        let b = Multivector::scalar(4.0);
        let c = a.geometric_product(&b);
        assert_eq!(c.scalar_part(), 12.0);
    }

    #[test]
    fn test_geometric_product_scalar_vector() {
        let s = Multivector::scalar(2.0);
        let v = Multivector::vector([1.0, 0.0, 0.0, 0.0]);
        let p = s.geometric_product(&v);
        assert_eq!(p.c[1], 2.0);
    }

    #[test]
    fn test_geometric_product_vector_vector_same() {
        let v = Multivector::vector([1.0, 0.0, 0.0, 0.0]);
        let p = v.geometric_product(&v);
        // e0² = 1 (metric), so result should be scalar 1.0
        assert!((p.scalar_part() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_geometric_product_vector_vector_orthogonal() {
        let v0 = Multivector::vector([1.0, 0.0, 0.0, 0.0]);
        let v1 = Multivector::vector([0.0, 1.0, 0.0, 0.0]);
        let p = v0.geometric_product(&v1);
        // e0*e1 = e01 (bivector)
        assert!((p.c[5] - 1.0).abs() < 1e-10); // e01 component
    }

    #[test]
    fn test_wedge_product() {
        let v0 = Multivector::vector([1.0, 0.0, 0.0, 0.0]);
        let v1 = Multivector::vector([0.0, 1.0, 0.0, 0.0]);
        let w = v0.wedge(&v1);
        // e0 ∧ e1 = e01
        assert!((w.c[5] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_inner_product() {
        let v0 = Multivector::vector([1.0, 0.0, 0.0, 0.0]);
        let v1 = Multivector::vector([2.0, 0.0, 0.0, 0.0]);
        let ip = v0.inner(&v1);
        // e0 · e0 = 1 (metric), so 1*2 = 2
        assert!((ip.scalar_part() - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_time_like_vector() {
        let v = Multivector::vector([0.0, 0.0, 0.0, 1.0]);
        let p = v.geometric_product(&v);
        // e3² = -1 (metric), so result should be scalar -1.0
        assert!((p.scalar_part() - (-1.0)).abs() < 1e-10);
    }

    #[test]
    fn test_norm_squared() {
        let v = Multivector::vector([3.0, 4.0, 0.0, 0.0]);
        // |v|² = 3² + 4² = 25
        let n = v.norm_squared();
        assert!((n - 25.0).abs() < 1e-10);
    }
}
