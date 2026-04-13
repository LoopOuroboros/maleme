use indicatif::{ProgressBar, ProgressStyle};
use maleme::{
    AgentAdapter, ClaudeAdapter, CodexAdapter, FuckDetector, OpenCodeAdapter, write_report_and_open,
};

#[tokio::main]
async fn main() {
    let home = std::env::var("HOME").unwrap();
    let codex = CodexAdapter::new(&home);
    let claude = ClaudeAdapter::new(&home);
    let opencode = OpenCodeAdapter::new(&home);
    let codex_enabled = codex.check().await;
    let claude_enabled = claude.check().await;
    let opencode_enabled = opencode.check().await;
    let codex_units = if codex_enabled {
        codex.session_file_count().await.unwrap()
    } else {
        0
    };
    let claude_units = if claude_enabled {
        claude.transcript_file_count().await.unwrap()
    } else {
        0
    };
    let opencode_units = if opencode_enabled { 1 } else { 0 };
    let token_units = codex_enabled as u64 + claude_enabled as u64 + opencode_enabled as u64;
    let total_units = codex_units as u64 + claude_units as u64 + opencode_units + token_units + 1;
    let progress = ProgressBar::new(total_units.max(1));
    progress.set_style(
        ProgressStyle::with_template("{spinner:.green} {bar:40.cyan/blue} {pos}/{len} {msg}")
            .unwrap(),
    );

    let mut messages = Vec::new();

    if codex_enabled {
        messages.extend(
            codex
                .collect_messages_with_progress(progress.clone())
                .await
                .unwrap(),
        );
    }

    if claude_enabled {
        messages.extend(
            claude
                .collect_messages_with_progress(progress.clone())
                .await
                .unwrap(),
        );
    }

    if opencode_enabled {
        messages.extend(
            opencode
                .collect_messages_with_progress(progress.clone())
                .await
                .unwrap(),
        );
    }

    messages.sort_by_key(|message| message.time);

    let mut tokens = 0_i64;

    if codex_enabled {
        progress.set_message("Codex tokens".to_owned());
        tokens += codex.tokens().await.unwrap();
        progress.inc(1);
    }

    if claude_enabled {
        progress.set_message("Claude tokens".to_owned());
        tokens += claude.tokens().await.unwrap();
        progress.inc(1);
    }

    if opencode_enabled {
        progress.set_message("OpenCode tokens".to_owned());
        tokens += opencode.tokens().await.unwrap();
        progress.inc(1);
    }

    let detector = FuckDetector::new().unwrap();
    progress.set_message("生成 HTML 报告".to_owned());
    let report_path = write_report_and_open(&messages, tokens, &detector).unwrap();
    progress.inc(1);
    progress.finish_with_message("报告已生成并打开");

    println!("report: {}", report_path.display());
    println!("messages: {}", messages.len());
    println!("tokens: {}", tokens);
}
