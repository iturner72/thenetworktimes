use cfg_if::cfg_if;
cfg_if! { if #[cfg(feature = "ssr")] {
    use crate::schema::*;
    use diesel::prelude::*;
    use serde::{Deserialize, Serialize};
    use chrono::NaiveDateTime;

    #[derive(Debug, Serialize, Queryable, Deserialize, Identifiable, Insertable)]
    #[diesel(table_name = threads)]
    pub struct Thread {
        #[diesel(column_name = id)]
        pub id: String,
        pub created_at: Option<NaiveDateTime>,
        pub updated_at: Option<NaiveDateTime>,
    }

    // used for querying messages directly from the database
    #[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, Insertable, Associations, Default)]
    #[diesel(belongs_to(Thread, foreign_key = thread_id))]
    #[diesel(table_name = messages)]
    pub struct Message {
        pub id: i32,
        #[diesel(column_name = thread_id)]
        pub thread_id: Option<String>,
        pub content: Option<String>,
        pub role: String,
        pub active_model: String,
        pub active_lab: Option<String>,
        pub created_at: Option<NaiveDateTime>,
        pub updated_at: Option<NaiveDateTime>,
    }

    // message data from the client ("new type" or "insert type" pattern)
    #[derive(Insertable, Deserialize)]
    #[diesel(table_name = messages)]
    pub struct NewMessage {
        pub thread_id: Option<String>,
        pub content: Option<String>,
        pub role: String,
        pub active_model: String,
        pub active_lab: Option<String>,
    }
}}
