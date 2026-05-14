use crate::projection::{
    aggregate::model::
        guest_aggregate::
            GuestAggregate,

    signal::model::
        guest_activity_signal::
            GuestActivitySignal,
};

pub fn materialize_guest_activity_signal(
    aggregate: &GuestAggregate,
) -> GuestActivitySignal
{
    GuestActivitySignal::new(
        aggregate.guest_id,
        true,
    )
}