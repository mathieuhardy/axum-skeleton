use database::models::users::{User, UserRole};

pub const USERS_COUNT: usize = 3;

pub const VALID_PASSWORD: &str = "Z0*zZZZZ";

// Admin user section
pub const ADMIN_FIRST_NAME: &str = "Giga";
pub const ADMIN_LAST_NAME: &str = "Chad";
pub const ADMIN_EMAIL: &str = "giga@chad.com";
pub const ADMIN_PASSWORD: &str = VALID_PASSWORD;

// Normal user section
pub const NORMAL_FIRST_NAME: &str = "Giga";
pub const NORMAL_LAST_NAME: &str = "Chad";
pub const NORMAL_EMAIL: &str = "john@doe.com";
pub const NORMAL_PASSWORD: &str = VALID_PASSWORD;

// Guest user section
pub const GUEST_FIRST_NAME: &str = "Pae";
pub const GUEST_LAST_NAME: &str = "Sano";
pub const GUEST_EMAIL: &str = "pae@sano.com";
pub const GUEST_PASSWORD: &str = VALID_PASSWORD;

// Invalid entries
pub const INVALID_EMAIL: &str = "invalid.com";
pub const INVALID_PASSWORD: &str = "invalid";

#[derive(Clone)]
pub enum DataType {
    Form,
    Json,
}

#[derive(Default)]
pub enum EmailValidity {
    Invalid,
    #[default]
    Valid,
}

#[derive(Default)]
pub enum FirstNameValidity {
    Invalid,
    #[default]
    Valid,
}

#[derive(Default)]
pub enum LastNameValidity {
    Invalid,
    #[default]
    Valid,
}

#[derive(Default)]
pub enum PasswordValidity {
    Invalid,
    #[default]
    Valid,
}

pub fn get_user_info(role: UserRole) -> Option<User> {
    match role {
        UserRole::Admin => Some(User {
            first_name: ADMIN_FIRST_NAME.to_string(),
            last_name: ADMIN_LAST_NAME.to_string(),
            email: ADMIN_EMAIL.to_string(),
            password: ADMIN_PASSWORD.to_string(),
            ..User::default()
        }),

        UserRole::Normal => Some(User {
            first_name: NORMAL_FIRST_NAME.to_string(),
            last_name: NORMAL_LAST_NAME.to_string(),
            email: NORMAL_EMAIL.to_string(),
            password: NORMAL_PASSWORD.to_string(),
            ..User::default()
        }),

        UserRole::Guest => Some(User {
            first_name: GUEST_FIRST_NAME.to_string(),
            last_name: GUEST_LAST_NAME.to_string(),
            email: GUEST_EMAIL.to_string(),
            password: GUEST_PASSWORD.to_string(),
            ..User::default()
        }),
    }
}
