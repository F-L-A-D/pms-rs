INSERT INTO housekeeping_daily_workload_aggregates (
            service_date,
            room_class,
            total_tracked_rooms,
            dirty_rooms,
            cleaning_rooms,
            cleaned_rooms,
            inspected_rooms,
            occupied_rooms,
            vacant_rooms,
            out_of_order_rooms,
            projection_version,
            updated_at
        )
        VALUES (
            ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?
        )
        ON CONFLICT(service_date, room_class)
        DO UPDATE SET
            total_tracked_rooms = excluded.total_tracked_rooms,
            dirty_rooms         = excluded.dirty_rooms,
            cleaning_rooms      = excluded.cleaning_rooms,
            cleaned_rooms       = excluded.cleaned_rooms,
            inspected_rooms     = excluded.inspected_rooms,
            occupied_rooms      = excluded.occupied_rooms,
            vacant_rooms        = excluded.vacant_rooms,
            out_of_order_rooms  = excluded.out_of_order_rooms,
            projection_version  = excluded.projection_version,
            updated_at          = excluded.updated_at
