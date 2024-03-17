use std::sync::{Arc, Mutex};

use utils::password::Checks;

use crate::prelude::*;

lazy_static::lazy_static! {
    static ref PASSWORD_CHECKS: Arc<Mutex<Checks>> = Arc::new(Mutex::new(Checks::default()));
}

pub fn set_checks(checks: Checks) {
    if let Ok(mut value) = PASSWORD_CHECKS.lock() {
        *value = checks;

        event!(Level::INFO, "✱ Password checks created");
        event!(Level::TRACE, "{:#?}", *value);
    } else {
        event!(Level::ERROR, "Cannot apply password checks");
    }
}

pub fn checks() -> Res<Checks> {
    if let Ok(value) = PASSWORD_CHECKS.lock() {
        Ok((*value).clone())
    } else {
        Err(Error::PasswordChecksAccess)
    }
}
