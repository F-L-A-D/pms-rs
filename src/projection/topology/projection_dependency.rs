use crate::projection::invalidation::{
    projection_invalidation::
        ProjectionInvalidation,

    projection_scope::
        ProjectionScope,
};

use super::projection_node::
    ProjectionNode;

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub enum InvalidationPolicy {

    Always,

    ScopeMatch {
        scope: ProjectionScope,
    },
}

#[derive(
    Debug,
    Clone,
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

    pub fn should_propagate(
        &self,
        invalidation:
            &ProjectionInvalidation,
    ) -> bool {

        match
            &self.invalidation_policy
        {
            InvalidationPolicy::Always => {
                true
            }

            InvalidationPolicy::ScopeMatch {
                scope
            } => {

                invalidation.scope
                    == *scope
            }
        }
    }
}