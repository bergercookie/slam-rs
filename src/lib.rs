#![feature(slice_patterns)]

extern crate generic_array;
extern crate kiss3d;
extern crate nalgebra as na;
extern crate typenum;
extern crate itertools;
extern crate rand; // TODO - take it out if not needed

/// Root library file for slam-rs
pub mod base;
pub mod factors;
pub mod geometry;
pub mod graphs;
pub mod solvers;


pub use self::base::*;
pub use self::factors::*;
pub use self::geometry::*;
pub use self::graphs::*;
pub use self::solvers::*;
