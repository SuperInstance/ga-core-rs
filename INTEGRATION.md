# INTEGRATION.md — ga-core-rs × conservation-law-rs × spectral-fleet-rs

**Geometric algebra core** provides conformal geometric algebra primitives
for rotations, reflections, and distances. It connects to Lagrangian
mechanics for rotor-based symmetry operations and to spectral methods for
rotating eigenvector bases.

## Synergy Map

```
conservation-law-rs              ga-core-rs                   spectral-fleet-rs
┌──────────────────┐            ┌──────────────────────┐     ┌─────────────────┐
│ AgentState        │◄──────────►│ Rotor                │◄────►│ l2_norm         │
│ verify_noether    │            │ Multivector          │     │ normalize       │
│ RotationSymmetry  │            │ Conformal            │     │ dot             │
│ total_energy      │            │ geometric_product    │     │ PowerIteration  │
└──────────────────┘            │ wedge                │     └─────────────────┘
                                │ inner                │
                                └──────────────────────┘
```

## Key Insight

Rotors in geometric algebra perform rotations via sandwich products,
exactly the same mathematical structure as Noether's theorem for rotation
symmetries. Spectral-fleet's eigenvectors define natural axes; rotors
rotate the fleet's coordinate system into alignment with those axes.

## Example 1: Rotor-Based Rotation Symmetry Verification

Use ga-core rotors to implement rotation symmetry for Noether verification.

```rust
use ga_core::rotor::Rotor;
use conservation_law::lagrangian::{AgentState, MechanicalLagrangian};
use conservation_law::noether::{RotationSymmetry, test_invariance};

fn verify_rotational_symmetry_with_rotor() {
    // Create a rotor for 45-degree rotation around Z axis
    let rotor = Rotor::from_axis_angle([0.0, 0.0, 1.0], std::f64::consts::FRAC_PI_4);

    // Apply to a 2D state embedded in 3D
    let state = AgentState::new([1.0, 0.5], [0.0, 0.0]);
    let rotated_q = rotor.apply([state.q[0], state.q[1], 0.0]);
    println!("Rotated position: [{:.4}, {:.4}]", rotated_q[0], rotated_q[1]);

    // Verify invariance of central potential
    let potential = |q: &[f64; 2]| {
        let r2 = q[0] * q[0] + q[1] * q[1];
        0.5 * r2
    };
    let lagrangian = MechanicalLagrangian { mass: 1.0, potential_fn: potential };
    let rot_sym = RotationSymmetry { i: 0, j: 1 };
    let inv = test_invariance(&lagrangian, &rot_sym, &state, 1e-3, 1e-10);
    println!("Rotation invariant: {} (ΔL = {:e})", inv.invariant, inv.delta_lagrangian);
}
```

## Example 2: Conformal Distance for Agent Affinity

Use conformal embedding to compute distances between agents, then build
a spectral affinity matrix.

```rust
use ga_core::conformal::Conformal;
use ga_core::multivector::Multivector;

fn conformal_affinity_matrix(positions: &[[f64; 3]]) -> Vec<Vec<f64>> {
    let n = positions.len();
    let mut affinity = vec![vec![0.0; n]; n];

    for i in 0..n {
        let p1 = Conformal::embed_point(positions[i]);
        for j in i..n {
            let p2 = Conformal::embed_point(positions[j]);
            let d = Conformal::conformal_distance(&p1, &p2);
            let a = (-d * d).exp();
            affinity[i][j] = a;
            affinity[j][i] = a;
        }
    }
    affinity
}
```

## Example 3: Rotate Eigenvectors with Rotors

After spectral decomposition, rotate the eigenvector basis to align with
a canonical coordinate system.

```rust
use ga_core::rotor::Rotor;
use spectral_fleet::power_iteration::Eigenpair;

fn align_eigenvector_with_axis(eigenpair: &Eigenpair<f64>, target_axis: [f64; 3]) -> Vec<f64> {
    let v = &eigenpair.vector;
    // Embed eigenvector in 3D
    let v3 = [v[0], v.get(1).copied().unwrap_or(0.0), 0.0];

    // Compute rotation from v3 to target_axis
    let dot_product = v3[0] * target_axis[0] + v3[1] * target_axis[1];
    let cross_z = v3[0] * target_axis[1] - v3[1] * target_axis[0];
    let angle = cross_z.atan2(dot_product);

    let rotor = Rotor::from_axis_angle([0.0, 0.0, 1.0], angle);
    let rotated = rotor.apply(v3);
    vec![rotated[0], rotated[1]]
}
```

## Cargo.toml Wiring

```toml
[dependencies]
ga-core = { git = "https://github.com/SuperInstance/ga-core-rs" }
conservation-law = { git = "https://github.com/SuperInstance/conservation-law-rs" }
spectral-fleet = { git = "https://github.com/SuperInstance/spectral-fleet-rs" }
```

## Design Patterns

### Pattern: Geometric Alignment of Agent Formations

Use conformal reflections to align an agent swarm with a target plane:

```rust
use ga_core::conformal::Conformal;
use ga_core::multivector::Multivector;

fn align_formation(points: &mut [[f64; 3]], plane_normal: [f64; 3], plane_dist: f64) {
    for p in points.iter_mut() {
        let reflected = Conformal::reflect(*p, plane_normal, plane_dist);
        *p = reflected;
    }
}
```

### Pattern: Conformal Barycenter Consensus

Agents vote on positions; the conformal barycenter is the fair meeting point:

```rust
use ga_core::conformal::Conformal;

fn fair_meetup(positions: &[[f64; 3]]) -> [f64; 3] {
    Conformal::barycenter(positions)
}
```
