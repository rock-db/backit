use std::fmt::Display;

use crate::failed;

pub trait ResultExt<T, E> {
    fn unwrap_or_fail(self, title: &str, msg: &str) -> T;
    fn unwrap_or_fail_with(self, title: &str, msg: &str) -> T;
}

impl<T, E: Display> ResultExt<T, E> for Result<T, E> {
    fn unwrap_or_fail(self, title: &str, msg: &str) -> T {
        match self {
            Ok(value) => value,
            Err(_) => {
                failed!(title, "{}", msg);
            }
        }
    }

    fn unwrap_or_fail_with(self, title: &str, msg: &str) -> T {
        match self {
            Ok(value) => value,
            Err(e) => {
                failed!(title, "{}: {}", msg, e);
            }
        }
    }
}
