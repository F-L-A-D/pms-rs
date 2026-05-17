#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConvergenceCheckpointability {
    CheckpointAllowed,
    CheckpointRejected,
}
