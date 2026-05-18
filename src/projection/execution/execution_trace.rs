use std::sync::{Mutex, OnceLock};

use crate::projection::topology::projection_node::ProjectionNode;

static TRACE: OnceLock<Mutex<Vec<ProjectionNode>>> = OnceLock::new();

fn trace_storage() -> &'static Mutex<Vec<ProjectionNode>> {
    TRACE.get_or_init(|| Mutex::new(vec![]))
}

pub fn clear_trace() {
    trace_storage().lock().unwrap().clear();
}

pub fn push_trace(node: ProjectionNode) {
    trace_storage().lock().unwrap().push(node);
}

pub fn execution_trace() -> Vec<ProjectionNode> {
    trace_storage().lock().unwrap().clone()
}
