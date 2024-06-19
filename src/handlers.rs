use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use axum::{extract::State, Json};
        use diesel::prelude::*;
        use serde::Deserialize;
        use crate::database::db::DbPool;
        use crate::models::conversations::{NewMessage, Thread};
        use log::error;

        #[derive(Deserialize)]
        pub struct MessagePayload {
            thread_id: String,
            content: String,
            role: String,
            active_model: String,
            active_lab: Option<String>,
        }

        pub async fn create_message(
            State(pool): State<DbPool>,
            Json(payload): Json<MessagePayload>,
        ) -> Result<(), String> {
            use crate::schema::{messages, threads};

            let new_message = NewMessage {
                thread_id: Some(payload.thread_id.clone()),
                content: Some(payload.content),
                role: payload.role,
                active_model: payload.active_model,
                active_lab: payload.active_lab,
            };

            let mut conn = pool.get().map_err(|err| {
                error!("Failed to get DB connection: {}", err);
                "Failed to get DB connection".to_string()
            })?;

            let result: Result<(), diesel::result::Error> = conn.transaction(|conn| {
                if threads::table.find(&payload.thread_id).first::<Thread>(conn).optional()?.is_none() {
                    let new_thread = Thread {
                        id: payload.thread_id.clone(),
                        created_at: None,
                        updated_at: None,
                    };
                    diesel::insert_into(threads::table)
                        .values(&new_thread)
                        .execute(conn)?;
                }

                diesel::insert_into(messages::table)
                    .values(&new_message)
                    .execute(conn)?;

                Ok(())
            });

            result.map_err(|err| {
                error!("Failed to insert message: {}", err);
                "Failed to insert message".to_string()
            })?;

            Ok(())
        }
    }
}
