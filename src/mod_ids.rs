// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::{err, Result};

const MIN_LENGTH: usize = 2;
const MAX_LENGTH: usize = 64;

fn is_valid_id_start(c: char) -> bool {
    matches!(c, 'a'..='z')
}

fn is_valid_in_id(c: char) -> bool {
    matches!(c, 'a'..='z' | '0'..='9' | '-' | '_')
}

pub fn validate_mod_id<S: AsRef<str>>(id: S) -> Result<()> {
    let id = id.as_ref();
    if id.len() < MIN_LENGTH || id.len() > MAX_LENGTH {
        return Err(err!(
            "Length must be between {} and {}",
            MIN_LENGTH,
            MAX_LENGTH
        ));
    }

    let mut chars = id.chars();
    let head = chars.next().unwrap();

    if !is_valid_id_start(head) {
        return Err(err!("'{}' is not valid at the start of an ID", head));
    }

    for c in chars {
        if !is_valid_in_id(c) {
            return Err(err!("'{}' is not valid in IDs", c));
        }
    }

    Ok(())
}

pub fn is_valid_mod_id<S: AsRef<str>>(id: S) -> bool {
    validate_mod_id(id).is_ok()
}

pub fn to_mod_id<S: AsRef<str>>(name: S) -> String {
    let mut output = String::new();

    for c in name.as_ref().chars().flat_map(|c| c.to_lowercase()) {
        if output.len() == MAX_LENGTH {
            break;
        }

        if output.is_empty() {
            if is_valid_id_start(c) {
                output.push(c);
            }
        } else if is_valid_in_id(c) {
            output.push(c);
        } else if c == ' ' {
            output.push('_');
        }
    }

    output
}
