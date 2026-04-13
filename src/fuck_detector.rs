use std::collections::BTreeMap;

use aho_corasick::{AhoCorasick, AhoCorasickBuilder, MatchKind};
use thiserror::Error;

const BUNDLED_LEXICON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/data/profanity_lexicon.txt"
));

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfanityEntry {
    pub code: i64,
    pub text: String,
}

#[derive(Debug)]
pub struct FuckDetector {
    matcher: AhoCorasick,
    entries: Vec<ProfanityEntry>,
    match_rules: Vec<MatchRule>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MatchRule {
    None,
    AsciiTokenBoundary,
    SingleHanBoundary,
}

impl FuckDetector {
    pub fn new() -> Result<Self, FuckDetectorError> {
        Self::from_lexicon(BUNDLED_LEXICON)
    }

    pub fn from_lexicon(lexicon: &str) -> Result<Self, FuckDetectorError> {
        let mut entries = Vec::new();
        let mut match_rules = Vec::new();

        for (index, line) in lexicon.lines().enumerate() {
            let line_number = index + 1;
            if line.is_empty() {
                return Err(FuckDetectorError::InvalidLexiconLine { line: line_number });
            }

            let text = line.to_owned();
            match_rules.push(match_rule_for(&text));
            entries.push(ProfanityEntry {
                code: line_number as i64,
                text,
            });
        }

        let patterns = entries
            .iter()
            .map(|entry| entry.text.as_str())
            .collect::<Vec<_>>();
        let matcher = AhoCorasickBuilder::new()
            .ascii_case_insensitive(true)
            .match_kind(MatchKind::LeftmostLongest)
            .build(patterns)
            .map_err(FuckDetectorError::Matcher)?;

        Ok(Self {
            matcher,
            entries,
            match_rules,
        })
    }

    pub fn entries(&self) -> &[ProfanityEntry] {
        &self.entries
    }

    pub fn detect(&self, text: &str) -> BTreeMap<String, i64> {
        let mut counts = BTreeMap::new();

        for found in self.matcher.find_iter(text) {
            let index = found.pattern().as_usize();

            if !matches_rule(self.match_rules[index], text, found.start(), found.end()) {
                continue;
            }

            let entry = &self.entries[index];
            *counts.entry(entry.text.clone()).or_insert(0) += 1;
        }

        counts
    }
}

fn match_rule_for(text: &str) -> MatchRule {
    if text
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || ch == '\'' || ch == ' ' || ch == '-')
    {
        return MatchRule::AsciiTokenBoundary;
    }

    let mut chars = text.chars();
    match (chars.next(), chars.next()) {
        (Some(ch), None) if is_han_character(ch) => MatchRule::SingleHanBoundary,
        _ => MatchRule::None,
    }
}

fn matches_rule(rule: MatchRule, text: &str, start: usize, end: usize) -> bool {
    match rule {
        MatchRule::None => true,
        MatchRule::AsciiTokenBoundary => has_ascii_token_boundary(text, start, end),
        MatchRule::SingleHanBoundary => has_non_han_boundary(text, start, end),
    }
}

fn has_ascii_token_boundary(text: &str, start: usize, end: usize) -> bool {
    let left = match text[..start].chars().next_back() {
        Some(ch) => !ch.is_ascii_alphanumeric(),
        None => true,
    };
    let right = match text[end..].chars().next() {
        Some(ch) => !ch.is_ascii_alphanumeric(),
        None => true,
    };

    left && right
}

fn has_non_han_boundary(text: &str, start: usize, end: usize) -> bool {
    let left = match text[..start].chars().next_back() {
        Some(ch) => !is_han_character(ch),
        None => true,
    };
    let right = match text[end..].chars().next() {
        Some(ch) => !is_han_character(ch),
        None => true,
    };

    left && right
}

fn is_han_character(ch: char) -> bool {
    matches!(
        ch as u32,
        0x3400..=0x4DBF
            | 0x4E00..=0x9FFF
            | 0xF900..=0xFAFF
            | 0x20000..=0x2A6DF
            | 0x2A700..=0x2B73F
            | 0x2B740..=0x2B81F
            | 0x2B820..=0x2CEAF
            | 0x2CEB0..=0x2EBEF
            | 0x2F800..=0x2FA1F
            | 0x3007
    )
}

#[derive(Debug, Error)]
pub enum FuckDetectorError {
    #[error("invalid lexicon line {line}")]
    InvalidLexiconLine { line: usize },
    #[error("failed to build profanity matcher")]
    Matcher(#[source] aho_corasick::BuildError),
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::FuckDetector;

    #[test]
    fn parses_bundled_lexicon() {
        let detector = FuckDetector::new().unwrap();

        assert!(detector.entries().len() >= 200);
        assert_eq!(detector.entries()[0].code, 1);
    }

    #[test]
    fn counts_multilingual_profanities() {
        let detector = FuckDetector::from_lexicon("fuck\nmotherfucker\n傻逼\nバカ\n씨발").unwrap();

        let counts = detector.detect("motherfucker FUCK fuck 傻逼 バカ 씨발 씨발");

        assert_eq!(
            counts,
            BTreeMap::from([
                ("fuck".to_owned(), 2),
                ("motherfucker".to_owned(), 1),
                ("傻逼".to_owned(), 1),
                ("バカ".to_owned(), 1),
                ("씨발".to_owned(), 2),
            ])
        );
    }

    #[test]
    fn skips_ascii_partial_matches() {
        let detector = FuckDetector::from_lexicon("shit\nasshole").unwrap();
        let counts = detector.detect("shitake glasshole shit asshole");

        assert_eq!(
            counts,
            BTreeMap::from([("asshole".to_owned(), 1), ("shit".to_owned(), 1),])
        );
    }

    #[test]
    fn rejects_empty_lexicon_line() {
        let error = FuckDetector::from_lexicon("fuck\n\nshit").unwrap_err();

        assert!(error.to_string().contains("invalid lexicon line 2"));
    }

    #[test]
    fn single_han_entries_do_not_match_inside_words() {
        let detector = FuckDetector::from_lexicon("操\n草\n靠").unwrap();
        let counts = detector.detect("操作 稿草 靠谱 操，你就不会吗？ 操 我");

        assert_eq!(counts, BTreeMap::from([("操".to_owned(), 2)]));
    }

    #[test]
    fn single_han_entries_still_match_with_non_han_neighbors() {
        let detector = FuckDetector::from_lexicon("操").unwrap();
        let counts = detector.detect("操! a操? [操] 操 我");

        assert_eq!(counts, BTreeMap::from([("操".to_owned(), 4)]));
    }
}
