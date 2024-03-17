#[derive(Clone, Debug, Default)]
pub struct Checks {
    pub digit: bool,
    pub lowercase: bool,
    pub uppercase: bool,
    pub special: bool,
    pub spaces: bool,
    pub min_length: u32,
    pub max_length: Option<u32>,
}

impl Checks {
    pub fn is_ok(&self) -> bool {
        !self.digit && !self.lowercase && !self.uppercase && !self.special && !self.spaces
    }
}

// TODO: document
pub fn verify(password: &str, mut checks: Checks) -> bool {
    let length = password.len() as u32;

    if length < checks.min_length {
        return false;
    }

    if let Some(max_length) = checks.max_length {
        if length > max_length {
            return false;
        }
    }

    let expect_spaces = checks.spaces;

    for c in password.chars() {
        if checks.digit && c.is_numeric() {
            checks.digit = false;
        } else if checks.lowercase && c.is_lowercase() {
            checks.lowercase = false;
        } else if checks.uppercase && c.is_uppercase() {
            checks.uppercase = false;
        } else if checks.special && !c.is_alphanumeric() {
            checks.special = false;
        } else if expect_spaces && c.is_whitespace() {
            checks.spaces = false;
        } else if !expect_spaces && c.is_whitespace() {
            checks.spaces = true;
        }
    }

    checks.is_ok()
}
