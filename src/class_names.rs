use crate::{err, Result};

pub fn sanitize_class_name(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut prev_allowed = false;
    let mut started = false;

    for c in s.trim().chars() {
        if !(c.is_alphanumeric() || c == '_') {
            prev_allowed = false;
            continue;
        }

        if !started {
            if c.is_ascii_alphabetic() {
                result.push(c.to_ascii_uppercase());
                started = true;
                prev_allowed = true;
            }
            continue;
        }

        let c = if !prev_allowed && c.is_ascii_alphabetic() {
            c.to_ascii_uppercase()
        } else {
            c
        };

        result.push(c);
        prev_allowed = true;
    }

    result
}

pub fn validate_name<S: AsRef<str>>(name: S) -> Result<()> {
    let name = name.as_ref();

    if name.is_empty() {
        return Err(err!("Name cannot be empty"));
    }

    let mut chars = name.chars();
    if !chars.next().unwrap().is_ascii_alphabetic() {
        return Err(err!("Java class name must start with a letter"));
    }

    for c in chars {
        if !(c.is_ascii_alphanumeric() || c == '_') {
            return Err(err!("'{}' is not valid in a Java class name", c));
        }
    }

    Ok(())
}

pub fn is_valid_main_class_name<S: AsRef<str>>(id: S) -> bool {
    validate_name(id).is_ok()
}
