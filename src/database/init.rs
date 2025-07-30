use crate::{errors::core::Result, state::AppState};
use std::sync::Arc;

pub async fn initialize_database(app_state: Arc<AppState>) -> Result<()> {
    app_state
        .db
        .query(
            "
        DEFINE TABLE users SCHEMAFULL;
        DEFINE FIELD id ON users TYPE string;
        DEFINE FIELD name ON users TYPE string;
        DEFINE FIELD email ON users TYPE string;
        DEFINE FIELD password ON users TYPE string;
        DEFINE FIELD role ON users TYPE string;
        DEFINE FIELD verified ON users TYPE bool;
        DEFINE FIELD created_at ON users TYPE datetime;
        DEFINE FIELD updated_at ON users TYPE datetime;
        DEFINE INDEX email_idx ON users COLUMNS email UNIQUE;
    ",
        )
        .await
        .map_err(|e| crate::errors::db::DatabaseError::query_failed(e, None))?;

    app_state
        .db
        .query(
            "
        DEFINE TABLE token_sessions SCHEMAFULL;
        DEFINE FIELD id ON token_sessions TYPE string;
        DEFINE FIELD user_id ON token_sessions TYPE string;
        DEFINE FIELD access_token_jti ON token_sessions TYPE string;
        DEFINE FIELD refresh_token_jti ON token_sessions TYPE string;
        DEFINE FIELD created_at ON token_sessions TYPE datetime;
        DEFINE FIELD last_active_at ON token_sessions TYPE datetime;
        DEFINE FIELD is_active ON token_sessions TYPE bool;
        DEFINE FIELD device_info ON token_sessions TYPE option<string>;
        DEFINE FIELD ip_address ON token_sessions TYPE option<string>;
        DEFINE FIELD location ON token_sessions TYPE option<string>;
        DEFINE INDEX access_jti_idx ON token_sessions COLUMNS access_token_jti;
        DEFINE INDEX refresh_jti_idx ON token_sessions COLUMNS refresh_token_jti;
    ",
        )
        .await
        .map_err(|e| crate::errors::db::DatabaseError::query_failed(e, None))?;

    Ok(())
}
