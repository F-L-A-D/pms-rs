use super::projection_node::
    ProjectionNode;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
)]
pub enum InvalidationPolicy {

    Always,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
)]
pub struct ProjectionDependency {

    pub upstream:
        ProjectionNode,

    pub downstream:
        ProjectionNode,

    pub invalidation_policy:
        InvalidationPolicy,
}

impl ProjectionDependency {

    pub fn new(
        upstream: ProjectionNode,
        downstream: ProjectionNode,
    ) -> Self {

        Self {
            upstream,
            downstream,

            invalidation_policy:
                InvalidationPolicy::Always,
        }
    }
}