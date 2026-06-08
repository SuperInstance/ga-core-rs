# ga-core-rs

Geometric algebra using Cl(3,1) spacetime algebra.

Multivectors, rotors, conformal embedding — a unified mathematical framework where vectors, planes, spheres, and rotations are all first-class citizens that compose via a single operation: the geometric product.

For agent systems, geometric algebra provides the natural language for spatial reasoning. Agents have position, velocity, orientation, and collision geometry — all expressible as multivector operations. No quaternions. No separate rotation matrices. One algebra.

Part of the **sunset-ecosystem**: agent bodies in conformal GA feed into `conservation-law` (energy conservation during collisions), and fleet coordination via `si-fleet-api` uses rotors for orientation-aware dispatch.

## The Math

### Geometric Algebra Cl(3,1)

The geometric algebra over $\mathbb{R}^{3,1}$ has basis vectors $e_0, e_1, e_2, e_3$ with metric:

$$e_0^2 = e_1^2 = e_2^2 = +1, \quad e_3^2 = -1$$

The algebra has $2^4 = 16$ basis elements:
- 1 scalar: $1$
- 4 vectors: $e_0, e_1, e_2, e_3$
- 6 bivectors: $e_{01}, e_{02}, e_{03}, e_{12}, e_{13}, e_{23}$
- 4 trivectors: $e_{012}, e_{013}, e_{023}, e_{123}$
- 1 pseudoscalar: $e_{0123}$

### The Geometric Product

The fundamental operation, combining the inner and outer products:

$$ab = a \cdot b + a \wedge b$$

For vectors, $a \cdot b$ gives a scalar and $a \wedge b$ gives a bivector. The geometric product unifies them.

### Rotors

A rotor $R$ is an even-grade element satisfying $R\tilde{R} = 1$. Rotations are applied via the sandwich product:

$$v' = Rv\tilde{R}$$

where $\tilde{R}$ is the reverse. From axis-angle:

$$R = \cos\frac{\theta}{2} - \sin\frac{\theta}{2} B$$

where $B$ is the rotation plane bivector.

### Conformal Model

Euclidean 3D points are embedded in conformal space:

$$P = e_+ + \mathbf{p} + \frac{1}{2}|\mathbf{p}|^2 e_-$$

In conformal GA, the inner product of two points encodes their distance:

$$P_1 \cdot P_2 = -\frac{1}{2}d^2$$

### Agent Physics

Agent bodies carry position (conformal multivector), velocity (vector multivector), and orientation (rotor). Collisions are sphere-sphere tests, movement is rotor-sandwiched velocity integration.

## Installation

```toml
[dependencies]
ga-core-rs = { git = "https://github.com/SuperInstance/ga-core-rs" }
```

## Usage

### Multivector Arithmetic

```rust
use ga_core::Multivector;

// Scalar
let s = Multivector::scalar(5.0);
println!("Scalar: {}", s.scalar_part());

// Vector in 4D: e0=1, e1=2, e2=3, e3=4
let v = Multivector::vector([1.0, 2.0, 3.0, 4.0]);
println!("Vector part: {:?}", v.vector_part());

// Bivector: e01=1, e02=2, e03=3, e12=4, e13=5, e23=6
let b = Multivector::bivector([1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
println!("Bivector part: {:?}", b.bivector_part());

// Addition
let sum = s.add(&v);
println!("Scalar + Vector: grade(0)={}, grade(1)={:?}",
    sum.scalar_part(), sum.vector_part());

// Scalar multiplication
let scaled = v.scale(2.0);
println!("2v: {:?}", scaled.vector_part());

// Geometric product: the fundamental operation
let v1 = Multivector::vector([1.0, 0.0, 0.0, 0.0]); // e0
let v2 = Multivector::vector([0.0, 1.0, 0.0, 0.0]); // e1
let gp = v1.geometric_product(&v2);
// e0 * e1 = e01 (bivector)
println!("e0 * e1: bivector part = {:?}", gp.bivector_part()); // [1, 0, 0, 0, 0, 0]
println!("e0 * e1: scalar = {}", gp.scalar_part());           // 0
```

