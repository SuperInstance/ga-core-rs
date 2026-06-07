//! Agent physics: position, velocity, orientation and forces in conformal GA.
//!
//! Agent bodies live in conformal geometric algebra space so that collisions,
//! distances, and rotations are expressed as native GA operations (sphere–sphere
//! tests, rotor sandwich products, etc.).

use crate::multivector::Multivector;
use crate::rotor::Rotor;
use crate::conformal::Conformal;

/// A physical body for an agent, expressed in conformal GA.
///
/// Position and velocity are 3-D Euclidean vectors stored inside `Multivector`
/// (e1/e2/e3 components).  Orientation is a `Rotor`.
#[derive(Debug, Clone)]
pub struct AgentBody {
    /// Agent identifier.
    pub id: u64,
    /// Conformal-encoded position.
    pub position: Multivector,
    /// Velocity as a vector multivector.
    pub velocity: Multivector,
    /// Orientation rotor.
    pub orientation: Rotor,
    /// Effective radius for collision detection.
    pub radius: f64,
}

impl AgentBody {
    /// Create a new body at the origin with zero velocity.
    pub fn new(id: u64) -> Self {
        Self {
            id,
            position: Conformal::embed_point([0.0, 0.0, 0.0]),
            velocity: Multivector::zero(),
            orientation: Rotor::identity(),
            radius: 1.0,
        }
    }

    /// Create a body at a specific position with a given radius.
    pub fn at(id: u64, pos: [f64; 3], radius: f64) -> Self {
        Self {
            id,
            position: Conformal::embed_point(pos),
            velocity: Multivector::zero(),
            orientation: Rotor::identity(),
            radius,
        }
    }

    /// Create a body with position, velocity, and radius.
    pub fn with_velocity(id: u64, pos: [f64; 3], vel: [f64; 3], radius: f64) -> Self {
        let mut v = Multivector::zero();
        v.c[2] = vel[0];
        v.c[3] = vel[1];
        v.c[4] = vel[2];
        Self {
            id,
            position: Conformal::embed_point(pos),
            velocity: v,
            orientation: Rotor::identity(),
            radius,
        }
    }

    /// Euclidean 3-D position.
    pub fn pos(&self) -> [f64; 3] {
        Conformal::extract_point(&self.position)
    }

    /// Euclidean 3-D velocity.
    pub fn vel(&self) -> [f64; 3] {
        [self.velocity.c[2], self.velocity.c[3], self.velocity.c[4]]
    }

    /// Forward direction (e1 rotated by orientation).
    pub fn forward(&self) -> [f64; 3] {
        self.orientation.apply([1.0, 0.0, 0.0])
    }
}

// ---------------------------------------------------------------------------
// ForceField
// ---------------------------------------------------------------------------

/// A force field that applies forces (as bivector fields) to agent bodies.
#[derive(Debug, Clone)]
pub struct ForceField {
    /// Origin of the field.
    pub origin: [f64; 3],
    /// Strength of the field.
    pub strength: f64,
    /// Kind of field.
    pub kind: FieldKind,
}

/// What kind of force field to apply.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FieldKind {
    /// Constant directional force (like gravity).
    Constant([f64; 3]),
    /// Radial attraction towards the origin.
    Attraction,
    /// Radial repulsion from the origin.
    Repulsion,
}

impl ForceField {
    /// Create a constant directional force field.
    pub fn constant(force: [f64; 3]) -> Self {
        Self {
            origin: [0.0; 3],
            strength: 1.0,
            kind: FieldKind::Constant(force),
        }
    }

    /// Create an attraction field at `origin` with given `strength`.
    pub fn attraction(origin: [f64; 3], strength: f64) -> Self {
        Self {
            origin,
            strength,
            kind: FieldKind::Attraction,
        }
    }

    /// Create a repulsion field at `origin` with given `strength`.
    pub fn repulsion(origin: [f64; 3], strength: f64) -> Self {
        Self {
            origin,
            strength,
            kind: FieldKind::Repulsion,
        }
    }

