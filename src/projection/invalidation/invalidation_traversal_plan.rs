use super::{
    affected_projection_subgraph::
        AffectedProjectionSubgraph,

    projection_scope::
        ProjectionScope,
};

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub struct InvalidationTraversalPlan {

    pub affected_subgraph:
        AffectedProjectionSubgraph,

    pub scope:
        ProjectionScope,
}

impl InvalidationTraversalPlan {

    pub fn new(
        affected_subgraph:
            AffectedProjectionSubgraph,

        scope:
            ProjectionScope,
    ) -> Self {

        Self {
            affected_subgraph,
            scope,
        }
    }
}