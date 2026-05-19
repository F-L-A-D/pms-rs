use std::{
    sync::{Mutex, OnceLock},
    thread::{self, ThreadId},
};

use crate::projection::topology::projection_node::ProjectionNode;

#[derive(Default)]
struct ExecutionTrace {
    owner: Option<ThreadId>,
    nodes: Vec<ProjectionNode>,
}

static TRACE: OnceLock<Mutex<ExecutionTrace>> = OnceLock::new();

fn trace_storage() -> &'static Mutex<ExecutionTrace> {
    TRACE.get_or_init(|| Mutex::new(ExecutionTrace::default()))
}

pub fn clear_trace() {
    let mut trace = trace_storage().lock().unwrap();

    trace.owner = Some(thread::current().id());
    trace.nodes.clear();
}

pub fn push_trace(node: ProjectionNode) {
    let mut trace = trace_storage().lock().unwrap();

    if trace.owner == Some(thread::current().id()) {
        trace.nodes.push(node);
    }
}

pub fn execution_trace() -> Vec<ProjectionNode> {
    trace_storage().lock().unwrap().nodes.clone()
}
