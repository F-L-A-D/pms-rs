use crate::projection::{
    invalidation::projection_scope::ProjectionScope,

    topology::projection_node::ProjectionNode,

    invalidation::affected_projection_subgraph::
        AffectedProjectionSubgraph,
};

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub struct ConvergenceTraversalPlan {

    pub affected_subgraph:
        AffectedProjectionSubgraph,

    pub ordered_nodes:
        Vec<ProjectionNode>,

    pub scope:
        ProjectionScope,
}

impl ConvergenceTraversalPlan {

    pub fn new(
        affected_subgraph:
            AffectedProjectionSubgraph,

        ordered_nodes:
            Vec<ProjectionNode>,

        scope:
            ProjectionScope,
    ) -> Self {

        Self {
            affected_subgraph,
            ordered_nodes,
            scope,
        }
    }

    pub fn convergence_nodes(
        &self,
    ) -> &[ProjectionNode] {

        &self.ordered_nodes
    }

    pub fn contains(
        &self,
        node: ProjectionNode,
    ) -> bool {

        self.ordered_nodes
            .contains(&node)
    }
}