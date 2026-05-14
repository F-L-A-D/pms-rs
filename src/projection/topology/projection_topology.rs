use super::{
    projection_dependency::
        ProjectionDependency,

    projection_node::
        ProjectionNode,
};

pub fn projection_dependencies(
) -> Vec<ProjectionDependency>
{
    vec![
        ProjectionDependency::new(
            ProjectionNode::GuestAggregate,
            ProjectionNode::GuestActivitySignal,
        ),
    ]
}