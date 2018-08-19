use ::na;
use base::types::NodeId;

/// Represent a Generic Factor
/// * `D` - Dimensions of the factor (2D, 3D, etc.)
pub trait FactorTrait<D: na::Dim> {
    /// Get the nodes that may be involved in the Factor
    fn get_node_ids(&self) -> Vec<NodeId>;
}


