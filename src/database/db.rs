use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use diesel::prelude::*;
        use diesel::PgConnection;
        use deadpool_diesel::{Manager, Pool, Runtime};

        use crate::models::conversations::{NewMessage, Thread, Message};
        use crate::schema::{threads, messages};

        pub type DbPool = Pool<Manager<PgConnection>>;

        pub fn establish_connection(database_url: &str) -> DbPool {
            let manager = Manager::new(database_url, Runtime::Tokio1);
            Pool::builder(manager)
                .max_size(8)
                .build()
                .expect("Failed to create pool.")
        }
        
        pub fn create_thread(conn: &mut PgConnection, new_thread: &Thread) -> QueryResult<usize> {
            diesel::insert_into(threads::table)
                .values(new_thread)
                .execute(conn)
        }
        
        pub fn add_message(conn: &mut PgConnection, new_message: &NewMessage) -> QueryResult<usize> {
            diesel::insert_into(messages::table)
                .values(new_message)
                .execute(conn)
        }
        
        pub fn get_messages_by_thread(conn: &mut PgConnection, thread_id: &str) -> QueryResult<Vec<Message>> {
            messages::table
                .filter(messages::thread_id.eq(thread_id))
                .load::<Message>(conn)
        }
    }
}
