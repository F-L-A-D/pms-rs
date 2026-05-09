#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
)]
pub enum ProjectionScope {

    Global,

    Date {
        date: String,
    },
}