use std::{
    fs::{self, create_dir_all},
    path::PathBuf,
};

use chrono::SecondsFormat;
use fern::colors::{Color, ColoredLevelConfig};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct LoggingConfiguration {
    pub enable_console: bool,
    pub path: Option<PathBuf>,
    pub root_level: log::Level,
    pub targets: Option<Vec<LoggingTarget>>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct LoggingTarget {
    pub target: String,
    pub level: log::Level,
}

impl LoggingConfiguration {
    pub fn init(&self) {
        let mut fern = fern::Dispatch::new();

        if self.enable_console {
            let colors = ColoredLevelConfig::new()
                .error(Color::BrightRed)
                .warn(Color::BrightYellow)
                .info(Color::BrightGreen)
                .debug(Color::BrightWhite)
                .trace(Color::BrightBlack);

            let console = fern::Dispatch::new()
                .format(move |out, message, record| {
                    out.finish(format_args!(
                        "{color_line}[{date} {level} {target} {color_line}] {message}\x1B[0m",
                        color_line =
                            format_args!("\x1B[{}m", colors.get_color(&record.level()).to_fg_str()),
                        date = chrono::Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true),
                        target = record.target(),
                        level = colors.color(record.level()),
                        message = message,
                    ));
                })
                .chain(std::io::stdout());
            fern = fern.chain(console);
        }

        match &self.path {
            Some(path) => 'filelogging: {
                if !fs::exists(path).unwrap_or(false) {
                    match create_dir_all(path) {
                        Ok(()) => (),
                        Err(error) => {
                            println!(
                                "Unable to create the log path. File logging is disabled.\n\t{error:?}"
                            );
                            break 'filelogging;
                        }
                    }
                }

                let log_file_location =
                    fern::DateBased::new(&path.join("impetus-impresario"), ".%Y-%m-%d.log")
                        .utc_time();

                let file = fern::Dispatch::new()
                    .format(|out, message, record| {
                        out.finish(format_args!(
                            "[{level} {date}] ({target}) {message}",
                            date = chrono::Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true),
                            level = record.level(),
                            target = record.target(),
                            message = message,
                        ))
                    })
                    .chain(log_file_location);

                fern = fern.chain(file);
            }
            None => {}
        };

        match &self.targets {
            Some(targets) => {
                for target in targets {
                    fern = fern.level_for(
                        target.target.clone(),
                        target.level.to_level_filter().clone(),
                    );
                }
            }
            None => {}
        }

        fern.level(self.root_level.to_level_filter())
            .apply()
            .unwrap();
    }
}