    /// Evaluate the force vector at a given position.
    pub fn force_at(&self, pos: [f64; 3]) -> [f64; 3] {
        match self.kind {
            FieldKind::Constant(f) => [f[0] * self.strength, f[1] * self.strength, f[2] * self.strength],
            FieldKind::Attraction => {
                let dx = self.origin[0] - pos[0];
                let dy = self.origin[1] - pos[1];
                let dz = self.origin[2] - pos[2];
                let dist = (dx * dx + dy * dy + dz * dz).sqrt().max(0.01);
                let s = self.strength / dist;
                [dx / dist * s, dy / dist * s, dz / dist * s]
            }
            FieldKind::Repulsion => {
                let dx = pos[0] - self.origin[0];
                let dy = pos[1] - self.origin[1];
                let dz = pos[2] - self.origin[2];
                let dist = (dx * dx + dy * dy + dz * dz).sqrt().max(0.01);
                let s = self.strength / dist;
                [dx / dist * s, dy / dist * s, dz / dist * s]
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Free functions
// ---------------------------------------------------------------------------

/// Apply a force (given as a `Multivector` vector) to an `AgentBody` for time
/// step `dt`, updating velocity and position.
pub fn apply_force(body: &mut AgentBody, force: &Multivector, dt: f64) {
    // F = ma, assume unit mass → a = F
    // v += a * dt
    body.velocity = body.velocity.add(&force.scale(dt));
    // x += v * dt
    let displacement = body.velocity.scale(dt);
    let pos_mv = body.position.add(&displacement);
    body.position = pos_mv;
}

/// Apply a `ForceField` to a body for time step `dt`.
pub fn apply_field(body: &mut AgentBody, field: &ForceField, dt: f64) {
    let pos = body.pos();
    let f = field.force_at(pos);
    let mut force = Multivector::zero();
    force.c[2] = f[0];
    force.c[3] = f[1];
    force.c[4] = f[2];
    apply_force(body, &force, dt);
}

/// Check whether two bodies collide (sphere–sphere test using conformal GA
/// sphere representations).
pub fn check_collision(a: &AgentBody, b: &AgentBody) -> bool {
    distance(a, b) < (a.radius + b.radius)
}

/// Euclidean distance between two agent bodies.
pub fn distance(a: &AgentBody, b: &AgentBody) -> f64 {
    let pa = a.pos();
    let pb = b.pos();
    let dx = pa[0] - pb[0];
    let dy = pa[1] - pb[1];
    let dz = pa[2] - pb[2];
    (dx * dx + dy * dy + dz * dz).sqrt()
}

/// Check collision with an explicit radius override (for variable-size tests).
pub fn check_collision_with_radius(a: &AgentBody, b: &AgentBody, radius_sum: f64) -> bool {
    distance(a, b) < radius_sum
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_body_new() {
        let b = AgentBody::new(0);
        assert_eq!(b.id, 0);
        let p = b.pos();
        assert!((p[0]).abs() < 1e-10);
        assert!((p[1]).abs() < 1e-10);
        assert!((p[2]).abs() < 1e-10);
    }

    #[test]
    fn test_agent_body_at() {
        let b = AgentBody::at(1, [3.0, 4.0, 5.0], 2.0);
        let p = b.pos();
        assert!((p[0] - 3.0).abs() < 1e-10);
        assert!((p[1] - 4.0).abs() < 1e-10);
        assert!((p[2] - 5.0).abs() < 1e-10);
        assert!((b.radius - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_agent_body_with_velocity() {
        let b = AgentBody::with_velocity(2, [0.0; 3], [1.0, 2.0, 3.0], 1.0);
        let v = b.vel();
        assert!((v[0] - 1.0).abs() < 1e-10);
        assert!((v[1] - 2.0).abs() < 1e-10);
        assert!((v[2] - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_forward_direction_identity() {
        let b = AgentBody::new(0);
        let fwd = b.forward();
        assert!((fwd[0] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_apply_force() {
        let mut b = AgentBody::new(0);
        let mut f = Multivector::zero();
        f.c[2] = 10.0; // 10 m/s² along x
        apply_force(&mut b, &f, 1.0);
        let v = b.vel();
        assert!((v[0] - 10.0).abs() < 1e-10);
        let p = b.pos();
        assert!(p[0] > 0.0); // moved
    }

    #[test]
    fn test_apply_force_two_steps() {
        let mut b = AgentBody::new(0);
        let mut f = Multivector::zero();
        f.c[3] = 5.0; // 5 m/s² along y
        apply_force(&mut b, &f, 1.0);
        apply_force(&mut b, &f, 1.0);
        let v = b.vel();
        assert!((v[1] - 10.0).abs() < 1e-10);
    }

    #[test]
    fn test_distance() {
        let a = AgentBody::at(0, [0.0, 0.0, 0.0], 1.0);
        let b = AgentBody::at(1, [3.0, 4.0, 0.0], 1.0);
        assert!((distance(&a, &b) - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_check_collision_true() {
        let a = AgentBody::at(0, [0.0, 0.0, 0.0], 2.0);
        let b = AgentBody::at(1, [3.0, 0.0, 0.0], 2.0);
        assert!(check_collision(&a, &b)); // dist=3, radii sum=4
    }

    #[test]
    fn test_check_collision_false() {
        let a = AgentBody::at(0, [0.0, 0.0, 0.0], 1.0);
        let b = AgentBody::at(1, [5.0, 0.0, 0.0], 1.0);
        assert!(!check_collision(&a, &b)); // dist=5, radii sum=2
    }

    #[test]
    fn test_check_collision_with_radius() {
        let a = AgentBody::at(0, [0.0, 0.0, 0.0], 1.0);
        let b = AgentBody::at(1, [4.0, 0.0, 0.0], 1.0);
        assert!(!check_collision_with_radius(&a, &b, 3.0)); // dist=4 > 3
        assert!(check_collision_with_radius(&a, &b, 5.0));  // dist=4 < 5
    }

    #[test]
    fn test_constant_force_field() {
        let field = ForceField::constant([0.0, -9.8, 0.0]);
        let f = field.force_at([10.0, 20.0, 30.0]);
        assert!((f[1] - (-9.8)).abs() < 1e-10);
    }

    #[test]
    fn test_attraction_field() {
        let field = ForceField::attraction([0.0, 0.0, 0.0], 10.0);
        let f = field.force_at([1.0, 0.0, 0.0]);
        // Should point towards origin from x=1 → negative x direction
        assert!(f[0] < 0.0);
    }

    #[test]
    fn test_repulsion_field() {
        let field = ForceField::repulsion([0.0, 0.0, 0.0], 10.0);
        let f = field.force_at([1.0, 0.0, 0.0]);
        // Should push away from origin → positive x direction
        assert!(f[0] > 0.0);
    }

    #[test]
    fn test_apply_field_moves_body() {
        let mut b = AgentBody::new(0);
        let field = ForceField::constant([1.0, 0.0, 0.0]);
        apply_field(&mut b, &field, 1.0);
        let p = b.pos();
        assert!(p[0] > 0.0);
    }
}
