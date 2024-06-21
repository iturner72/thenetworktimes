use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use axum::extract::FromRef;
        use leptos::LeptosOptions;
        use crate::database::db::DbPool;

        #[derive(FromRef, Clone)]
        pub struct AppState {
            pub leptos_options: LeptosOptions,
            pub pool: DbPool,
        }
    }
}

