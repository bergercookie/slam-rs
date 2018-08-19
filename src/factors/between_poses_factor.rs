use base::types::NodeId;
use factors::factor_trait::FactorTrait;

use geometry::poses::{PoseN};
use ::generic_array;
use ::typenum;
use ::std;
use ::na;


/// Actual Between Poses Factor
/// * `T` - Scalar Type
/// * `P` - Concrete Pose Type
/// * `D` - Dimensions of the factor (2D, 3D, etc.)
#[derive(Debug)]
pub struct BetweenPosesFactor<T, D>
where T: na::Real,
      D: na::Dim + na::DimName,
      <D as na::DimName>::Value: std::ops::Mul<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>>,
      <D as na::DimName>::Value: std::ops::Mul,
      <<D as na::DimName>::Value as std::ops::Mul<typenum::UInt<typenum::UTerm, typenum::B1>>>::Output: generic_array::ArrayLength<T>,
      <<D as na::DimName>::Value as std::ops::Mul>::Output: generic_array::ArrayLength<T>, {
    src: NodeId,
    dst: NodeId,
    pose: PoseN<T, D>,
}

impl<T, D> BetweenPosesFactor<T, D>
where T: na::Real,
      D: na::Dim + na::DimName,
      <D as na::DimName>::Value: std::ops::Mul<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>>,
      <D as na::DimName>::Value: std::ops::Mul,
      <<D as na::DimName>::Value as std::ops::Mul<typenum::UInt<typenum::UTerm, typenum::B1>>>::Output: generic_array::ArrayLength<T>,
      <<D as na::DimName>::Value as std::ops::Mul>::Output: generic_array::ArrayLength<T>, {

    pub fn src(&self) -> NodeId {
        self.src
    }
    pub fn dst(&self) -> NodeId {
        self.dst
    }

    pub fn new(_src: NodeId, _dst: NodeId, _pose: PoseN<T, D>) -> BetweenPosesFactor<T, D> {
        BetweenPosesFactor {
            src: _src,
            dst: _dst,
            pose: _pose,
        }
    }
}

impl<T, D> FactorTrait<D> for BetweenPosesFactor<T, D>
where T: na::Real,
      D: na::Dim + na::DimName,
      <D as na::DimName>::Value: std::ops::Mul<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>>,
      <D as na::DimName>::Value: std::ops::Mul,
      <<D as na::DimName>::Value as std::ops::Mul<typenum::UInt<typenum::UTerm, typenum::B1>>>::Output: generic_array::ArrayLength<T>,
      <<D as na::DimName>::Value as std::ops::Mul>::Output: generic_array::ArrayLength<T>, {


    fn get_node_ids(&self) -> Vec<NodeId> {
        vec![self.src(), self.dst()]
    }
}
