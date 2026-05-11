use std::collections::{
    HashSet,
    VecDeque,
};

use crate::projection::invalidation::{
    projection_invalidation::
        ProjectionInvalidation,
    
    affected_projection_subgraph::
        AffectedProjectionSubgraph,
};

use super::{
    projection_dependency::
        ProjectionDependency,

    projection_node::
        ProjectionNode,

    projection_topology::
        projection_dependencies,
    
    convergence_traversal_plan::
        ConvergenceTraversalPlan,
};

pub fn downstream_of(
    node: ProjectionNode,
    invalidation:
        &ProjectionInvalidation,
) -> Vec<ProjectionDependency>
{
    projection_dependencies()
        .into_iter()

        .filter(
            |dependency| {

                dependency.upstream
                    == node

                &&

                dependency
                    .should_propagate(
                        invalidation
                    )
            }
        )

        .collect()
}

pub fn derive_convergence_plan(
    invalidation:
        &ProjectionInvalidation,
) -> ConvergenceTraversalPlan {

    let mut ordered =
        Vec::new();

    let mut traversed_dependencies =
        Vec::new();

    let mut queue =
        VecDeque::new();

    let mut visited =
        HashSet::new();

    queue.push_back(
        invalidation.source
    );

    while let Some(current)
        = queue.pop_front()
    {
        if visited.contains(&current) {
            continue;
        }

        visited.insert(current);

        ordered.push(current);

        for dependency in
            downstream_of(
                current,
                invalidation,
            )
        {
            traversed_dependencies.push(
                dependency.clone()
            );

            queue.push_back(
                dependency.downstream
            );
        }
    }

    let affected_subgraph =
        AffectedProjectionSubgraph::new(
            ordered.clone(),
            traversed_dependencies,
        );

    ConvergenceTraversalPlan::new(
        affected_subgraph,
        ordered,
        invalidation.scope.clone(),
    )
}