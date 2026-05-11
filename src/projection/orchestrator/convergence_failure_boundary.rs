#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
)]
pub enum ConvergenceFailureBoundary {
    BeforeBoundaryVisibility,
    AfterPartialMaterialization,
}