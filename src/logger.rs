use crate::constants;
use chrono::Local;
use directories::ProjectDirs;
use lazy_static::lazy_static;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::sync::Mutex;

pub struct Logger {
    file: Mutex<std::fs::File>,
}

impl Logger {
    fn init() -> Self {
        let proj_dirs = ProjectDirs::from(
            constants::QUALIFIER,
            constants::ORGANIZATION,
            constants::APP_NAME,
        )
        .expect("Failed to get application directory");
        
        let log_dir = proj_dirs.data_dir().join("logs");

        if !log_dir.exists() {
            fs::create_dir_all(&log_dir).expect("Failed to create log directory");
        }

        let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
        let log_path = log_dir.join(format!("{}.log", timestamp));

        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_path)
            .expect("Failed to create log file");

        Logger {
            file: Mutex::new(file),
        }
    }

    pub fn log(message: &str) {
        lazy_static! {
            static ref INSTANCE: Logger = Logger::init();
        }

        let now = Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
        let log_line = format!("{} - {}\n", now, message);

        print!("{}", log_line);

        let mut file = INSTANCE.file.lock().unwrap();

        file.write_all(log_line.as_bytes())
            .expect("Failed to write to log file");
    }
}
