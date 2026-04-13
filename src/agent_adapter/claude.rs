use std::path::{Path, PathBuf};

use indicatif::ProgressBar;
use serde::Deserialize;
use time::{OffsetDateTime, format_description::well_known::Rfc3339};
use tokio::fs;

use super::{
    AdapterError, AdapterKind, AgentAdapter, UserMessage, UserMessageStream,
    normalize::normalize_claude_text, stream_messages,
};

#[derive(Debug, Clone)]
pub struct ClaudeAdapter {
    root_dir: PathBuf,
}

impl ClaudeAdapter {
    pub fn new(home: impl AsRef<Path>) -> Self {
        Self {
            root_dir: home.as_ref().join(".claude"),
        }
    }

    pub fn from_path(path: impl Into<PathBuf>) -> Self {
        Self {
            root_dir: path.into(),
        }
    }

    fn transcripts_dir(&self) -> PathBuf {
        self.root_dir.join("transcripts")
    }

    fn stats_cache_path(&self) -> PathBuf {
        self.root_dir.join("stats-cache.json")
    }

    pub async fn transcript_file_count(&self) -> Result<usize, AdapterError> {
        let transcripts_dir = self.transcripts_dir();
        let mut count = 0_usize;
        let mut entries =
            fs::read_dir(&transcripts_dir)
                .await
                .map_err(|source| AdapterError::Io {
                    path: transcripts_dir.clone(),
                    source,
                })?;

        while let Some(entry) = entries
            .next_entry()
            .await
            .map_err(|source| AdapterError::Io {
                path: transcripts_dir.clone(),
                source,
            })?
        {
            if entry.path().to_string_lossy().ends_with(".jsonl") {
                count += 1;
            }
        }

        Ok(count)
    }

    pub async fn collect_messages_with_progress(
        &self,
        progress: ProgressBar,
    ) -> Result<Vec<UserMessage>, AdapterError> {
        let transcripts_dir = self.transcripts_dir();
        let mut paths = Vec::new();
        let mut entries =
            fs::read_dir(&transcripts_dir)
                .await
                .map_err(|source| AdapterError::Io {
                    path: transcripts_dir.clone(),
                    source,
                })?;

        while let Some(entry) = entries
            .next_entry()
            .await
            .map_err(|source| AdapterError::Io {
                path: transcripts_dir.clone(),
                source,
            })?
        {
            let path = entry.path();

            if path.to_string_lossy().ends_with(".jsonl") {
                paths.push(path);
            }
        }

        paths.sort();

        let total_files = paths.len();
        let mut messages = Vec::new();

        for (index, path) in paths.into_iter().enumerate() {
            progress.set_message(format!(
                "Claude {}/{} · {}",
                index + 1,
                total_files,
                path.file_name().unwrap().to_string_lossy()
            ));
            let contents = fs::read_to_string(&path)
                .await
                .map_err(|source| AdapterError::Io {
                    path: path.clone(),
                    source,
                })?;

            for (line_index, raw_line) in contents.lines().enumerate() {
                let line_number = line_index + 1;
                let kind: ClaudeEventKind = serde_json::from_str(raw_line).map_err(|source| {
                    AdapterError::InvalidJsonLine {
                        path: path.clone(),
                        line: line_number,
                        source,
                    }
                })?;

                if kind.event_type == "user" {
                    let event: ClaudeUserEvent =
                        serde_json::from_str(raw_line).map_err(|source| {
                            AdapterError::InvalidJsonLine {
                                path: path.clone(),
                                line: line_number,
                                source,
                            }
                        })?;
                    let datetime =
                        OffsetDateTime::parse(&event.timestamp, &Rfc3339).map_err(|source| {
                            AdapterError::InvalidTimestamp {
                                path: path.clone(),
                                line: line_number,
                                value: event.timestamp.clone(),
                                source,
                            }
                        })?;
                    let text = normalize_claude_text(&event.content);

                    if !text.is_empty() {
                        messages.push(UserMessage {
                            adapter: AdapterKind::Claude,
                            text,
                            time: (datetime.unix_timestamp_nanos() / 1_000_000) as i64,
                        });
                    }
                }
            }

            progress.inc(1);
        }

        Ok(messages)
    }
}

impl AgentAdapter for ClaudeAdapter {
    async fn check(&self) -> bool {
        fs::metadata(self.transcripts_dir()).await.is_ok()
    }

