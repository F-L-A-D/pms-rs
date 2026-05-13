use uuid::Uuid;

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

    pub target:
        ProjectionRefreshTarget,
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub enum ProjectionRefreshTarget {

    Global,
    
    Guest {
        guest_id: Uuid,
    },

    InventoryDate {
        date: String,
    },
}

impl ProjectionInvalidation {

    pub fn new(
        source: ProjectionNode,
        scope: ProjectionScope,
        target: ProjectionRefreshTarget,
    ) -> Self {

        Self {
            source,
            scope,
            target,
        }
    }
}