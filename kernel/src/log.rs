use arrayvec::ArrayString;
use lazy_static::lazy_static;
use spin::Mutex;
use core::fmt::{Write, Arguments};
use core::fmt;
use crate::limine;

type GlobalLog = StaticLog;

lazy_static! {
    static ref GLOBAL_LOG: Mutex<GlobalLog> = Mutex::new(GlobalLog::new());
}

pub fn print(msg: Arguments) {
    GLOBAL_LOG.lock().write_fmt(msg).expect("Could not write to GLOBAL_LOG!")
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => ($crate::log::print(format_args!($($arg)*)));
}

// Static Log implementation

const STATIC_LOG_MAX_CHARACTERS: usize = 65535;

struct StaticLog {
    content: ArrayString<STATIC_LOG_MAX_CHARACTERS>,
}

impl StaticLog {
    fn new() -> Self {
        Self {
            content: ArrayString::<STATIC_LOG_MAX_CHARACTERS>::new()
        }
    }
}

impl Write for StaticLog {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        if s.len() > self.content.remaining_capacity() {
            return Err(fmt::Error);
        }
        limine::print_bytes(s.as_bytes());
        self.content.push_str(s);
        Ok(())
    }
} 