use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use axum::{extract::State, Json};
        use diesel::prelude::*;
        use deadpool_diesel::postgres::{Manager, Pool, Runtime};
        use http::StatusCode;
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
        ) -> Result<(), StatusCode> {
            use crate::schema::{messages, threads};

            let new_message = NewMessage {
                thread_id: Some(payload.thread_id.clone()),
                content: Some(payload.content),
                role: payload.role,
                active_model: payload.active_model,
                active_lab: payload.active_lab,
            };

            let conn = pool.get().await.map_err(|err| {
                tracing::error!("Failed to get database connection: {:?}", err);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

            let result: Result<(), diesel::result::Error> = conn.interact(move |conn| {
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
            }).await.map_err(|err| {
                error!("Failed to insert message: {}", err);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

            result.map_err(|err| {
                error!("Failed to insert message: {}", err);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

            Ok(())
        }

        pub fn setup_database(database_url: &str) -> DbPool {
            let manager = Manager::new(database_url, Runtime::Tokio1); 
            let pool = Pool::builder(manager)
                .max_size(8) 
                .build()
                .expect("Failed to create pool.");
            pool
        }
    }
}
