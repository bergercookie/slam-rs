use base::types::NodeId;
use geometry::poses::PoseTrait;

/// Represent a Generic Factor
pub trait FactorTrait {
    /// Dimensions of the Factor
    fn dims(&self) -> u8;
}

/// Actual Between Poses Factor
/// * `T` - Scalar Type
/// * `P` - Concrete Pose Type
pub struct BetweenPosesFactor<P> {
    _src: NodeId,
    _dst: NodeId,
    pose: P,
}

impl<P> BetweenPosesFactor<P> {
    fn src(&self) -> NodeId {
        self._src
    }
    fn dst(&self) -> NodeId {
        self._dst
    }

    fn new(src: NodeId, dst: NodeId, pose: P) -> BetweenPosesFactor<P> {
        BetweenPosesFactor {
            _src: src,
            _dst: dst,
            pose: P::new(),
        }
    }
}

impl<P: PoseTrait> FactorTrait for BetweenPosesFactor<P> {
    fn dims(&self) -> u8 {
        P::dims()
    }
}
