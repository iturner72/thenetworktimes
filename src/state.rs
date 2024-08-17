use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use axum::extract::FromRef;
        use leptos::LeptosOptions;
        use redis::aio::Connection;
        use std::pin::Pin;
        use futures::io::AsyncRead;
        use crate::database::db::DbPool;

        #[derive(FromRef, Clone)]
        pub struct AppState {
            pub leptos_options: LeptosOptions,
            pub pool: DbPool,
            pub redis_pool: Connection<Pin<Box<dyn AsyncRead + Send + Sync>>>,
        }
    }
}

