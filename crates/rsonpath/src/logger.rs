use color_eyre::eyre::{Result, WrapErr as _};
use log::LevelFilter;
use simple_logger::SimpleLogger;

pub(super) fn configure(verbose: bool) -> Result<()> {
    SimpleLogger::new()
        .with_level(if verbose { LevelFilter::Trace } else { LevelFilter::Warn })
        .init()
        .wrap_err("Logger configuration error.")
}
