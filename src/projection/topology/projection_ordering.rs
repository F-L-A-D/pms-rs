use std::collections::{
    HashSet,
    VecDeque,
};

use super::{
    projection_node::
        ProjectionNode,

    projection_topology::
        projection_dependencies,
};

use crate::projection::invalidation::{
    invalidation_traversal_plan::
        InvalidationTraversalPlan,

    projection_invalidation::
        ProjectionInvalidation,
};

pub fn downstream_of(
    node: ProjectionNode,
) -> Vec<ProjectionNode>
{
    projection_dependencies()
        .into_iter()
        .filter(
            |dependency|
                dependency.upstream == node
        )
        .map(
            |dependency|
                dependency.downstream
        )
        .collect()
}

pub fn rebuild_order(
    start: ProjectionNode,
) -> Vec<ProjectionNode>
{
    let mut ordered =
        Vec::new();

    let mut queue =
        VecDeque::new();

    let mut visited =
        HashSet::new();

    queue.push_back(start);

    while let Some(current)
        = queue.pop_front()
    {
        if visited.contains(&current) {
            continue;
        }

        visited.insert(current);

        ordered.push(current);

        for downstream
            in downstream_of(current)
        {
            queue.push_back(
                downstream
            );
        }
    }

    ordered
}

pub fn invalidation_traversal_plan(
    invalidation: &ProjectionInvalidation,
) -> InvalidationTraversalPlan {

    let ordered =
        rebuild_order(
            invalidation.source
        );

    InvalidationTraversalPlan::new(
        ordered,
        invalidation.scope.clone(),
    )
}