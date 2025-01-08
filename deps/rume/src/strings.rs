#[derive(Debug, PartialEq)]
pub enum SplitBehavior {
    KeepToken,
    SkipToken,
}

pub fn split(str: &str, delim: &str, behavior_opt: Option<SplitBehavior>) -> Vec<String> {
    let mut strings = Vec::new();

    let behavior = behavior_opt.unwrap_or(SplitBehavior::KeepToken);
    let str_len = str.len();

    let mut last_pos: usize = if behavior == SplitBehavior::SkipToken {
        str.find(|c| !delim.contains(c)).unwrap_or(str_len)
    } else {
        0
    };

    let mut pos: usize = if let Some(new_pos) = str[last_pos..].find(delim) {
        last_pos + new_pos
    } else {
        str_len
    };

    while pos != str_len || last_pos != str_len {
        strings.push(str[last_pos..pos].to_string());
        if behavior == SplitBehavior::SkipToken {
            last_pos = if let Some(new_pos) = str[pos..].find(|c| !delim.contains(c)) {
                pos + new_pos
            } else {
                str_len
            };
        } else {
            if pos == str_len {
                break;
            }
            last_pos = pos + 1;
        }

        pos = if let Some(new_pos) = str[last_pos..].find(delim) {
            last_pos + new_pos
        } else {
            str_len
        };
    }

    strings
}
