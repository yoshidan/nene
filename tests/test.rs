mod gen;

use gen::user;
use std::env;

#[tokio::test]
async fn test_generated() {
    let v = user::User {
        user_id: "test_user".to_string(),
        ..Default::default()
    };

    let database = env::var("SPANNER_DSN").unwrap();
    let client = google_cloud_spanner::client::Client::new(database)
        .await
        .unwrap();
    let _ = client.apply(vec![v.insert_or_update()]).await.unwrap();
    let mut tx = client.single().await.unwrap();
    let user = user::User::find_by_pk(&mut tx, &"test_user", None)
        .await
        .unwrap();
    assert!(user.is_some());

    let ser = serde_json::to_string(&v).unwrap();
    let dser: user::User = serde_json::from_str(ser.as_str()).unwrap();
    assert_eq!(dser.user_id, v.user_id);
    assert_eq!(dser.updated_at, v.updated_at);
    assert_eq!(dser.not_null_date, v.not_null_date);
    assert_eq!(dser.not_null_timestamp, v.not_null_timestamp);
}
