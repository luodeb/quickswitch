use anyhow::{Ok, Result};
use chrono::Local;
use std::{env, fs::OpenOptions};
use tracing::{instrument, warn};
use tracing_appender::non_blocking;
use tracing_subscriber::{EnvFilter, fmt::time::Uptime};

#[instrument]
pub fn init_logging(verbose_level: u8, log_file: Option<&std::path::Path>) -> Result<()> {
    // set the default log level based on verbosity
    let warn_tag = verbose_level > 3;
    let filter = match verbose_level {
        0 => {
            if env::var("RUST_LOG").is_ok() {
                EnvFilter::from_default_env()
            } else {
                return Ok(());
            }
        }
        1 => EnvFilter::new("INFO"),
        2 => EnvFilter::new("DEBUG"),
        _ => EnvFilter::new("TRACE"),
    };

    // Initialize log file
    let writer = match log_file {
        Some(path) => OpenOptions::new()
            .create(true)
            .append(true)
            .write(true)
            .open(path)
            .map_err(|e| anyhow::anyhow!("Failed to open log file {:?}: {}", path, e))?,
        None => {
            let date = Local::now().format("%Y-%m-%d").to_string();
            let pid = std::process::id();
            let file = tempfile::Builder::new()
                .prefix(&format!("qw-{}-{}-", date, pid))
                .suffix(".log")
                .rand_bytes(5)
                .tempfile()?
                .keep()?;
            file.0
        }
    };
    let (appender, _guard) = non_blocking(writer);

    tracing_subscriber::fmt()
        .with_thread_ids(false)
        .with_thread_names(false)
        .with_target(false)
        .with_file(true)
        .with_line_number(true)
        .with_timer(Uptime::default())
        .with_writer(appender)
        .with_env_filter(filter)
        .init();

    // Keep the guard alive for the duration of the program
    std::mem::forget(_guard);

    if warn_tag {
        warn!("Invalid verbosity level. Defaulting to TRACE.");
    }

    Ok(())
}
