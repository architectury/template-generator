// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

pub mod filer;

use std::collections::{HashMap, HashSet};

pub enum TemplatePart {
    Line(String),
    Conditional {
        condition: String,
        inner: Box<Vec<TemplatePart>>,
    },
}

struct Frame {
    parts: Vec<TemplatePart>,
    condition: Option<String>,
}

pub fn read_template<S: AsRef<str>>(input: S) -> Result<Vec<TemplatePart>, String> {
    let input = input.as_ref();
    let mut frames = vec![Frame {
        parts: vec![],
        condition: None,
    }];
    for (line_number, line) in input.lines().enumerate() {
        if line.starts_with("//%") || line.starts_with("#%") {
            let parts: Vec<&str> = line.split_whitespace().skip(1).collect();
            match parts[0] {
                "if" => frames.push(Frame {
                    parts: vec![],
                    condition: Some(parts[1].to_owned()),
                }),
                "end" => {
                    if frames.len() <= 1 {
                        return Err("Cannot end if block at top level".to_owned());
                    }
                    let frame = frames.pop().unwrap();
                    frames
                        .last_mut()
                        .unwrap()
                        .parts
                        .push(TemplatePart::Conditional {
                            condition: frame.condition.unwrap(),
                            inner: Box::new(frame.parts),
                        })
                }
                _ => {
                    return Err(format!(
                        "Unknown template command {} on line {}",
                        parts[1],
                        line_number + 1
                    ))
                }
            }
        } else {
            frames
                .last_mut()
                .unwrap()
                .parts
                .push(TemplatePart::Line(line.to_owned()));
        }
    }

    Ok(frames.pop().unwrap().parts)
}

pub struct Context {
    variables: HashMap<String, String>,
    flags: HashSet<String>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            flags: HashSet::new(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, &str)> {
        self.variables.iter().map(|(k, v)| (k.as_str(), v.as_str()))
    }

    pub fn has<K: AsRef<str>>(&self, key: K) -> bool {
        self.flags.contains(&key.as_ref().to_owned())
    }

    pub fn put<K, V>(&mut self, key: K, value: V)
    where
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.variables
            .insert(key.as_ref().to_owned(), value.as_ref().to_owned());
    }

    pub fn define<F: AsRef<str>>(&mut self, flag: F) {
        self.flags.insert(flag.as_ref().to_owned());
    }
}

pub fn apply_variables(context: &Context, text: &str, use_delimiters: bool) -> String {
    let mut text = String::from(text);

    for (key, value) in context.iter() {
        let to_replace = if use_delimiters {
            format!("%{}%", key)
        } else {
            key.to_owned()
        };
        text = text.replace(&to_replace, value);
    }

    text
}

pub fn apply_template(context: &Context, parts: Vec<TemplatePart>) -> Vec<String> {
    let mut output: Vec<String> = vec![];

    for part in parts {
        match part {
            TemplatePart::Line(text) => output.push(apply_variables(context, text.as_str(), true)),
            TemplatePart::Conditional { condition, inner } => {
                if context.has(&condition) {
                    output.append(&mut apply_template(context, *inner))
                }
            }
        }
    }

    output
}
