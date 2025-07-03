use crate::config::AppEnv;
use std::{backtrace, panic, thread};
use tracing::{error, level_filters::LevelFilter};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling;
use tracing_subscriber::prelude::*;

pub struct Logger;

impl Logger {
    pub fn init(app_env: &AppEnv) -> Option<WorkerGuard> {
        // initialise tracing based on environment and keep guard alive if using non_blocking writer
        let guard_opt = match app_env {
            // Development: log everything to stdout with color and DEBUG level
            AppEnv::Development => {
                let console_logger = std::io::stdout();
                let (non_blocking, guard) = tracing_appender::non_blocking(console_logger);

                tracing_subscriber::fmt()
                    .with_writer(non_blocking)
                    .with_max_level(LevelFilter::DEBUG)
                    .init();

                Some(guard)
            }

            // Production: separate INFO (access) and ERROR logs, still echo INFO to stdout
            AppEnv::Production => {
                let access_file = rolling::daily("logs", "access.log");
                let error_file = rolling::daily("logs", "error.log");

                use tracing::Level;
                use tracing_subscriber::filter::filter_fn;

                let access_layer = tracing_subscriber::fmt::layer()
                    .with_writer(access_file)
                    .with_ansi(false)
                    .with_filter(filter_fn(|meta| meta.level() == &Level::INFO));

                let error_layer = tracing_subscriber::fmt::layer()
                    .with_writer(error_file)
                    .with_ansi(false)
                    .with_filter(filter_fn(|meta| meta.level() == &Level::ERROR));

                tracing_subscriber::registry()
                    .with(access_layer)
                    .with(error_layer)
                    .init();

                None
            }
        };

        // catch panic and log them using tracing instead of default output to StdErr
        panic::set_hook(Box::new(|info| {
            let thread = thread::current();
            let thread = thread.name().unwrap_or("unknown");

            let msg = match info.payload().downcast_ref::<&'static str>() {
                Some(s) => *s,
                None => match info.payload().downcast_ref::<String>() {
                    Some(s) => &**s,
                    None => "Box<Any>",
                },
            };

            let backtrace = backtrace::Backtrace::capture();

            match info.location() {
                Some(location) => {
                    // without backtrace
                    if msg.starts_with("notrace - ") {
                        error!(
                            target: "panic", "thread '{}' panicked at '{}': {}:{}",
                            thread,
                            msg.replace("notrace - ", ""),
                            location.file(),
                            location.line()
                        );
                    }
                    // with backtrace
                    else {
                        error!(
                            target: "panic", "thread '{}' panicked at '{}': {}:{}\n{:?}",
                            thread,
                            msg,
                            location.file(),
                            location.line(),
                            backtrace
                        );
                    }
                }
                None => {
                    // without backtrace
                    if msg.starts_with("notrace - ") {
                        error!(
                            target: "panic", "thread '{}' panicked at '{}'",
                            thread,
                            msg.replace("notrace - ", ""),
                        );
                    }
                    // with backtrace
                    else {
                        error!(
                            target: "panic", "thread '{}' panicked at '{}'\n{:?}",
                            thread,
                            msg,
                            backtrace
                        );
                    }
                }
            }
        }));
        guard_opt
    }
}