    async fn poll(&self) -> Result<UserMessageStream, AdapterError> {
        let transcripts_dir = self.transcripts_dir();
        let mut paths = Vec::new();
        let mut entries =
            fs::read_dir(&transcripts_dir)
                .await
                .map_err(|source| AdapterError::Io {
                    path: transcripts_dir.clone(),
                    source,
                })?;

        while let Some(entry) = entries
            .next_entry()
            .await
            .map_err(|source| AdapterError::Io {
                path: transcripts_dir.clone(),
                source,
            })?
        {
            let path = entry.path();

            if path.to_string_lossy().ends_with(".jsonl") {
                paths.push(path);
            }
        }

        paths.sort();

        let mut messages = Vec::new();

        for path in paths {
            let contents = fs::read_to_string(&path)
                .await
                .map_err(|source| AdapterError::Io {
                    path: path.clone(),
                    source,
                })?;

            for (index, raw_line) in contents.lines().enumerate() {
                let line_number = index + 1;
                let kind: ClaudeEventKind = serde_json::from_str(raw_line).map_err(|source| {
                    AdapterError::InvalidJsonLine {
                        path: path.clone(),
                        line: line_number,
                        source,
                    }
                })?;

                if kind.event_type == "user" {
                    let event: ClaudeUserEvent =
                        serde_json::from_str(raw_line).map_err(|source| {
                            AdapterError::InvalidJsonLine {
                                path: path.clone(),
                                line: line_number,
                                source,
                            }
                        })?;
                    let datetime =
                        OffsetDateTime::parse(&event.timestamp, &Rfc3339).map_err(|source| {
                            AdapterError::InvalidTimestamp {
                                path: path.clone(),
                                line: line_number,
                                value: event.timestamp.clone(),
                                source,
                            }
                        })?;
                    let text = normalize_claude_text(&event.content);

                    if !text.is_empty() {
                        messages.push(UserMessage {
                            adapter: AdapterKind::Claude,
                            text,
                            time: (datetime.unix_timestamp_nanos() / 1_000_000) as i64,
                        });
                    }
                }
            }
        }

        Ok(stream_messages(messages))
    }

    async fn tokens(&self) -> Result<i64, AdapterError> {
        let stats_cache_path = self.stats_cache_path();
        let contents = fs::read_to_string(&stats_cache_path)
            .await
            .map_err(|source| AdapterError::Io {
                path: stats_cache_path.clone(),
                source,
            })?;
        let stats: ClaudeStatsCache =
            serde_json::from_str(&contents).map_err(|source| AdapterError::InvalidJsonLine {
                path: stats_cache_path,
                line: 1,
                source,
            })?;
        let mut total = 0_i64;

        for usage in stats.model_usage.into_values() {
            total += usage.input_tokens;
            total += usage.output_tokens;
            total += usage.cache_read_input_tokens;
            total += usage.cache_creation_input_tokens;
        }

        Ok(total)
    }
}