### Inner and Outer Products

```rust
use ga_core::Multivector;

// Wedge (outer) product: a ∧ b gives the bivector
let e0 = Multivector::vector([1.0, 0.0, 0.0, 0.0]);
let e1 = Multivector::vector([0.0, 1.0, 0.0, 0.0]);
let wedge = e0.wedge(&e1);
println!("e0 ∧ e1 = bivector {:?}", wedge.bivector_part()); // [1, 0, 0, 0, 0, 0]

// Inner product: a · b gives the scalar
let a = Multivector::vector([1.0, 0.0, 0.0, 0.0]);
let b = Multivector::vector([2.0, 0.0, 0.0, 0.0]);
let inner = a.inner(&b);
println!("e0 · 2e0 = {}", inner.scalar_part()); // 2.0

// Orthogonal vectors: inner = 0, wedge = bivector
let c = Multivector::vector([0.0, 0.0, 1.0, 0.0]); // e2
let d = Multivector::vector([0.0, 0.0, 0.0, 1.0]); // e3 (timelike)
let ip = c.inner(&d);
println!("e2 · e3 = {}", ip.scalar_part()); // 0

// Timelike vector: v² = -1
let timelike = Multivector::vector([0.0, 0.0, 0.0, 1.0]);
let self_product = timelike.geometric_product(&timelike);
println!("e3² = {}", self_product.scalar_part()); // -1.0

// Norm squared
let v = Multivector::vector([3.0, 4.0, 0.0, 0.0]);
println!("|v|² = {}", v.norm_squared()); // 25.0
```

### Reverse, Conjugate, Dual

```rust
use ga_core::Multivector;

let b = Multivector::bivector([1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);

// Reverse: flips order of all products. Grade-k gets sign (-1)^(k(k-1)/2)
// Bivectors change sign
let rev = b.reverse();
println!("Bivector reverse: {:?}", rev.bivector_part()); // [-1, -2, -3, -4, -5, -6]

// Conjugate (Clifford conjugation): reverses AND negates odd grades
let conj = b.conjugate();
println!("Bivector conjugate: {:?}", conj.bivector_part()); // [1, 2, 3, 4, 5, 6] (grade 2 unchanged)

// Dual: multiply by pseudoscalar
let v = Multivector::vector([1.0, 0.0, 0.0, 0.0]);
let dual = v.dual();
println!("Dual of e0: {:?}", dual); // maps to complementary grade
```

### Rotors — Rotation via Sandwich Product

```rust
use ga_core::{Rotor, Multivector};

// Identity rotor: no rotation
let id = Rotor::identity();
let v = [1.0, 2.0, 3.0];
let rotated = id.apply(v);
println!("Identity rotation: {:?}", rotated); // [1.0, 2.0, 3.0]

// Rotation around z-axis by 90°
let rz = Rotor::from_axis_angle([0.0, 0.0, 1.0], std::f64::consts::FRAC_PI_2);
let x_hat = [1.0, 0.0, 0.0];
let y_hat = rz.apply(x_hat);
println!("Rotate x̂ 90° around ẑ: {:?}", y_hat); // ≈ [0, 1, 0]

// Rotation around x-axis by 180°
let rx = Rotor::from_axis_angle([1.0, 0.0, 0.0], std::f64::consts::PI);
let up = [0.0, 1.0, 0.0];
let flipped = rx.apply(up);
println!("Flip ŷ around x̂: {:?}", flipped); // ≈ [0, -1, 0]

// Compose rotations
let r1 = Rotor::from_axis_angle([0.0, 0.0, 1.0], std::f64::consts::FRAC_PI_4);
let r2 = Rotor::from_axis_angle([1.0, 0.0, 0.0], std::f64::consts::FRAC_PI_4);
let composed = r1.compose(&r2);
let test = composed.apply([1.0, 0.0, 0.0]);
println!("Composed rotation of x̂: {:?}", test);

// SLERP between rotors
let from = Rotor::identity();
let to = Rotor::from_axis_angle([0.0, 1.0, 0.0], std::f64::consts::PI);
let mid = from.slerp(&to, 0.5);
let mid_rotated = mid.apply([1.0, 0.0, 0.0]);
println!("SLERP at t=0.5: {:?}", mid_rotated);

// Extract rotation matrix
let r = Rotor::from_axis_angle([0.0, 0.0, 1.0], std::f64::consts::FRAC_PI_2);
let mat = r.to_rotation_matrix();
println!("Rotation matrix:");
for row in &mat {
    println!("  {:?}", row);
}
```

