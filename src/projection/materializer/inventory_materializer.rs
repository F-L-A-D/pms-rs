use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct InventoryProjectionRow {
    pub date: String,
    pub room_class: String,
    pub reserved_rooms: i32,
}

#[derive(Debug, Clone)]
pub struct HotelInventoryView {
    pub date: String,
    pub reserved_rooms: i32,
}

pub fn materialize_hotel_inventory(
    rows: Vec<InventoryProjectionRow>,
) -> Vec<HotelInventoryView> {

    let mut aggregated:
        HashMap<String, i32> = HashMap::new();

    for row in rows {

        *aggregated
            .entry(row.date)
            .or_insert(0)
            += row.reserved_rooms;
    }

    let mut result =
        aggregated
            .into_iter()
            .map(|(date, reserved_rooms)| {
                HotelInventoryView {
                    date,
                    reserved_rooms,
                }
            })
            .collect::<Vec<_>>();

    result.sort_by(|a, b| {
        a.date.cmp(&b.date)
    });

    result
}