#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
)]
pub enum ConvergenceFailureKind {
    ProjectionExecutionFailure,
    BoundaryViolation,
    TopologyViolation,
}