#[derive(Debug, Deserialize)]
struct ClaudeEventKind {
    #[serde(rename = "type")]
    event_type: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ClaudeUserEvent {
    #[serde(rename = "type")]
    _event_type: String,
    timestamp: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct ClaudeStatsCache {
    #[serde(rename = "version")]
    _version: serde_json::Value,
    #[serde(rename = "modelUsage")]
    model_usage: std::collections::BTreeMap<String, ClaudeModelUsage>,
}

#[derive(Debug, Deserialize)]
struct ClaudeModelUsage {
    #[serde(default, rename = "inputTokens")]
    input_tokens: i64,
    #[serde(default, rename = "outputTokens")]
    output_tokens: i64,
    #[serde(default, rename = "cacheReadInputTokens")]
    cache_read_input_tokens: i64,
    #[serde(default, rename = "cacheCreationInputTokens")]
    cache_creation_input_tokens: i64,
    #[serde(default, rename = "webSearchRequests")]
    _web_search_requests: serde_json::Value,
    #[serde(default, rename = "costUSD")]
    _cost_usd: serde_json::Value,
    #[serde(default, rename = "contextWindow")]
    _context_window: serde_json::Value,
}

#[cfg(test)]
mod tests {
    use std::fs;

    use futures::TryStreamExt;
    use tempfile::tempdir;

    use super::{AgentAdapter, ClaudeAdapter};

    #[tokio::test]
    async fn parses_and_cleans_claude_messages() {
        let temp = tempdir().unwrap();
        let transcripts = temp.path().join(".claude/transcripts");
        fs::create_dir_all(&transcripts).unwrap();
        fs::write(
            transcripts.join("ses_1.jsonl"),
            concat!(
                "{\"type\":\"assistant\",\"timestamp\":\"2026-03-04T07:01:57.000Z\",\"content\":\"ignore\"}\n",
                "{\"type\":\"user\",\"timestamp\":\"2026-03-04T07:01:56.809Z\",\"content\":\"\\n\\n---\\n\\n[SYSTEM DIRECTIVE: TEST]\\nignore\\n\\n---\\n\\nactual user text\\n<!-- OMO_INTERNAL_INITIATOR -->\"}\n",
                "{\"type\":\"user\",\"timestamp\":\"2026-03-04T07:01:58.000Z\",\"content\":\"[>0;276;0c]10;rgb:e2e2/e8e8/f0f0\\u001b\\\\]11;rgb:0202/0606/1717\\u001b\\n\"}\n",
            ),
        )
        .unwrap();

        let messages = ClaudeAdapter::new(temp.path())
            .poll()
            .await
            .unwrap()
            .try_collect::<Vec<_>>()
            .await
            .unwrap();

        assert_eq!(messages.len(), 1);
        assert_eq!(format!("{:?}", messages[0].adapter), "Claude");
        assert_eq!(messages[0].text, "actual user text");
        assert_eq!(messages[0].time, 1_772_607_716_809);
    }

    #[tokio::test]
    async fn fails_on_invalid_user_shape() {
        let temp = tempdir().unwrap();
        let transcripts = temp.path().join(".claude/transcripts");
        fs::create_dir_all(&transcripts).unwrap();
        fs::write(
            transcripts.join("ses_1.jsonl"),
            "{\"type\":\"user\",\"timestamp\":\"bad\",\"content\":\"hello\"}\n",
        )
        .unwrap();

        let error = match ClaudeAdapter::new(temp.path()).poll().await {
            Ok(_) => panic!("expected claude poll to fail"),
            Err(error) => error,
        };

        assert!(error.to_string().contains("invalid timestamp"));
    }

    #[tokio::test]
    async fn sums_claude_tokens_from_stats_cache() {
        let temp = tempdir().unwrap();
        let claude_dir = temp.path().join(".claude");
        fs::create_dir_all(claude_dir.join("transcripts")).unwrap();
        fs::write(
            claude_dir.join("stats-cache.json"),
            r#"{"version":2,"lastComputedDate":"2026-02-11","dailyActivity":[],"dailyModelTokens":[],"modelUsage":{"claude-opus":{"inputTokens":1,"outputTokens":2,"cacheReadInputTokens":3,"cacheCreationInputTokens":4,"webSearchRequests":0,"costUSD":0,"contextWindow":0,"maxOutputTokens":0},"claude-sonnet":{"inputTokens":5,"outputTokens":6,"cacheReadInputTokens":7,"cacheCreationInputTokens":8,"webSearchRequests":0,"costUSD":0,"contextWindow":0,"maxOutputTokens":0}},"totalSessions":1,"totalMessages":1,"longestSession":{},"firstSessionDate":"2025-11-20T06:26:38.724Z","hourCounts":{"14":1},"totalSpeculationTimeSavedMs":0}"#,
        )
        .unwrap();

        let adapter = ClaudeAdapter::new(temp.path());

        assert_eq!(adapter.tokens().await.unwrap(), 36);
    }

    #[tokio::test]
    async fn tolerates_missing_and_new_claude_stats_fields() {
        let temp = tempdir().unwrap();
        let claude_dir = temp.path().join(".claude");
        fs::create_dir_all(claude_dir.join("transcripts")).unwrap();
        fs::write(
            claude_dir.join("stats-cache.json"),
            r#"{"version":{"major":2},"modelUsage":{"claude-opus":{"inputTokens":1,"outputTokens":2,"cacheReadInputTokens":3,"brandNewField":999},"claude-sonnet":{"outputTokens":6,"cacheCreationInputTokens":8,"anotherNewField":"x"}},"newTopLevelField":{"anything":true}}"#,
        )
        .unwrap();

        let adapter = ClaudeAdapter::new(temp.path());

        assert_eq!(adapter.tokens().await.unwrap(), 20);
    }
}
