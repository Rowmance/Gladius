//! Logging related config

use chrono;
use fern;
use fern::colors::ColoredLevelConfig;
use log;
use std;

/// Sets up the logger.
pub fn setup() -> Result<(), fern::InitError> {
    let colors = ColoredLevelConfig::default();
    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{} [{}] {} {}",
                chrono::Local::now().format("%H:%M:%S"),
                record.target(),
                colors.color(record.level()),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
//        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}

// TODO: Add logging in key areas
// TODO: Allow log-level overriding without source code modifications (ie, environment variables)
// TODO: https://github.com/rust-lang/rfcs/issues/1664
