use crate::constant::{AppConfig, Environment};
use std::fs;
use tracing::{Level, Subscriber};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{
  fmt::{self, format::FmtSpan},
  layer::SubscriberExt,
  util::SubscriberInitExt,
  Layer,
};

pub fn setup_logger(config: &AppConfig) -> anyhow::Result<()> {
  if !config.log_config.enabled {
    return Ok(());
  }

  // Create logs directory if it doesn't exist
  if config.is_production() {
    fs::create_dir_all("logs")?;
  }

  let log_level = match config.log_config.level.to_lowercase().as_str() {
    "trace" => Level::TRACE,
    "debug" => Level::DEBUG,
    "info" => Level::INFO,
    "warn" => Level::WARN,
    "error" => Level::ERROR,
    _ => Level::INFO,
  };

  match config.environment {
    Environment::Production => setup_production_logger(config, log_level)?,
    Environment::Development => setup_development_logger(log_level)?,
  }

  Ok(())
}

fn setup_production_logger(config: &AppConfig, log_level: Level) -> anyhow::Result<()> {
  let file_appender =
    RollingFileAppender::new(Rotation::DAILY, "logs", &config.log_config.file_path);

  let file_layer = fmt::layer()
    .with_file(true)
    .with_line_number(true)
    .with_thread_ids(true)
    .with_target(true)
    .with_span_events(FmtSpan::FULL)
    .with_writer(file_appender)
    .with_filter(tracing_subscriber::filter::LevelFilter::from_level(
      log_level,
    ));

  tracing_subscriber::registry().with(file_layer).try_init()?;

  Ok(())
}

fn setup_development_logger(log_level: Level) -> anyhow::Result<()> {
  let console_layer = fmt::layer()
    .with_file(true)
    .with_line_number(true)
    .with_thread_ids(true)
    .with_target(true)
    .with_span_events(FmtSpan::FULL)
    .with_filter(tracing_subscriber::filter::LevelFilter::from_level(
      log_level,
    ))
    .pretty();

  tracing_subscriber::registry()
    .with(console_layer)
    .try_init()?;

  Ok(())
}

// Convenience macros for logging
#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        tracing::error!($($arg)*);
    };
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        tracing::warn!($($arg)*);
    };
}

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        tracing::info!($($arg)*);
    };
}

#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        tracing::debug!($($arg)*);
    };
}

#[macro_export]
macro_rules! log_trace {
    ($($arg:tt)*) => {
        tracing::trace!($($arg)*);
    };
}
