use base::types::*;
use std::str;
use std::fmt;

use ::generic_array;
use ::typenum;
use ::std;
use ::na;

/// A Pose Trait
/// * `D` - Dimensions of the pose (2D, 3D, etc.)
pub trait PoseTrait<D>
where D: na::DimName,
{
    fn dims() -> Option<usize> {
        D::try_to_usize()
    }
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

    /// Create a Pose instance from a string.
    /// Implementations should use the part of the string they are interested in. The rest of it
    /// will be ignored
    fn from_string(s: String) -> Self;
}

/// Represent a 3D pose - no covariance information
#[derive(Debug)]
pub struct PoseN<T, D>
where T: na::Real,
      D: na::DimName,
      // for some weird reason this gibberish is needed and that's indicated when running cargo
      // build. Maybe I'm just using the wrong trait/type for D but there's got to  be a less
      // verbose way of doing it
      <D as na::DimName>::Value: std::ops::Mul<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>>,
      <D as na::DimName>::Value: std::ops::Mul,
      <<D as na::DimName>::Value as std::ops::Mul<typenum::UInt<typenum::UTerm, typenum::B1>>>::Output: generic_array::ArrayLength<T>,
      <<D as na::DimName>::Value as std::ops::Mul>::Output: generic_array::ArrayLength<T>, {
    pub trans: na::VectorN<T, D>,
    pub rot: na::geometry::Rotation<T, D>,
}

impl<T> PoseTrait<na::U3> for PoseN<T, na::U3>
where
    T: na::Real + str::FromStr,
    <T as str::FromStr>::Err: fmt::Debug,
{
    fn from_string(s: String) -> Self {
        // order: x, y, z, qx, qy, qz, qw
        let mut vals = s.split_whitespace();
        let get_number = |s: &str| -> T { s.parse::<T>().unwrap() };

        // translation
        let x = get_number(vals.next().unwrap());
        let y = get_number(vals.next().unwrap());
        let z = get_number(vals.next().unwrap());
        let trans_: na::Vector3<T> = na::Vector3::new(x, y, z);

        // rotation
        let qx = get_number(vals.next().unwrap());
        let qy = get_number(vals.next().unwrap());
        let qz = get_number(vals.next().unwrap());
        let qw = get_number(vals.next().unwrap());
        let quat_: na::UnitQuaternion<T> = na::UnitQuaternion::from_quaternion(na::Quaternion::new(qx, qy, qz, qw));

        PoseN::<T, na::U3>::new(trans_, quat_)
    }
}

impl<T> PoseN<T, na::U3>
where
    T: na::Real,
{
    pub fn new(trans_: na::Vector3<T>, quat_: na::UnitQuaternion<T>) -> PoseN<T, na::U3> {
        PoseN {
            trans: trans_,
            rot: quat_.to_rotation_matrix(),
        }
    }
}

pub type Pose2D = PoseN<f64, na::U2>;
pub type Pose3D = PoseN<f64, na::U3>;
