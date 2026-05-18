use crate::projection::{
    orchestrator::{
        convergence_checkpoint::ConvergenceCheckpointability,
        convergence_failure_boundary::ConvergenceFailureBoundary,
        convergence_failure_kind::ConvergenceFailureKind,
        convergence_resume_eligibility::ConvergenceResumeEligibility,
        convergence_visibility::ConvergenceVisibility,
    },
    topology::{
        convergence_traversal_plan::ConvergenceTraversalPlan, projection_node::ProjectionNode,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConvergenceExecutionStatus {
    Fulfilled,
    Failed,
    Aborted,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConvergenceExecutionResult {
    pub plan: ConvergenceTraversalPlan,

    pub completed_nodes: Vec<ProjectionNode>,

    pub status: ConvergenceExecutionStatus,

    pub visibility: ConvergenceVisibility,

    pub resume_eligibility: ConvergenceResumeEligibility,

    pub checkpointability: ConvergenceCheckpointability,

    pub failure_kind: Option<ConvergenceFailureKind>,

    pub failure_boundary: Option<ConvergenceFailureBoundary>,

    pub failed_node: Option<ProjectionNode>,
}

impl ConvergenceExecutionResult {
    pub fn fulfilled(plan: ConvergenceTraversalPlan, completed_nodes: Vec<ProjectionNode>) -> Self {
        Self {
            plan,
            completed_nodes,

            status: ConvergenceExecutionStatus::Fulfilled,

            visibility: ConvergenceVisibility::BoundaryVisible,

            resume_eligibility: ConvergenceResumeEligibility::ResumeRejected,

            checkpointability: ConvergenceCheckpointability::CheckpointRejected,

            failure_kind: None,

            failure_boundary: None,

            failed_node: None,
        }
    }

    pub fn failed(
        plan: ConvergenceTraversalPlan,
        completed_nodes: Vec<ProjectionNode>,
        failed_node: ProjectionNode,
    ) -> Self {
        Self {
            plan,
            completed_nodes,

            status: ConvergenceExecutionStatus::Failed,

            visibility: ConvergenceVisibility::NotVisible,

            resume_eligibility: ConvergenceResumeEligibility::ResumeAllowed,

            checkpointability: ConvergenceCheckpointability::CheckpointRejected,

            failure_kind: Some(ConvergenceFailureKind::ProjectionExecutionFailure),

            failure_boundary: Some(ConvergenceFailureBoundary::AfterPartialMaterialization),

            failed_node: Some(failed_node),
        }
    }

    pub fn aborted(plan: ConvergenceTraversalPlan, completed_nodes: Vec<ProjectionNode>) -> Self {
        Self {
            plan,
            completed_nodes,

            status: ConvergenceExecutionStatus::Aborted,

            visibility: ConvergenceVisibility::NotVisible,

            resume_eligibility: ConvergenceResumeEligibility::ResumeAllowed,

            checkpointability: ConvergenceCheckpointability::CheckpointRejected,

            failure_kind: None,

            failure_boundary: Some(ConvergenceFailureBoundary::BeforeBoundaryVisibility),

            failed_node: None,
        }
    }

    pub fn convergence_fulfilled(&self) -> bool {
        self.status == ConvergenceExecutionStatus::Fulfilled
    }

    pub fn completed_all_nodes(&self) -> bool {
        self.plan
            .convergence_nodes()
            .iter()
            .all(|node| self.completed_nodes.contains(node))
    }

    pub fn partially_completed(&self) -> bool {
        !self.completed_nodes.is_empty() && !self.completed_all_nodes()
    }

    pub fn authoritative_boundary_fulfilled(&self) -> bool {
        self.convergence_fulfilled() && self.completed_all_nodes()
    }

    pub fn authoritatively_visible(&self) -> bool {
        self.visibility == ConvergenceVisibility::BoundaryVisible
    }

    pub fn preserves_boundary_atomicity(&self) -> bool {
        self.authoritative_boundary_fulfilled() && self.authoritatively_visible()
    }

    pub fn resumable(&self) -> bool {
        self.resume_eligibility == ConvergenceResumeEligibility::ResumeAllowed
    }

    pub fn checkpointable(&self) -> bool {
        self.checkpointability == ConvergenceCheckpointability::CheckpointAllowed
    }

    pub fn completed_boundary_nodes(&self) -> &[ProjectionNode] {
        &self.completed_nodes
    }

    pub fn remaining_nodes(&self) -> Vec<ProjectionNode> {
        self.plan
            .convergence_nodes()
            .iter()
            .filter(|node| !self.completed_nodes.contains(node))
            .copied()
            .collect()
    }

    pub fn preserves_resume_boundary(&self) -> bool {
        self.partially_completed() && self.resumable() && !self.remaining_nodes().is_empty()
    }

    pub fn preserves_checkpoint_boundary(&self) -> bool {
        self.completed_boundary_nodes()
            .iter()
            .all(|node| self.plan.convergence_nodes().contains(node))
    }

    pub fn failed_due_to_projection_execution(&self) -> bool {
        self.failure_kind == Some(ConvergenceFailureKind::ProjectionExecutionFailure)
    }

    pub fn aborted_before_boundary_visibility(&self) -> bool {
        self.status == ConvergenceExecutionStatus::Aborted && !self.authoritatively_visible()
    }

    pub fn preserves_authoritative_isolation(&self) -> bool {
        match self.failure_boundary {
            None => self.authoritative_boundary_fulfilled(),

            Some(ConvergenceFailureBoundary::BeforeBoundaryVisibility) => {
                !self.authoritatively_visible()
            }

            Some(ConvergenceFailureBoundary::AfterPartialMaterialization) => {
                !self.authoritatively_visible() && self.partially_completed()
            }
        }
    }

    pub fn convergence_failed_at(&self, node: ProjectionNode) -> bool {
        self.failed_node == Some(node)
    }

    pub fn preserves_failure_isolation(&self) -> bool {
        match self.failed_node {
            None => true,

            Some(node) => !self.completed_nodes.contains(&node),
        }
    }
}
