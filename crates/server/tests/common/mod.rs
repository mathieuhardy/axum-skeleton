#[allow(dead_code)]
pub const ADMIN_EMAIL: &str = "giga@chad.com";

#[allow(dead_code)]
pub const ADMIN_PASSWORD: &str = "Z0*zZZZZ";

#[allow(dead_code)]
pub const INVALID_EMAIL: &str = "invalid.com";

#[allow(dead_code)]
pub const INVALID_PASSWORD: &str = "invalid";

#[allow(dead_code)]
#[derive(Clone)]
pub enum DataType {
    Form,
    Json,
}

#[allow(dead_code)]
pub enum EmailValidity {
    Invalid,
    Valid,
}

#[allow(dead_code)]
pub enum FirstNameValidity {
    Invalid,
    Valid,
}

#[allow(dead_code)]
pub enum LastNameValidity {
    Invalid,
    Valid,
}

#[allow(dead_code)]
pub enum PasswordValidity {
    Invalid,
    Valid,
}
