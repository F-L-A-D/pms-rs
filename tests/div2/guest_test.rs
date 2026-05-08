use pms_rs::db::connection::Db;

use pms_rs::domain::guest::Guest;

use pms_rs::repository::sqlite::guest_repository::SqliteGuestRepository;

#[tokio::test]
async fn should_create_and_find_guest() {

    let db = Db::new("sqlite::memory:").await;

    let guest =
        Guest::new(
            "guest-001".into(),
            "Yamada".into(),
            "Taro".into(),
            Some("09012345678".into()),
            Some("test@example.com".into()),
        )
        .unwrap();

    SqliteGuestRepository::save(
        &db.pool,
        &guest,
    )
    .await
    .unwrap();

    let found =
        SqliteGuestRepository::find_by_id(
            &db.pool,
            "guest-001",
        )
        .await
        .unwrap()
        .unwrap();

    assert_eq!(guest, found);
}

#[tokio::test]
async fn should_fail_when_last_name_empty() {

    let result =
        Guest::new(
            "guest-001".into(),
            "".into(),
            "Taro".into(),
            None,
            None,
        );

    assert!(result.is_err());
}

#[tokio::test]
async fn should_update_guest_profile() {

    let db = Db::new("sqlite::memory:").await;

    let mut guest =
        Guest::new(
            "guest-001".into(),
            "Yamada".into(),
            "Taro".into(),
            None,
            None,
        )
        .unwrap();

    SqliteGuestRepository::save(
        &db.pool,
        &guest,
    )
    .await
    .unwrap();

    guest.update_profile(
        "Suzuki".into(),
        "Jiro".into(),
        Some("09099999999".into()),
        Some("updated@example.com".into()),
    )
    .unwrap();

    SqliteGuestRepository::update(
        &db.pool,
        &guest,
    )
    .await
    .unwrap();

    let found =
        SqliteGuestRepository::find_by_id(
            &db.pool,
            "guest-001",
        )
        .await
        .unwrap()
        .unwrap();

    assert_eq!(found.last_name, "Suzuki");
    assert_eq!(found.first_name, "Jiro");
}

#[tokio::test]
async fn should_find_guest_by_name() {

    let db = Db::new("sqlite::memory:").await;

    let guest1 =
        Guest::new(
            "guest-001".into(),
            "Suzuki".into(),
            "Taro".into(),
            None,
            None,
        )
        .unwrap();

    let guest2 =
        Guest::new(
            "guest-002".into(),
            "Yamada".into(),
            "Hanako".into(),
            None,
            None,
        )
        .unwrap();

    SqliteGuestRepository::save(
        &db.pool,
        &guest1,
    )
    .await
    .unwrap();

    SqliteGuestRepository::save(
        &db.pool,
        &guest2,
    )
    .await
    .unwrap();

    let result =
        SqliteGuestRepository::find_by_name(
            &db.pool,
            "Suzuki",
        )
        .await
        .unwrap();

    assert_eq!(result.len(), 1);

    assert_eq!(
        result[0].last_name,
        "Suzuki",
    );
}