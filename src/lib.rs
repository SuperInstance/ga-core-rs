#![allow(
    clippy::needless_range_loop,
    clippy::new_without_default,
    clippy::type_complexity,
    dead_code
)]
//! # GA Core
//!
//! Conformal geometric algebra using Cl(3,1) spacetime algebra.
//!
//! # Key Concepts
//! - **Multivectors**: Elements of the geometric algebra with scalar, vector, bivector,
//!   trivector, and pseudoscalar parts
//! - **Geometric product**: The fundamental product combining inner and outer products
//! - **Rotors**: Even-grade elements that perform rotations via sandwich products
//! - **Conformal model**: Embeds Euclidean 3D into 5D conformal space for unified
//!   treatment of points, lines, planes, circles, and spheres

mod conformal;
mod multivector;
mod rotor;

pub use conformal::Conformal;
pub use multivector::Multivector;
pub use rotor::Rotor;
