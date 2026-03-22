#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CheckboxKind {
    Bulleted,
    Plain,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CheckboxState {
    Unchecked,
    Checked,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct CheckboxPrefix {
    indent_len: usize,
    marker_len: usize,
    kind: CheckboxKind,
    state: CheckboxState,
}

fn split_line_and_eol(line: &str) -> (&str, &str) {
    if let Some(content) = line.strip_suffix("\r\n") {
        (content, "\r\n")
    } else if let Some(content) = line.strip_suffix('\n') {
        (content, "\n")
    } else {
        (line, "")
    }
}

fn leading_indent_len(s: &str) -> usize {
    let bytes = s.as_bytes();
    let mut indent_len = 0usize;
    while indent_len < bytes.len() && (bytes[indent_len] == b' ' || bytes[indent_len] == b'\t') {
        indent_len += 1;
    }
    indent_len
}

fn checkbox_prefix(line: &str) -> Option<CheckboxPrefix> {
    let (content, _) = split_line_and_eol(line);
    let indent_len = leading_indent_len(content);
    let rest = &content[indent_len..];
    let (kind, state, marker_len) = if rest.starts_with("- [ ] ") {
        (
            CheckboxKind::Bulleted,
            CheckboxState::Unchecked,
            "- [ ] ".len(),
        )
    } else if rest.starts_with("- [x] ") {
        (
            CheckboxKind::Bulleted,
            CheckboxState::Checked,
            "- [x] ".len(),
        )
    } else if rest.starts_with("[ ] ") {
        (CheckboxKind::Plain, CheckboxState::Unchecked, "[ ] ".len())
    } else if rest.starts_with("[x] ") {
        (CheckboxKind::Plain, CheckboxState::Checked, "[x] ".len())
    } else {
        return None;
    };

    Some(CheckboxPrefix {
        indent_len,
        marker_len,
        kind,
        state,
    })
}

pub fn has_checkbox_prefix(line: &str) -> bool {
    checkbox_prefix(line).is_some()
}

pub fn toggle_checkbox_line(line: &str) -> Option<String> {
    let prefix = checkbox_prefix(line)?;
    let (content, eol) = split_line_and_eol(line);
    let replacement = match (prefix.kind, prefix.state) {
        (CheckboxKind::Bulleted, CheckboxState::Unchecked) => "- [x] ",
        (CheckboxKind::Bulleted, CheckboxState::Checked) => "- [ ] ",
        (CheckboxKind::Plain, CheckboxState::Unchecked) => "[x] ",
        (CheckboxKind::Plain, CheckboxState::Checked) => "[ ] ",
    };

    let mut out = String::with_capacity(line.len());
    out.push_str(&content[..prefix.indent_len]);
    out.push_str(replacement);
    out.push_str(&content[prefix.indent_len + prefix.marker_len..]);
    out.push_str(eol);
    Some(out)
}

pub fn insert_checkbox_line(line: &str) -> Option<String> {
    if has_checkbox_prefix(line) {
        return None;
    }

    let (content, eol) = split_line_and_eol(line);
    let indent_len = leading_indent_len(content);
    let rest = &content[indent_len..];
    let (marker, suffix) = if let Some(after_bullet) = rest.strip_prefix("- ") {
        ("- [ ] ", after_bullet)
    } else {
        ("[ ] ", rest)
    };

    let mut out = String::with_capacity(line.len() + marker.len());
    out.push_str(&content[..indent_len]);
    out.push_str(marker);
    out.push_str(suffix);
    out.push_str(eol);
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_checkbox_prefix_with_indentation() {
        assert!(has_checkbox_prefix("  - [ ] task"));
        assert!(has_checkbox_prefix("\t[x] task"));
        assert!(!has_checkbox_prefix("  - task"));
    }

    #[test]
    fn toggles_bulleted_checkbox() {
        assert_eq!(
            toggle_checkbox_line("- [ ] ship it"),
            Some(String::from("- [x] ship it"))
        );
        assert_eq!(
            toggle_checkbox_line("- [x] ship it"),
            Some(String::from("- [ ] ship it"))
        );
    }

    #[test]
    fn toggles_plain_checkbox() {
        assert_eq!(
            toggle_checkbox_line("[ ] ship it"),
            Some(String::from("[x] ship it"))
        );
        assert_eq!(
            toggle_checkbox_line("[x] ship it"),
            Some(String::from("[ ] ship it"))
        );
    }

    #[test]
    fn toggle_preserves_indentation_and_eol() {
        assert_eq!(
            toggle_checkbox_line("\t- [ ] ship it\r\n"),
            Some(String::from("\t- [x] ship it\r\n"))
        );
    }

    #[test]
    fn toggle_ignores_non_checkbox_lines() {
        assert_eq!(toggle_checkbox_line("plain text"), None);
        assert_eq!(toggle_checkbox_line("- task"), None);
    }

    #[test]
    fn insert_checkbox_preserves_indentation() {
        assert_eq!(
            insert_checkbox_line("  ship it"),
            Some(String::from("  [ ] ship it"))
        );
    }

    #[test]
    fn insert_checkbox_converts_plain_bullet_to_task_bullet() {
        assert_eq!(
            insert_checkbox_line("\t- ship it"),
            Some(String::from("\t- [ ] ship it"))
        );
    }

    #[test]
    fn insert_checkbox_on_empty_line() {
        assert_eq!(insert_checkbox_line(""), Some(String::from("[ ] ")));
    }

    #[test]
    fn insert_checkbox_preserves_eol() {
        assert_eq!(
            insert_checkbox_line("  ship it\n"),
            Some(String::from("  [ ] ship it\n"))
        );
    }

    #[test]
    fn insert_checkbox_skips_existing_checkbox_lines() {
        assert_eq!(insert_checkbox_line("  [ ] ship it"), None);
        assert_eq!(insert_checkbox_line("- [x] ship it"), None);
    }
}
