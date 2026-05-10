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

        for dependency in
            projection_dependencies()
        {
            if dependency.upstream
                == current
            {
                queue.push_back(
                    dependency.downstream
                );
            }
        }
    }

    ordered
}