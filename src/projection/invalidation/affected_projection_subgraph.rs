use crate::projection::topology::{
    projection_dependency::ProjectionDependency, projection_node::ProjectionNode,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AffectedProjectionSubgraph {
    pub nodes: Vec<ProjectionNode>,

    pub dependencies: Vec<ProjectionDependency>,
}

impl AffectedProjectionSubgraph {
    pub fn new(nodes: Vec<ProjectionNode>, dependencies: Vec<ProjectionDependency>) -> Self {
        Self {
            nodes,
            dependencies,
        }
    }

    pub fn contains_node(&self, node: ProjectionNode) -> bool {
        self.nodes.contains(&node)
    }

    pub fn downstream_of(&self, node: ProjectionNode) -> Vec<ProjectionNode> {
        self.dependencies
            .iter()
            .filter(|dependency| dependency.upstream == node)
            .map(|dependency| dependency.downstream)
            .collect()
    }

    pub fn contains_dependency(
        &self,
        upstream: ProjectionNode,

        downstream: ProjectionNode,
    ) -> bool {
        self.dependencies.iter().any(|dependency| {
            dependency.upstream == upstream && dependency.downstream == downstream
        })
    }
}
