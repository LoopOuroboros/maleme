use std::path::{Path, PathBuf};

use indicatif::ProgressBar;
use rusqlite::{Connection, OpenFlags};
use tokio::{fs, task};

use super::{
    AdapterError, AdapterKind, AgentAdapter, UserMessage, UserMessageStream,
    normalize::trim_to_owned, stream_messages,
};

const QUERY: &str = r#"
SELECT
  json_extract(p.data, '$.text') AS text,
  m.time_created AS time
FROM message m
JOIN part p ON p.message_id = m.id
WHERE json_extract(m.data, '$.role') = 'user'
  AND json_extract(p.data, '$.type') = 'text'
ORDER BY m.time_created ASC, p.time_created ASC
"#;

const TOKENS_QUERY: &str = r#"
SELECT COALESCE(SUM(
  CAST(json_extract(data, '$.tokens.input') AS INTEGER) +
  CAST(json_extract(data, '$.tokens.output') AS INTEGER) +
  CAST(json_extract(data, '$.tokens.reasoning') AS INTEGER) +
  CAST(json_extract(data, '$.tokens.cache.read') AS INTEGER) +
  CAST(json_extract(data, '$.tokens.cache.write') AS INTEGER)
), 0)
FROM part
WHERE json_extract(data, '$.type') = 'step-finish'
"#;

#[derive(Debug, Clone)]
pub struct OpenCodeAdapter {
    db_path: PathBuf,
}

impl OpenCodeAdapter {
    pub fn new(home: impl AsRef<Path>) -> Self {
        Self {
            db_path: home.as_ref().join(".local/share/opencode/opencode.db"),
        }
    }

    pub fn from_path(path: impl Into<PathBuf>) -> Self {
        Self {
            db_path: path.into(),
        }
    }

    pub async fn collect_messages_with_progress(
        &self,
        progress: ProgressBar,
    ) -> Result<Vec<UserMessage>, AdapterError> {
        progress.set_message("OpenCode 1/1 · opencode.db".to_owned());
        let db_path = self.db_path.clone();
        let messages = task::spawn_blocking(move || {
            let connection =
                Connection::open_with_flags(&db_path, OpenFlags::SQLITE_OPEN_READ_ONLY).map_err(
                    |source| AdapterError::SqliteOpen {
                        path: db_path.clone(),
                        source,
                    },
                )?;
            let mut statement =
                connection
                    .prepare(QUERY)
                    .map_err(|source| AdapterError::SqliteQuery {
                        path: db_path.clone(),
                        source,
                    })?;
            let rows = statement
                .query_map([], |row| {
                    let text: String = row.get(0)?;
                    let time: i64 = row.get(1)?;
                    Ok(UserMessage {
                        adapter: AdapterKind::OpenCode,
                        text,
                        time,
                    })
                })
                .map_err(|source| AdapterError::SqliteQuery {
                    path: db_path.clone(),
                    source,
                })?;
            let mut messages = Vec::new();

            for row in rows {
                let message = row.map_err(|source| AdapterError::SqliteQuery {
                    path: db_path.clone(),
                    source,
                })?;
                let text = trim_to_owned(&message.text);

                if !text.is_empty() {
                    messages.push(UserMessage {
                        adapter: AdapterKind::OpenCode,
                        text,
                        time: message.time,
                    });
                }
            }

            Ok(messages)
        })
        .await
        .map_err(AdapterError::Join)??;

        progress.inc(1);
        Ok(messages)
    }
}

impl AgentAdapter for OpenCodeAdapter {
    async fn check(&self) -> bool {
        fs::metadata(&self.db_path).await.is_ok()
    }

    async fn poll(&self) -> Result<UserMessageStream, AdapterError> {
        let db_path = self.db_path.clone();
        let messages = task::spawn_blocking(move || {
            let connection =
                Connection::open_with_flags(&db_path, OpenFlags::SQLITE_OPEN_READ_ONLY).map_err(
                    |source| AdapterError::SqliteOpen {
                        path: db_path.clone(),
                        source,
                    },
                )?;
            let mut statement =
                connection
                    .prepare(QUERY)
                    .map_err(|source| AdapterError::SqliteQuery {
                        path: db_path.clone(),
                        source,
                    })?;
            let rows = statement
                .query_map([], |row| {
                    let text: String = row.get(0)?;
                    let time: i64 = row.get(1)?;
                    Ok(UserMessage {
                        adapter: AdapterKind::OpenCode,
                        text,
                        time,
                    })
                })
                .map_err(|source| AdapterError::SqliteQuery {
                    path: db_path.clone(),
                    source,
                })?;
            let mut messages = Vec::new();

            for row in rows {
                let message = row.map_err(|source| AdapterError::SqliteQuery {
                    path: db_path.clone(),
                    source,
                })?;
                let text = trim_to_owned(&message.text);

                if !text.is_empty() {
                    messages.push(UserMessage {
                        adapter: AdapterKind::OpenCode,
                        text,
                        time: message.time,
                    });
                }
            }

            Ok(messages)
        })
        .await
        .map_err(AdapterError::Join)??;

        Ok(stream_messages(messages))
    }

