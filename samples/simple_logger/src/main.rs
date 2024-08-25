// 实现日志框架
use log::{error, info, warn, Level, LevelFilter, Metadata, Record};
use simple_logger::SimpleLogger;

static MY_LOGGER: SimpleLogger = SimpleLogger;

fn main() {
    log::set_logger(&MY_LOGGER).unwrap();
    log::set_max_level(LevelFilter::Info);

    info!("hello log");
    warn!("warning");
    error!("oops");
}
