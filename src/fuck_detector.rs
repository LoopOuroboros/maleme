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
    requires_boundary: Vec<bool>,
}

impl FuckDetector {
    pub fn new() -> Result<Self, FuckDetectorError> {
        Self::from_lexicon(BUNDLED_LEXICON)
    }

    pub fn from_lexicon(lexicon: &str) -> Result<Self, FuckDetectorError> {
        let mut entries = Vec::new();
        let mut requires_boundary = Vec::new();

        for (index, line) in lexicon.lines().enumerate() {
            let line_number = index + 1;
            if line.is_empty() {
                return Err(FuckDetectorError::InvalidLexiconLine { line: line_number });
            }

            let text = line.to_owned();
            requires_boundary.push(is_boundary_sensitive(&text));
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
            requires_boundary,
        })
    }

    pub fn entries(&self) -> &[ProfanityEntry] {
        &self.entries
    }

    pub fn detect(&self, text: &str) -> BTreeMap<String, i64> {
        let mut counts = BTreeMap::new();

        for found in self.matcher.find_iter(text) {
            let index = found.pattern().as_usize();

            if self.requires_boundary[index]
                && !has_token_boundary(text, found.start(), found.end())
            {
                continue;
            }

            let entry = &self.entries[index];
            *counts.entry(entry.text.clone()).or_insert(0) += 1;
        }

        counts
    }
}

fn is_boundary_sensitive(text: &str) -> bool {
    text.chars()
        .all(|ch| ch.is_ascii_alphanumeric() || ch == '\'' || ch == ' ' || ch == '-')
}

fn has_token_boundary(text: &str, start: usize, end: usize) -> bool {
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
}
