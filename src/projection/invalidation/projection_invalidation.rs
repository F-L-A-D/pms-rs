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
pub struct ProjectionInvalidation {

    pub source:
        ProjectionNode,

    pub scope:
        ProjectionScope,
}

impl ProjectionInvalidation {

    pub fn new(
        source: ProjectionNode,
        scope: ProjectionScope,
    ) -> Self {

        Self {
            source,
            scope,
        }
    }
}