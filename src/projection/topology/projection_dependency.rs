use super::projection_node::
    ProjectionNode;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
)]
pub struct ProjectionDependency {
    pub upstream: ProjectionNode,
    pub downstream: ProjectionNode,
}

impl ProjectionDependency {

    pub fn new(
        upstream: ProjectionNode,
        downstream: ProjectionNode,
    ) -> Self {

        Self {
            upstream,
            downstream,
        }
    }
}