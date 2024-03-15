use std::sync::{Arc, Mutex};

use crate::prelude::*;

lazy_static::lazy_static! {
    static ref PASSWORD_PATTERN: Arc<Mutex<String>> = Arc::new(Mutex::new("".to_string()));
}

pub fn set_pattern(pattern: String) {
    if let Ok(mut value) = PASSWORD_PATTERN.lock() {
        *value = pattern;

        event!(Level::INFO, "✱ Password pattern created");
        event!(Level::TRACE, "{}", *value);
    } else {
        event!(Level::ERROR, "Cannot apply password pattern");
    }
}

pub fn pattern() -> Res<String> {
    if let Ok(value) = PASSWORD_PATTERN.lock() {
        Ok((*value).clone())
    } else {
        Err(Error::PasswordPatternAccess)
    }
}
