use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use log::LevelFilter;
        use std::env;

        pub fn init_logging() {
            let log_level = env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string());
            let level = match log_level.to_lowercase().as_str() {
                "trace" => LevelFilter::Trace,
                "debug" => LevelFilter::Debug,
                "info" => LevelFilter::Info,
                "warn" => LevelFilter::Warn,
                "error" => LevelFilter::Error,
                _ => LevelFilter::Info,
            };
            env_logger::Builder::new().filter_level(level).init();
        }
    } else {
        use wasm_bindgen::prelude::*;

        #[wasm_bindgen]
        extern "C" {
            #[wasm_bindgen(js_namespace = console)]
            pub fn log(s: &str);
        }

        pub fn init_logging() {
            // Client-side logging initialization (if needed)
            // For now, we'll just use console.log
        }
    }
}

#[macro_export]
macro_rules! custom_log {
    ($level:expr, $($arg:tt)+) => {{
        cfg_if::cfg_if! {
            if #[cfg(feature = "ssr")] {
                match $level {
                    "trace" => log::trace!("[SERVER] {}", format_args!($($arg)+)),
                    "debug" => log::debug!("[SERVER] {}", format_args!($($arg)+)),
                    "info" => log::info!("[SERVER] {}", format_args!($($arg)+)),
                    "warn" => log::warn!("[SERVER] {}", format_args!($($arg)+)),
                    "error" => log::error!("[SERVER] {}", format_args!($($arg)+)),
                    _ => log::info!("[SERVER] {}", format_args!($($arg)+)),
                }
            } else {
                let msg = format!("[CLIENT] [{}] {}", $level.to_uppercase(), format_args!($($arg)+));
                $crate::wogging::log(&msg);
            }
        }
    }};
}

#[macro_export]
macro_rules! log_trace { ($($arg:tt)+) => { $crate::custom_log!("trace", $($arg)+); }; }
#[macro_export]
macro_rules! log_debug { ($($arg:tt)+) => { $crate::custom_log!("debug", $($arg)+); }; }
#[macro_export]
macro_rules! log_info  { ($($arg:tt)+) => { $crate::custom_log!("info",  $($arg)+); }; }
#[macro_export]
macro_rules! log_warn  { ($($arg:tt)+) => { $crate::custom_log!("warn",  $($arg)+); }; }
#[macro_export]
macro_rules! log_error { ($($arg:tt)+) => { $crate::custom_log!("error", $($arg)+); }; }

// Re-export macros for easier importing
pub use {custom_log, log_trace, log_debug, log_info, log_warn, log_error};
