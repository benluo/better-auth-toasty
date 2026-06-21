use better_auth_toasty::ToastyAdapter;
use toasty::Db;

pub async fn setup_adapter(connection_url: &str) -> ToastyAdapter {
    let db = Db::builder()
        .models(toasty::models!(
            better_auth_toasty::models::User,
            better_auth_toasty::models::Session,
            better_auth_toasty::models::Account,
            better_auth_toasty::models::Verification,
        ))
        .connect(connection_url)
        .await
        .expect("failed to connect to database");

    db.push_schema()
        .await
        .expect("failed to push schema");

    ToastyAdapter::new(db)
}
