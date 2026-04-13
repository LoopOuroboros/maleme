const CLAUDE_INITIATOR_MARKER: &str = "<!-- OMO_INTERNAL_INITIATOR -->";
const CLAUDE_WRAPPER_DELIMITER: &str = "\n\n---\n\n";
const CODEX_AGENTS_PREFIX: &str = "# AGENTS.md instructions for ";
const CODEX_ENVIRONMENT_PREFIX: &str = "<environment_context>";
const CODEX_USER_INSTRUCTIONS_PREFIX: &str = "<user_instructions>";

pub fn trim_to_owned(text: &str) -> String {
    text.trim().to_owned()
}

pub fn normalize_codex_text(raw: &str) -> String {
    let trimmed = raw.trim();

    if trimmed.starts_with(CODEX_AGENTS_PREFIX)
        || trimmed.starts_with(CODEX_ENVIRONMENT_PREFIX)
        || trimmed.starts_with(CODEX_USER_INSTRUCTIONS_PREFIX)
    {
        return String::new();
    }

    trimmed.to_owned()
}

pub fn normalize_claude_text(raw: &str) -> String {
    let without_marker = raw.replace(CLAUDE_INITIATOR_MARKER, "");
    let trimmed = without_marker.trim();

    if is_control_only(trimmed) {
        return String::new();
    }

    if trimmed.starts_with("---") && trimmed.contains("[SYSTEM DIRECTIVE:") {
        let parts: Vec<&str> = trimmed.split(CLAUDE_WRAPPER_DELIMITER).collect();
        let content = parts[parts.len() - 1].trim();
        return content.to_owned();
    }

    trimmed.to_owned()
}

fn is_control_only(text: &str) -> bool {
    let stripped = strip_ansi_sequences(text);
    let visible: String = stripped
        .chars()
        .filter(|ch| !ch.is_control() || matches!(ch, '\n' | '\r' | '\t'))
        .collect();
    let compact = visible.trim();

    compact.is_empty() || (compact.starts_with("[>") && compact.contains("rgb:"))
}

fn strip_ansi_sequences(text: &str) -> String {
    let mut chars = text.chars().peekable();
    let mut out = String::new();

    while let Some(ch) = chars.next() {
        if ch == '\u{1b}' {
            if matches!(chars.peek(), Some('[' | ']' | '(' | ')')) {
                chars.next();
                while let Some(next) = chars.next() {
                    if ('@'..='~').contains(&next) || next == '\u{7}' {
                        break;
                    }
                }
            }
            continue;
        }

        out.push(ch);
    }

    out
}

#[cfg(test)]
mod tests {
    use super::{normalize_claude_text, normalize_codex_text};

    #[test]
    fn drops_codex_injected_wrapper_text() {
        assert_eq!(
            normalize_codex_text("# AGENTS.md instructions for /tmp/project"),
            ""
        );
        assert_eq!(
            normalize_codex_text(
                "<environment_context>\n  <cwd>/tmp</cwd>\n</environment_context>"
            ),
            ""
        );
        assert_eq!(normalize_codex_text("<user_instructions>\nignore"), "");
    }

    #[test]
    fn strips_claude_wrapper() {
        let input = "\n\n---\n\n[SYSTEM DIRECTIVE: TEST]\nignore\n\n---\n\nactual user text\n<!-- OMO_INTERNAL_INITIATOR -->";
        assert_eq!(normalize_claude_text(input), "actual user text");
    }

    #[test]
    fn drops_control_only_text() {
        let input = "[>0;276;0c]10;rgb:e2e2/e8e8/f0f0\u{1b}\\]11;rgb:0202/0606/1717\u{1b}\n";
        assert_eq!(normalize_claude_text(input), "");
    }
}
