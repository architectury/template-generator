const MIN_LENGTH: usize = 2;
const MAX_LENGTH: usize = 64;

fn is_valid_id_start(c: char) -> bool {
    matches!(c, 'a'..='z')
}

fn is_valid_in_id(c: char) -> bool {
    matches!(c, 'a'..='z' | '0'..='9' | '-' | '_')
}

pub fn is_valid_mod_id<S: AsRef<str>>(id: S) -> bool {
    let id = id.as_ref();
    if id.len() < MIN_LENGTH || id.len() > MAX_LENGTH {
        return false;
    }

    let mut chars = id.chars();
    is_valid_id_start(chars.next().unwrap()) && chars.all(|c| is_valid_in_id(c))
}

pub fn to_mod_id<S: AsRef<str>>(name: S) -> Option<String> {
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

    if is_valid_mod_id(&output) {
        Some(output)
    } else {
        None
    }
}
