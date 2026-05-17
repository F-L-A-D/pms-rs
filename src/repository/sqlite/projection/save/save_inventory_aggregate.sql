INSERT INTO inventory_aggregates (
            service_date,
            room_class,
            total_rooms,
            out_of_order_rooms,
            reservable_rooms,
            confirmed_reservations,
            pending_reservations,
            cancelled_reservations,
            available_rooms,
            available_rooms_including_pending,
            projection_version,
            updated_at
        )
        VALUES (
            ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?
        )
        ON CONFLICT(service_date, room_class)
        DO UPDATE SET
            total_rooms                        = excluded.total_rooms,
            out_of_order_rooms                 = excluded.out_of_order_rooms,
            reservable_rooms                   = excluded.reservable_rooms,
            confirmed_reservations             = excluded.confirmed_reservations,
            pending_reservations               = excluded.pending_reservations,
            cancelled_reservations             = excluded.cancelled_reservations,
            available_rooms                    = excluded.available_rooms,
            available_rooms_including_pending  = excluded.available_rooms_including_pending,
            projection_version                 = excluded.projection_version,
            updated_at                         = excluded.updated_at
