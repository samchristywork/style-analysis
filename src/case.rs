// camelCase
pub fn is_camel(s: &str) -> bool {
    // First character should be lowercase
    match s.chars().nth(0) {
        Some(c) => {
            if !c.is_ascii_lowercase() {
                return false;
            }
        }
        None => return false,
    };

    // All other characters should be uppercase or lowercase
    for c in s.chars() {
        if !(c.is_ascii_lowercase() || c.is_ascii_uppercase()) {
            return false;
        }
    }
    true
}

// PascalCase
pub fn is_pascal(s: &str) -> bool {
    // First character should be uppercase
    match s.chars().nth(0) {
        Some(c) => {
            if !c.is_ascii_uppercase() {
                return false;
            }
        }
        None => return false,
    };

    // All other characters should be uppercase or lowercase
    for c in s.chars() {
        if !(c.is_ascii_lowercase() || c.is_ascii_uppercase()) {
            return false;
        }
    }
    true
}

// SCREAMING_SNAKE
pub fn is_screaming_snake(s: &str) -> bool {
    // First character should be uppercase
    match s.chars().nth(0) {
        Some(c) => {
            if !c.is_ascii_uppercase() {
                return false;
            }
        }
        None => return false,
    };

    // Last character should be uppercase
    match s.chars().nth_back(0) {
        Some(c) => {
            if !c.is_ascii_uppercase() {
                return false;
            }
        }
        None => return false,
    };

    // All other characters should be uppercase or "_"
    for c in s.chars() {
        if !(c == '_' || c.is_ascii_uppercase()) {
            return false;
        }
    }
    true
}

// UPPERCASE
pub fn is_uppercase(s: &str) -> bool {
    // All characters should be uppercase
    for c in s.chars() {
        if !c.is_ascii_uppercase() {
            return false;
        }
    }
    true
}

// kebab-case
pub fn is_kebab_case(s: &str) -> bool {
    // First character should be lowercase
    match s.chars().nth_back(0) {
        Some(c) => {
            if !c.is_ascii_lowercase() {
                return false;
            }
        }
        None => return false,
    };

    // Last character should be lowercase
    match s.chars().nth_back(0) {
        Some(c) => {
            if !c.is_ascii_lowercase() {
                return false;
            }
        }
        None => return false,
    };

    // All other characters should be lowercase or "-"
    for c in s.chars() {
        if !(c == '-' || c.is_ascii_lowercase()) {
            return false;
        }
    }
    true
}

// lowercase
pub fn is_lowercase(s: &str) -> bool {
    // All characters should be lowercase
    for c in s.chars() {
        if !c.is_ascii_lowercase() {
            return false;
        }
    }
    true
}

// snake_case
pub fn is_snake_case(s: &str) -> bool {
    // First character should be lowercase
    match s.chars().nth_back(0) {
        Some(c) => {
            if !c.is_ascii_lowercase() {
                return false;
            }
        }
        None => return false,
    };

    // Last character should be lowercase
    match s.chars().nth_back(0) {
        Some(c) => {
            if !c.is_ascii_lowercase() {
                return false;
            }
        }
        None => return false,
    };

    // All other characters should be lowercase or "_"
    for c in s.chars() {
        if !(c == '_' || c.is_ascii_lowercase()) {
            return false;
        }
    }
    true
}
