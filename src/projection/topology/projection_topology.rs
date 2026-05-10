use crate::projection::invalidation::
    projection_scope::ProjectionScope;

use super::{
    projection_dependency::{
        InvalidationPolicy,
        ProjectionDependency,
    },

    projection_node::
        ProjectionNode,
};

pub fn projection_dependencies()
    -> Vec<ProjectionDependency>
{
    vec![
        ProjectionDependency {

            upstream:
                ProjectionNode::Inventory,

            downstream:
                ProjectionNode::HotelInventory,

            invalidation_policy:
                InvalidationPolicy::ScopeMatch {
                    scope:
                        ProjectionScope::Inventory,
                },
        },
    ]
}