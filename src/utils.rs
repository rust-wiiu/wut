// This file contains useful helper stuff
// Only keep private stuff here
// Having public stuff crammed into "utils" in not good design

use alloc::string::String;

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