### Conformal Embedding — Points, Planes, Reflections

```rust
use ga_core::{Multivector, Rotor, Conformal};

// Embed a 3D point into conformal space
let p = [1.0, 2.0, 3.0];
let embedded = Conformal::embed_point(p);
println!("Embedded point: scalar={:.1}, vector={:?}",
    embedded.scalar_part(), embedded.vector_part());
// scalar = 0.5 * |p|² = 7.0, vector = [1, 1, 2, 3]

// Extract the Euclidean point back
let extracted = Conformal::extract_point(&embedded);
println!("Extracted: {:?}", extracted); // [1.0, 2.0, 3.0]

// Reflect a point through a plane
let point = [3.0, 4.0, 5.0];
let normal = [0.0, 0.0, 1.0]; // z-plane
let reflected = Conformal::reflect(point, normal, 0.0);
println!("Reflected through xy-plane: {:?}", reflected); // [3, 4, -5]

// Distance-preserving reflection
let d_before = (point[0].powi(2) + point[1].powi(2) + point[2].powi(2)).sqrt();
let d_after = (reflected[0].powi(2) + reflected[1].powi(2) + reflected[2].powi(2)).sqrt();
println!("Distance preserved: {}", (d_before - d_after).abs() < 1e-10);

// Create a plane as a multivector
let plane = Conformal::plane([0.0, 0.0, 1.0], 5.0);
println!("Plane: scalar={}, z component={}",
    plane.scalar_part(), plane.c[4]);

// Project a point onto a plane
let above = [1.0, 1.0, 10.0];
let projected = Conformal::project_onto_plane(above, [0.0, 0.0, 1.0], 0.0);
println!("Projected onto xy-plane: {:?}", projected); // [1, 1, 0]

// Midpoint
let a = [0.0, 0.0, 0.0];
let b = [2.0, 4.0, 6.0];
let mid = Conformal::midpoint(a, b);
println!("Midpoint: {:?}", mid); // [1, 2, 3]

// Barycenter of multiple points
let pts = [[0.0, 0.0, 0.0], [2.0, 0.0, 0.0], [0.0, 2.0, 0.0]];
let bc = Conformal::barycenter(&pts, &[1.0, 1.0, 1.0]);
println!("Triangle barycenter: {:?}", bc);
```

### Agent Physics

```rust
use ga_core::{AgentBody, Rotor, Conformal};

// Create agents in 3D space
let agent1 = AgentBody::at(1, [0.0, 0.0, 0.0], 1.0);
let agent2 = AgentBody::with_velocity(2, [5.0, 0.0, 0.0], [1.0, 0.0, 0.0], 0.5);

// Query agent state
println!("Agent 1 position: {:?}", agent1.pos());  // [0, 0, 0]
println!("Agent 2 velocity: {:?}", agent2.vel());  // [1, 0, 0]
println!("Agent 2 radius: {}", agent2.radius);     // 0.5

// Oriented agent
let mut agent3 = AgentBody::at(3, [2.0, 3.0, 1.0], 0.8);
agent3.orientation = Rotor::from_axis_angle([0.0, 1.0, 0.0], std::f64::consts::FRAC_PI_4);
let forward = agent3.forward();
println!("Agent 3 forward direction: {:?}", forward);

// Check distance between agents
let p1 = agent1.pos();
let p2 = agent2.pos();
let dx = p2[0] - p1[0];
let dist = (dx * dx).sqrt(); // simplified for one axis
println!("Distance between agents: {:.2}", dist);
```

