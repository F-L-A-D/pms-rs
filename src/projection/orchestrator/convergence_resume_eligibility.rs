#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
)]
pub enum ConvergenceResumeEligibility {
    ResumeAllowed,
    ResumeRejected,
}