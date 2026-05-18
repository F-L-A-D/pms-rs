use crate::common::{app::spawn_app, room::create_room};

#[tokio::test]
async fn shoud_create_room() {
    let app = spawn_app().await;

    let room = create_room(&app.app).await;

    assert_eq!(room.room_class, "standard",);
}
