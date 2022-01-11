use directories_next::BaseDirs;
use log4rs::append::console::{ConsoleAppender, Target};
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::filter::threshold::ThresholdFilter;
use log4rs::Handle;

pub fn init() -> Result<Handle, log::SetLoggerError> {
    let level = if cfg!(debug_assertions) {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Info
    };
    let stderr = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{h({d(%Y-%m-%d %H:%M:%S %Z)(local)} [{l}:{t}])} {m}{n}",
        )))
        .target(Target::Stderr)
        .build();
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S %Z)(local)} [{l}:{t}] {m}{n}",
        )))
        .append(false)
        .build(
            BaseDirs::new()
                .unwrap()
                .cache_dir()
                .join(format!("{}.log", env!("CARGO_PKG_NAME"))),
        )
        .unwrap();
    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(level)))
                .build("stderr", Box::new(stderr)),
        )
        .build(
            Root::builder()
                .appender("logfile")
                .appender("stderr")
                .build(log::LevelFilter::Trace),
        )
        .unwrap();
    log4rs::init_config(config)
}
