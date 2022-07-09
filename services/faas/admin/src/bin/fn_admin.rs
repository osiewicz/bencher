use std::sync::Mutex;

use fn_admin::api::Api;

const API_NAME: &str = "admin";

#[tokio::main]
async fn main() -> Result<(), String> {
    let db_connection = util::db::get_db_connection().map_err(|e| e.to_string())?;
    util::server::get_server(API_NAME, &mut Api, Mutex::new(db_connection))
        .await?
        .await
}