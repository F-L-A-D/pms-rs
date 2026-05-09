use crate::projection::topology::
    projection_node::ProjectionNode;

use super::projection_scope::
    ProjectionScope;

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub struct InvalidationTraversalPlan {

    pub ordered_nodes:
        Vec<ProjectionNode>,

    pub scope:
        ProjectionScope,
}

impl InvalidationTraversalPlan {

    pub fn new(
        ordered_nodes: Vec<ProjectionNode>,
        scope: ProjectionScope,
    ) -> Self {

        Self {
            ordered_nodes,
            scope,
        }
    }
}