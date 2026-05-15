use std::ops::Range;

pub const SEARCH_PATTERN: &str = r"~~[^~\r\n]+~~";

// `MatchSet` / `find_inner_ranges` are exercised by the tests below and reserved
// for the pure-Rust scan path; the editor currently relies on Scintilla's regex
// search via `SEARCH_PATTERN`. Keep the API available without tripping dead_code.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchSet {
    pub inner_ranges: Vec<Range<usize>>,
    pub truncated: bool,
}

#[allow(dead_code)]
pub fn find_inner_ranges(text: &str, max_matches: usize) -> MatchSet {
    if max_matches == 0 {
        return MatchSet {
            inner_ranges: Vec::new(),
            truncated: false,
        };
    }

    let bytes = text.as_bytes();
    let mut inner_ranges = Vec::new();
    let mut pos = 0usize;

    while pos + 3 < bytes.len() {
        if bytes[pos] == b'~' && bytes[pos + 1] == b'~' {
            let inner_start = pos + 2;
            let mut inner_end = inner_start;
            while inner_end < bytes.len()
                && bytes[inner_end] != b'~'
                && bytes[inner_end] != b'\r'
                && bytes[inner_end] != b'\n'
            {
                inner_end += 1;
            }

            if inner_end > inner_start
                && inner_end + 1 < bytes.len()
                && bytes[inner_end] == b'~'
                && bytes[inner_end + 1] == b'~'
            {
                inner_ranges.push(inner_start..inner_end);
                if inner_ranges.len() >= max_matches {
                    return MatchSet {
                        inner_ranges,
                        truncated: true,
                    };
                }
                pos = inner_end + 2;
                continue;
            }
        }

        pos += 1;
    }

    MatchSet {
        inner_ranges,
        truncated: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_single_char_inner_range() {
        let ranges = find_inner_ranges("~~a~~", 10);
        assert_eq!(ranges.inner_ranges, vec![2..3]);
        assert!(!ranges.truncated);
    }

    #[test]
    fn matches_inner_text_in_sentence() {
        let ranges = find_inner_ranges("x ~~abc~~ y", 10);
        assert_eq!(ranges.inner_ranges, vec![4..7]);
    }

    #[test]
    fn does_not_match_across_newlines() {
        let ranges = find_inner_ranges("~~a\nb~~", 10);
        assert!(ranges.inner_ranges.is_empty());
    }

    #[test]
    fn does_not_match_empty_inner_text() {
        let ranges = find_inner_ranges("~~~~", 10);
        assert!(ranges.inner_ranges.is_empty());
    }

    #[test]
    fn reports_truncation_at_match_cap() {
        let ranges = find_inner_ranges("~~a~~ ~~b~~", 1);
        assert_eq!(ranges.inner_ranges, vec![2..3]);
        assert!(ranges.truncated);
    }
}
