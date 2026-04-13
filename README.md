# maleme

[![Repository](https://img.shields.io/badge/repository-GitHub-181717?logo=github)](https://github.com/Yeuoly/maleme)
[![Language](https://img.shields.io/badge/language-Rust-000000?logo=rust)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-WTFPL-brightgreen)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-local--first-blue)](https://github.com/Yeuoly/maleme)
[![Output](https://img.shields.io/badge/output-standalone%20HTML-orange)](https://github.com/Yeuoly/maleme)

`maleme` is a local-first Rust toolkit for collecting user-authored prompt history from coding agents, extracting profanity usage signals, and producing a compact visual analytics report.

The project is designed around three practical requirements:

1. Unified ingestion of locally stored conversation history from multiple coding agents.
2. Deterministic detection and aggregation of profanity usage across multilingual user input.
3. Repeatable generation of a browser-friendly HTML report suitable for local review.

## Highlights

- Local-only data processing with no mandatory hosted service dependency
- Unified adapters for Codex, Claude Code, and OpenCode
- Embedded multilingual profanity lexicon with file-based maintenance
- Single-file HTML report generation for lightweight sharing and inspection
- Token-normalized profanity metrics for cross-session comparison

## Overview

`maleme` reads local chat history from supported coding agents, normalizes user-authored text input, computes profanity frequency metrics, and renders a standalone HTML report.

The current implementation supports:

- Codex
- Claude Code
- OpenCode

The generated report includes:

- Daily profanity frequency over time
- A normalized profanity intensity metric (`SBAI`)
- A word cloud of the most frequently used terms

## Architecture

The repository is organized into a small set of focused modules:

- `src/agent_adapter/`
  Adapter implementations for each supported coding agent. Each adapter is responsible for:
  - local availability checks
  - user-message extraction
  - token usage extraction

- `src/fuck_detector.rs`
  Profanity lexicon loading and text matching logic.

- `src/report.rs`
  Report data aggregation, HTML rendering, and local browser launch.

- `data/profanity_lexicon.txt`
  Editable profanity lexicon embedded into the compiled binary at build time.

## Supported Data Sources

`maleme` operates against local files and databases already present on the host system. It does not require a remote service for analysis.

Current canonical sources:

- Codex:
  - `~/.codex/sessions/`
  - `~/.codex/archived_sessions/`
  - `~/.codex/state_5.sqlite`

- Claude Code:
  - `~/.claude/transcripts/`
  - `~/.claude/stats-cache.json`

- OpenCode:
  - `~/.local/share/opencode/opencode.db`

## Profanity Lexicon

The profanity lexicon is stored in:

- [`data/profanity_lexicon.txt`](/Users/yeuoly/Documents/code/maleme/data/profanity_lexicon.txt)

Format:

```text
term one
term two
term three
```

Rules:

- one entry per line
- line number is treated as the entry code
- modifying the text file and rebuilding the project updates the embedded lexicon

## Report Generation

Running the binary will:

1. scan supported local agent history
2. collect user-authored messages
3. calculate profanity metrics
4. generate a standalone HTML report in `~/Downloads`
5. open the report in the local default browser

## Development

Build:

```bash
cargo build
```

Run:

```bash
cargo run
```

Test:

```bash
cargo test
```

## Repository Metadata

- Source: [github.com/Yeuoly/maleme](https://github.com/Yeuoly/maleme)
- Primary language: Rust
- License: WTFPL
- Distribution model: local executable

## Star History

[![Star History Chart](https://api.star-history.com/svg?repos=Yeuoly/maleme&type=Date)](https://star-history.com/#Yeuoly/maleme&Date)

## Scope

This repository is intentionally local, explicit, and file-oriented. It is not intended to be a hosted analytics platform, a moderation service, or a generalized telemetry pipeline.

## License

This project is distributed under the terms of the [WTFPL](LICENSE).
