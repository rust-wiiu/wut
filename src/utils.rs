#![allow(dead_code)]

// This file contains useful helper stuff
// Only keep private stuff here
// Having public stuff crammed into "utils" in not good design

use alloc::{string::String, vec::Vec};

pub(crate) fn text_wrap<T: AsRef<str>>(text: T, width: usize) -> String {
    let text = text.as_ref();
    let mut result = String::new();
    let mut line = String::new();

    for word in text.split_whitespace() {
        if (line.len() + word.len() + 1) <= width {
            line.push_str(word);
            line.push(' ');
        } else {
            result.push_str(line.trim_end());
            result.push('\n');
            line = String::from(word);
            line.push(' ');
        }
    }

    if !line.is_empty() {
        result.push_str(line.trim_end());
    }

    result
}

pub(crate) fn into_utf16(s: &str) -> Vec<u16> {
    let mut v: Vec<u16> = s.encode_utf16().collect();
    v.push(0);
    v
}

pub(crate) fn from_utf16(ptr: *const u16) -> String {
    let len = (0..)
        .take_while(|&i| unsafe { *ptr.offset(i) } != 0)
        .count()
        .min(256);
    let v = unsafe { core::slice::from_raw_parts(ptr, len) };

    String::from_utf16_lossy(v)
}
