use crate::projection::{
    invalidation::affected_projection_subgraph::AffectedProjectionSubgraph,
    topology::{convergence_step::ConvergenceStep, projection_node::ProjectionNode},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConvergenceTraversalPlan {
    affected_subgraph: AffectedProjectionSubgraph,

    steps: Vec<ConvergenceStep>,
}

impl ConvergenceTraversalPlan {
    pub fn new(affected_subgraph: AffectedProjectionSubgraph, steps: Vec<ConvergenceStep>) -> Self {
        Self {
            affected_subgraph,
            steps,
        }
    }

    pub fn steps(&self) -> &[ConvergenceStep] {
        &self.steps
    }

    pub fn affected_subgraph(&self) -> &AffectedProjectionSubgraph {
        &self.affected_subgraph
    }

    pub fn convergence_nodes(&self) -> Vec<ProjectionNode> {
        self.steps.iter().map(|step| step.node()).collect()
    }
}
