pub fn is_camel(s: &str) -> bool {
    match s.chars().nth(0) {
        Some(c) => {
            if !c.is_ascii_lowercase() {
                return false;
            }
        }
        None => return false,
    };

    for c in s.chars() {
        if !(c.is_ascii_lowercase() || c.is_ascii_uppercase()) {
            return false;
        }
    }
    true
}

pub fn is_pascal(s: &str) -> bool {
    match s.chars().nth(0) {
        Some(c) => {
            if !c.is_ascii_uppercase() {
                return false;
            }
        }
        None => return false,
    };

    for c in s.chars() {
        if !(c.is_ascii_lowercase() || c.is_ascii_uppercase()) {
            return false;
        }
    }
    true
}

pub fn is_screaming_snake(s: &str) -> bool {
    match s.chars().nth(0) {
        Some(c) => {
            if !c.is_ascii_uppercase() {
                return false;
            }
        }
        None => return false,
    };

    match s.chars().nth_back(0) {
        Some(c) => {
            if !c.is_ascii_uppercase() {
                return false;
            }
        }
        None => return false,
    };

    for c in s.chars() {
        if !(c == '_' || c.is_ascii_uppercase()) {
            return false;
        }
    }
    true
}

pub fn is_uppercase(s: &str) -> bool {
    for c in s.chars() {
        if !c.is_ascii_uppercase() {
            return false;
        }
    }
    true
}

pub fn is_kebab_case(s: &str) -> bool {
    match s.chars().nth_back(0) {
        Some(c) => {
            if !c.is_ascii_lowercase() {
                return false;
            }
        }
        None => return false,
    };

    match s.chars().nth_back(0) {
        Some(c) => {
            if !c.is_ascii_lowercase() {
                return false;
            }
        }
        None => return false,
    };

    for c in s.chars() {
        if !(c == '-' || c.is_ascii_lowercase()) {
            return false;
        }
    }
    true
}

pub fn is_lowercase(s: &str) -> bool {
    for c in s.chars() {
        if !c.is_ascii_lowercase() {
            return false;
        }
    }
    true
}

pub fn is_snake_case(s: &str) -> bool {
    match s.chars().nth_back(0) {
        Some(c) => {
            if !c.is_ascii_lowercase() {
                return false;
            }
        }
        None => return false,
    };

    match s.chars().nth_back(0) {
        Some(c) => {
            if !c.is_ascii_lowercase() {
                return false;
            }
        }
        None => return false,
    };

    for c in s.chars() {
        if !(c == '_' || c.is_ascii_lowercase()) {
            return false;
        }
    }
    true
}
