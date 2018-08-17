extern crate nalgebra as na;

use base::types::*;

/// A Pose Trait
pub trait PoseTrait {
    fn dims() -> u8;
    /// Does it have any uncertainty information
    fn has_uncertainty() -> bool {
        false
    }

    /// Way that we are encoding uncertainty
    ///
    /// Querying this makes sense only when has_uncertainty returns true
    fn uncertainty_type() -> UncertaintyType {
        UncertaintyType::Information
    }
}

/// Represent a 3D pose - no covariance information
pub struct Pose3D<T: na::Real> {
    pub pos: na::Vector3<T>,
    pub rot: na::UnitQuaternion<T>,
}
impl<T> PoseTrait for Pose3D<T>
where
    T: na::Real,
{
    fn dims() -> u8 {
        3
    }
}
