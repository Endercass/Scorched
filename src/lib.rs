mod utils;

use owo_colors::{colors::css::*, OwoColorize};
use std::{fmt::Debug, fs::OpenOptions, io::prelude::*};

pub enum LogImportance {
    Error,
    Warning,
    Info,
    Debug,
}

pub struct LogData {
    pub importance: LogImportance,
    pub message: String,
}

pub fn log_this(data: LogData) {
    let formatted_time =
        utils::format_time::get_formatted_time(utils::format_time::TimeFormat::Ymdhms);

    //Creates logs folder if it doesn't exist
    if !std::path::Path::new("logs").exists() {
        std::fs::create_dir("logs").unwrap();
    }

    let file = OpenOptions::new().append(true).create(true).open(format!(
        "logs/{}.log",
        utils::format_time::get_formatted_time(utils::format_time::TimeFormat::Ymd)
    ));

    match data.importance {
        LogImportance::Error => {
            file.unwrap()
                .write_all(format!("{} [ERROR] {}\n", formatted_time, data.message).as_bytes())
                .unwrap();
            println!(
                "{} {} {}",
                formatted_time,
                "[ERROR]".fg::<Black>().bg::<Red>(),
                data.message
            );
        }
        LogImportance::Warning => {
            file.unwrap()
                .write_all(format!("{} [WARNING] {}\n", formatted_time, data.message).as_bytes())
                .unwrap();
            println!(
                "{} {} {}",
                formatted_time,
                "[WARNING]".fg::<Black>().bg::<Yellow>(),
                data.message
            );
        }
        LogImportance::Info => {
            file.unwrap()
                .write_all(format!("{} [INFO] {}\n", formatted_time, data.message).as_bytes())
                .unwrap();
            println!(
                "{} {} {}",
                formatted_time,
                "[INFO]".fg::<Black>().bg::<LightGray>(),
                data.message
            );
        }
        //Mainly unused, but still here
        LogImportance::Debug => {
            file.unwrap()
                .write_all(format!("{} [DEBUG] {}\n", formatted_time, data.message).as_bytes())
                .unwrap();
            println!(
                "{} {} {}",
                formatted_time,
                "[DEBUG]".fg::<Black>().bg::<LightBlue>(),
                data.message
            );
        }
    }
}

pub trait LogExpect<T, E: Debug> {
    fn log_expect(self, importance: LogImportance, msg: &str) -> T;
}

impl<T, E: Debug> LogExpect<T, E> for Result<T, E> {
    fn log_expect(self, importance: LogImportance, msg: &str) -> T {
        match self {
            Ok(val) => val,
            Err(err) => {
                log_this(LogData {
                    importance,
                    message: format!("{}: {:?}", msg, err),
                });
                panic!("{}: {:?}", msg, err);
            }
        }
    }
}

impl<T> LogExpect<T, ()> for Option<T> {
    fn log_expect(self, importance: LogImportance, msg: &str) -> T {
        match self {
            Some(val) => val,
            None => {
                log_this(LogData {
                    importance,
                    message: msg.to_string(),
                });
                panic!("{}", msg);
            }
        }
    }
}