    async fn tokens(&self) -> Result<i64, AdapterError> {
        let db_path = self.db_path.clone();
        let total = task::spawn_blocking(move || {
            let connection =
                Connection::open_with_flags(&db_path, OpenFlags::SQLITE_OPEN_READ_ONLY).map_err(
                    |source| AdapterError::SqliteOpen {
                        path: db_path.clone(),
                        source,
                    },
                )?;
            let total = connection
                .query_row(TOKENS_QUERY, [], |row| row.get::<_, i64>(0))
                .map_err(|source| AdapterError::SqliteQuery {
                    path: db_path.clone(),
                    source,
                })?;

            Ok(total)
        })
        .await
        .map_err(AdapterError::Join)??;

        Ok(total)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use futures::TryStreamExt;
    use rusqlite::Connection;
    use tempfile::tempdir;

    use super::{AgentAdapter, OpenCodeAdapter};

    #[tokio::test]
    async fn reads_user_text_messages_from_sqlite() {
        let temp = tempdir().unwrap();
        let db_dir = temp.path().join(".local/share/opencode");
        fs::create_dir_all(&db_dir).unwrap();
        let db_path = db_dir.join("opencode.db");
        let connection = Connection::open(&db_path).unwrap();

        connection
            .execute_batch(
                r#"
                CREATE TABLE message (
                    id TEXT PRIMARY KEY,
                    session_id TEXT NOT NULL,
                    time_created INTEGER NOT NULL,
                    time_updated INTEGER NOT NULL,
                    data TEXT NOT NULL
                );
                CREATE TABLE part (
                    id TEXT PRIMARY KEY,
                    message_id TEXT NOT NULL,
                    session_id TEXT NOT NULL,
                    time_created INTEGER NOT NULL,
                    time_updated INTEGER NOT NULL,
                    data TEXT NOT NULL
                );
                "#,
            )
            .unwrap();
        connection
            .execute(
                "INSERT INTO message (id, session_id, time_created, time_updated, data) VALUES (?1, ?2, ?3, ?4, ?5)",
                (
                    "msg_user",
                    "ses_1",
                    1000_i64,
                    1000_i64,
                    r#"{"role":"user"}"#,
                ),
            )
            .unwrap();
        connection
            .execute(
                "INSERT INTO message (id, session_id, time_created, time_updated, data) VALUES (?1, ?2, ?3, ?4, ?5)",
                (
                    "msg_assistant",
                    "ses_1",
                    2000_i64,
                    2000_i64,
                    r#"{"role":"assistant"}"#,
                ),
            )
            .unwrap();
        connection
            .execute(
                "INSERT INTO part (id, message_id, session_id, time_created, time_updated, data) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                (
                    "prt_user",
                    "msg_user",
                    "ses_1",
                    1001_i64,
                    1001_i64,
                    r#"{"type":"text","text":" hello "}"#,
                ),
            )
            .unwrap();
        connection
            .execute(
                "INSERT INTO part (id, message_id, session_id, time_created, time_updated, data) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                (
                    "prt_assistant",
                    "msg_assistant",
                    "ses_1",
                    2001_i64,
                    2001_i64,
                    r#"{"type":"text","text":"ignore"}"#,
                ),
            )
            .unwrap();

        let messages = OpenCodeAdapter::new(temp.path())
            .poll()
            .await
            .unwrap()
            .try_collect::<Vec<_>>()
            .await
            .unwrap();

        assert_eq!(messages.len(), 1);
        assert_eq!(format!("{:?}", messages[0].adapter), "OpenCode");
        assert_eq!(messages[0].text, "hello");
        assert_eq!(messages[0].time, 1000);
    }

    #[tokio::test]
    async fn sums_opencode_tokens_from_step_finish_parts() {
        let temp = tempdir().unwrap();
        let db_dir = temp.path().join(".local/share/opencode");
        fs::create_dir_all(&db_dir).unwrap();
        let db_path = db_dir.join("opencode.db");
        let connection = Connection::open(&db_path).unwrap();

        connection
            .execute_batch(
                r#"
                CREATE TABLE part (
                    id TEXT PRIMARY KEY,
                    message_id TEXT NOT NULL,
                    session_id TEXT NOT NULL,
                    time_created INTEGER NOT NULL,
                    time_updated INTEGER NOT NULL,
                    data TEXT NOT NULL
                );
                INSERT INTO part (id, message_id, session_id, time_created, time_updated, data)
                VALUES ('prt1', 'msg1', 'ses1', 1, 1, '{"type":"step-finish","tokens":{"input":1,"output":2,"reasoning":3,"cache":{"read":4,"write":5}}}');
                INSERT INTO part (id, message_id, session_id, time_created, time_updated, data)
                VALUES ('prt2', 'msg2', 'ses1', 2, 2, '{"type":"step-finish","tokens":{"input":6,"output":7,"reasoning":8,"cache":{"read":9,"write":10}}}');
                "#,
            )
            .unwrap();

        let adapter = OpenCodeAdapter::new(temp.path());

        assert_eq!(adapter.tokens().await.unwrap(), 55);
    }
}
