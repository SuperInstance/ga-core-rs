# ga-core

> Conformal geometric algebra in Cl(3,1) — multivectors, rotors, conformal embeddings, and sandwich products in pure Rust.

## What This Does

This crate implements the 16-component geometric algebra of Cl(3,1) spacetime. It gives you `Multivector` with scalar, vector, bivector, trivector, and pseudoscalar parts; the full geometric product combining inner and outer products; `Rotor` for axis-angle rotations via sandwich products; and `Conformal` for embedding Euclidean 3D points into a higher-dimensional space where translations, rotations, and reflections are unified as versor operations. Zero dependencies. No linear algebra crates required.

## Why It Matters

Neural networks learn in vector spaces because that is what our tools provide. But the physical world is not a vector space — it is a geometric algebra, where rotations, reflections, and projections are all instances of a single product. Geometric algebra is the native language of spatial reasoning. An AGI that manipulates concepts in GA rather than matrices will have fewer ad-hoc conventions, more composable transformations, and a direct path from symbolic reasoning to physical action.

## Quick Start

```bash
cargo add ga-core
```

```rust
use ga_core::{Multivector, Rotor, Conformal};

fn main() {
    // Rotate a point 90° around the Z axis
    let rotor = Rotor::from_axis_angle([0.0, 0.0, 1.0], std::f64::consts::FRAC_PI_2);
    let rotated = rotor.apply([1.0, 0.0, 0.0]);
    println!("Rotated: [{:.2}, {:.2}, {:.2}]", rotated[0], rotated[1], rotated[2]);

    // Conformal embedding: Euclidean point → conformal null vector
    let embedded = Conformal::embed_point([3.0, 4.0, 0.0]);
    println!("Embedded norm²: {:.2}", embedded.scalar_part());

    // Reflect through the XY plane
    let reflected = Conformal::reflect([1.0, 2.0, 3.0], [0.0, 0.0, 1.0], 0.0);
    println!("Reflected z: {:.2}", reflected[2]);
}
```

## Architecture

| Module | Purpose |
|--------|---------|
| `multivector` | 16-component multivectors in Cl(3,1), geometric/inner/wedge products, grade extraction |
| `rotor` | Even-grade rotors for rotations, composition, SLERP, and rotation matrix export |
| `conformal` | Embed/extract Euclidean points, reflections, projections, planes, and barycenters |

## API Tour

### `Multivector`

The fundamental element of geometric algebra. 16 `f64` components indexed by basis blade grade.

```rust
impl Multivector {
    pub fn zero() -> Self;
    pub fn scalar(v: f64) -> Self;
    pub fn vector(v: [f64; 4]) -> Self;      // e0, e1, e2, e3
    pub fn bivector(v: [f64; 6]) -> Self;    // e01, e02, e03, e12, e13, e23
    pub fn scalar_part(&self) -> f64;
    pub fn vector_part(&self) -> [f64; 4];
    pub fn bivector_part(&self) -> [f64; 6];
    pub fn grade_norm(&self, k: usize) -> f64;
    pub fn geometric_product(&self, other: &Self) -> Self;
    pub fn wedge(&self, other: &Self) -> Self;
    pub fn inner(&self, other: &Self) -> Self;
    pub fn reverse(&self) -> Self;
    pub fn dual(&self) -> Self;
    pub fn norm_squared(&self) -> f64;
    pub fn is_zero(&self, tolerance: f64) -> bool;
}
```

### `Rotor`

Rotations via sandwich product: `v' = R v R̃`.

```rust
impl Rotor {
    pub fn identity() -> Self;
    pub fn from_axis_angle(axis: [f64; 3], angle: f64) -> Self;
    pub fn from_reflection(normal: [f64; 3]) -> Self;
    pub fn compose(&self, other: &Self) -> Self;
    pub fn apply(&self, v: [f64; 3]) -> [f64; 3];
    pub fn slerp(&self, other: &Self, t: f64) -> Self;
    pub fn to_rotation_matrix(&self) -> [[f64; 3]; 3];
    pub fn normalize(&mut self);
}
```

### `Conformal`

Static methods for conformal geometric algebra operations.

```rust
impl Conformal {
    pub fn embed_point(p: [f64; 3]) -> Multivector;     // P = e₊ + p + ½|p|²e₋
    pub fn extract_point(m: &Multivector) -> [f64; 3];
    pub fn reflect(point: [f64; 3], normal: [f64; 3], distance: f64) -> [f64; 3];
    pub fn rotate(point: [f64; 3], rotor: &Rotor) -> [f64; 3];
    pub fn project_onto_plane(point: [f64; 3], normal: [f64; 3], distance: f64) -> [f64; 3];
    pub fn plane(normal: [f64; 3], distance: f64) -> Multivector;
    pub fn midpoint(a: [f64; 3], b: [f64; 3]) -> [f64; 3];
    pub fn barycenter(points: &[[f64; 3]], weights: &[f64]) -> [f64; 3];
}
```

## Performance

| Operation | Complexity | Notes |
|-----------|-----------|-------|
| Geometric product (simplified) | O(1) | 16-component bilinear expansion |
| Rotor apply | O(1) | Two geometric products + extraction |
| Rotor compose | O(1) | One geometric product + normalization |
| Conformal embed/extract | O(1) | Direct array mapping |
| SLERP | O(1) | Linear interpolation + renormalization |

The geometric product implementation is simplified for the most common grade combinations (scalar-scalar, scalar-vector, vector-vector). A full 16×16 multiplication table would handle all grades at a constant but higher cost.

## Ecosystem

- **[conservation-law](https://github.com/SuperInstance/conservation-law-rs)** — Use rotors as rotation symmetries in Noether's theorem verification
- **[categorical-agents](https://github.com/SuperInstance/categorical-agents-rs)** — Model GA operations as morphisms in a composable adjunction
- **[spectral-fleet](https://github.com/SuperInstance/spectral-fleet-rs)** — Embed fleet positions in conformal space before spectral clustering
- **[wasserstein-agents](https://github.com/SuperInstance/wasserstein-agents-rs)** — Transport agent distributions while preserving conformal distances

## Ideas for Improvement

1. **Full geometric product table** — Complete the 16×16 basis blade multiplication table for all grade combinations.
2. **SIMD acceleration** — Use `f64x2` or `f64x4` to accelerate multivector operations on ARM and x86.
3. **Conformal circles and spheres** — Direct construction of rounds and flats in conformal space.
4. **Motor (dual quaternion) generalization** — Extend rotors to motors for screw transformations (rotation + translation).
5. **Inverse kinematics solver** — Use conformal geometric algebra for closed-form 3-joint IK, a classic GA strength.

## License

MIT OR Apache-2.0
