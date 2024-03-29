pub const USERS_COUNT: usize = 2;

// Admin user section
pub const ADMIN_FIRST_NAME: &str = "Giga";
pub const ADMIN_LAST_NAME: &str = "Chad";
pub const ADMIN_EMAIL: &str = "giga@chad.com";
pub const ADMIN_PASSWORD: &str = "Z0*zZZZZ";

// Basic user section
pub const USER_EMAIL: &str = "john@doe.com";

// Invalid entries
pub const INVALID_EMAIL: &str = "invalid.com";
pub const INVALID_PASSWORD: &str = "invalid";

#[derive(Clone)]
pub enum DataType {
    Form,
    Json,
}

pub enum EmailValidity {
    Invalid,
    Valid,
}

pub enum FirstNameValidity {
    Invalid,
    Valid,
}

pub enum LastNameValidity {
    Invalid,
    Valid,
}

pub enum PasswordValidity {
    Invalid,
    Valid,
}