### Grade Analysis

```rust
use ga_core::Multivector;

let v = Multivector::vector([1.0, 2.0, 3.0, 4.0]);
println!("Grade 0 (scalar): {}", v.grade_norm(0));  // 0.0
println!("Grade 1 (vector): {}", v.grade_norm(1));   // 10.0 (sum of abs)
println!("Grade 2 (bivector): {}", v.grade_norm(2)); // 0.0

let s = Multivector::scalar(7.0);
println!("Scalar grade 0: {}", s.grade_norm(0)); // 7.0

// Check if zero
let zero = Multivector::zero();
assert!(zero.is_zero(1e-10));
```

## API Reference

### Multivector

| Method | Description |
|--------|-------------|
| `zero()` | Zero multivector |
| `scalar(v)` | Scalar element |
| `vector([e0, e1, e2, e3])` | Vector element |
| `bivector([e01..e23])` | Bivector element |
| `scalar_part()` | Extract scalar component |
| `vector_part()` | Extract vector components |
| `bivector_part()` | Extract bivector components |
| `grade_norm(k)` | Sum of absolute grade-$k$ components |
| `add(other)` | Addition |
| `sub(other)` | Subtraction |
| `scale(s)` | Scalar multiplication |
| `geometric_product(other)` | Geometric product $ab$ |
| `wedge(other)` | Outer product $a \wedge b$ |
| `inner(other)` | Inner product $a \cdot b$ |
| `reverse()` | Reverse $\tilde{M}$ |
| `conjugate()` | Clifford conjugate |
| `dual()` | Hodge dual |
| `norm_squared()` | $\langle\tilde{M}, M\rangle$ |

### Rotor

| Method | Description |
|--------|-------------|
| `identity()` | No rotation |
| `from_axis_angle(axis, θ)` | Rotation by angle $\theta$ around axis |
| `from_reflection(normal)` | 180° rotation |
| `compose(other)` | Compose two rotors |
| `apply([x, y, z])` | Sandwich product $Rv\tilde{R}$ |
| `slerp(other, t)` | Spherical linear interpolation |
| `normalize()` | Enforce $R\tilde{R} = 1$ |
| `to_rotation_matrix()` | Extract 3×3 matrix |
| `is_identity(tol)` | Check approximate identity |

### Conformal

| Method | Description |
|--------|-------------|
| `embed_point([x, y, z])` | Euclidean → conformal |
| `extract_point(M)` | Conformal → Euclidean |
| `reflect(point, normal, d)` | Reflect through plane |
| `rotate(point, rotor)` | Apply rotor rotation |
| `plane(normal, distance)` | Plane as multivector |
| `project_onto_plane(point, n, d)` | Orthogonal projection |
| `midpoint(a, b)` | Midpoint |
| `barycenter(points, weights)` | Weighted barycenter |

### Agent Physics

| Type | Description |
|------|-------------|
| `AgentBody` | Position, velocity, orientation, radius |
| `ForceField` | Directional, radial, and vortex forces |

## Why This Matters for Agent Systems

1. **No gimbal lock**: Rotors compose smoothly in any order. Euler angles can't.
2. **Unified geometry**: Points, lines, planes, circles, and spheres are all multivectors. Distance, incidence, and angle are all inner products.
3. **Efficient composition**: Composing $n$ rotations is one geometric product chain — no matrix multiplication.
4. **Conformal distance**: In conformal GA, the inner product of two embedded points directly gives $-\frac{1}{2}d^2$. No explicit distance formula needed.
5. **Agent physics**: Position, velocity, and orientation are native GA types. Collisions are sphere-sphere tests in conformal space.

## Integration

### With `conservation-law`

```rust
// GA handles spatial math; conservation-law verifies that
// kinetic + potential energy is conserved during agent interactions
```

### With `si-fleet-api`

```rust
// Agent positions and orientations are transmitted as multivector coefficients
// Fleet coordinator uses conformal distance for proximity-based grouping
```

## License

MIT
