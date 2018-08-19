/// Way of encoding the uncertainty of a measurement - either in covariance or information form
pub enum UncertaintyType {
    Covariance,
    Information,
}

/// Node ID type
///
/// This can be anything that may comrpise a Node in a Graph e.g., a Landmark Id or a Pose Id
pub type NodeId = u64;
pub type PoseId = NodeId;
pub type LandmarkId = NodeId;
