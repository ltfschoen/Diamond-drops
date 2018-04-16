use log;
use log::{Record, Level, Metadata, SetLoggerError, LevelFilter};
use chrono::Local;
use log4rs;

use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Logger, Root};

use std::env;
use std::path::PathBuf;

static LOGGER: DiamondDropsLogger = DiamondDropsLogger;

struct DiamondDropsLogger;

impl log::Log for DiamondDropsLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Trace
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} [{}] - {}", Local::now().format("%Y-%m-%dT%H:%M:%S"), record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

pub fn init() -> () {
    /*! Initialisation of [Log Crate](https://crates.io/crates/log) with choice of logging level macros */
    /*! from highest priority to lowest: `error!`, `warn!`, `info!`, `debug!` and `trace!`. */
    /*! [Compile time filters](https://docs.rs/log/0.4.1/log/#compile-time-filters) are configured in Cargo.toml */

//    let logger = log::set_logger(&LOGGER);
//    match logger {
//        Ok(res) => {
//            log::set_max_level(LevelFilter::Trace);
//            info!("Success initializing Rust Logger to max level: {}", log::max_level());
////            ()
//        }
//        Err(e) => {
//            eprintln!("Error initializing Rust Logger: {}", e);
//        }
//    }



    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} - [{l}] - {m}{n}")))
        .build("log/output.log")
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder()
            .appender("logfile")
            .build(LevelFilter::Trace))
        .unwrap();

    let handle = log4rs::init_config(config)
        .unwrap();



//    let logger = log::set_logger(&LOGGER);
//    match logger {
//        Ok(res) => {
//            log::set_max_level(LevelFilter::Trace);
//            info!("Success initializing Rust Logger to max level: {}", log::max_level());
////            ()
//        }
//        Err(e) => {
//            eprintln!("Error initializing Rust Logger: {}", e);
//        }
//    }

//            let project_root_directory = env!("CARGO_MANIFEST_DIR").to_string();
//            let logger_config_file_suffix = "/log4rs.yaml";
//            let log_filename = project_root_directory + &logger_config_file_suffix;
//
//            println!("{:?}", log_filename);
//            let log4rs_config = log4rs::init_file(log_filename, Default::default());
//
//            match log4rs_config {
//                Ok(res) => { println!("DONE {:?}", res); }
//                Err(e) => { println!("ERR {:?}", e); }
//            }

//            log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
//            info!("Loaded Rust Logger config file {:?} and ready to record logs to file", &log_filename);
}
