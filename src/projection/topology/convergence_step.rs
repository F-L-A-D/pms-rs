use crate::projection::
    topology::projection_node::
        ProjectionNode;

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub struct ConvergenceStep {
    node: ProjectionNode,
}

impl ConvergenceStep {
    
    pub fn new(
        node: ProjectionNode,
    ) -> Self {
        Self {
            node,
        }
    }

    pub fn node(&self) 
        -> ProjectionNode 
    {
        self.node
    }
}