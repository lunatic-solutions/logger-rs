use std::fmt::Write;

use ansi_term::Color;
use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::{level::LevelFilter, Event, Level, Metadata};

use super::Subscriber;

const GRAY: Color = Color::Black;

#[derive(Serialize, Deserialize)]
pub struct FmtSubscriber {
    color: bool,
    file: bool,
    level: bool,
    level_filter: LevelFilter,
    line_number: bool,
    target: bool,
    time: bool,
    time_format: Option<String>,
}

impl Default for FmtSubscriber {
    fn default() -> Self {
        Self {
            color: false,
            file: false,
            level: false,
            level_filter: LevelFilter::Off,
            line_number: false,
            target: false,
            time: false,
            time_format: None,
        }
    }
}

impl FmtSubscriber {
    pub fn new(level_filter: LevelFilter) -> Self {
        FmtSubscriber {
            level_filter,
            ..Default::default()
        }
    }

    pub fn pretty(mut self) -> Self {
        self.color = true;
        self.file = true;
        self.level = true;
        self.line_number = true;
        self.target = true;
        self.time = true;
        self
    }

    pub fn with_color(mut self, color: bool) -> Self {
        self.color = color;
        self
    }

    pub fn with_file(mut self, file: bool) -> Self {
        self.file = file;
        self
    }

    pub fn with_level(mut self, level: bool) -> Self {
        self.level = level;
        self
    }

    pub fn with_line_number(mut self, line_number: bool) -> Self {
        self.line_number = line_number;
        self
    }

    pub fn with_target(mut self, target: bool) -> Self {
        self.target = target;
        self
    }

    pub fn with_time(mut self, time: bool) -> Self {
        self.time = time;
        self
    }

    pub fn with_time_format(mut self, time_format: impl Into<String>) -> Self {
        self.time_format = Some(time_format.into());
        self
    }
}

impl Subscriber for FmtSubscriber {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= &self.level_filter
    }

    fn event(&self, event: &Event) {
        let mut line = String::new();
        if self.time {
            let now = Utc::now();
            let now_string = now
                .format(
                    self.time_format
                        .as_deref()
                        .unwrap_or("%Y-%m-%dT%H:%M:%S%.6fZ"),
                )
                .to_string();
            if self.color {
                write!(line, "{} ", GRAY.paint(now_string)).unwrap();
            } else {
                write!(line, "{now_string} ").unwrap();
            }
        }
        if self.level {
            if self.color {
                let level_string = match event.metadata().level() {
                    Level::Error => Color::Red,
                    Level::Warn => Color::Yellow,
                    Level::Info => Color::Green,
                    Level::Debug => Color::Blue,
                    Level::Trace => Color::Purple,
                }
                .paint(event.metadata().level().as_str());
                for _ in 0..(5 - event.metadata().level().as_str().len()) {
                    line.push(' ');
                }
                write!(line, "{level_string} ").unwrap();
            } else {
                let level_string = event.metadata().level().as_str();
                for _ in 0..(5 - event.metadata().level().as_str().len()) {
                    line.push(' ');
                }
                write!(line, "{level_string} ").unwrap();
            };
        }
        if self.target {
            if self.color {
                write!(
                    line,
                    "{}",
                    GRAY.paint(format!("{}: ", event.metadata().target()))
                )
                .unwrap();
            } else {
                write!(line, "{}: ", event.metadata().target()).unwrap();
            }
        }
        if self.file {
            if let Some(file) = event.metadata().file() {
                if self.color {
                    write!(line, "{}", GRAY.paint(format!("{file}:"))).unwrap();
                } else {
                    write!(line, "{}:", file).unwrap();
                }
            }
        }
        if self.line_number {
            if let Some(line_number) = event.metadata().line() {
                if self.color {
                    write!(line, "{}", GRAY.paint(format!("{line_number}: "))).unwrap();
                } else {
                    write!(line, "{}: ", line_number).unwrap();
                }
            }
        } else {
            line.push(' ');
        }
        println!("{line}{}", event.message());
    }
}
