CREATE TABLE IF NOT EXISTS leaderboard_submission_model_sbai (
  submission_id INTEGER NOT NULL,
  github_id INTEGER NOT NULL,
  model TEXT NOT NULL,
  profanity_count INTEGER NOT NULL DEFAULT 0,
  tokens INTEGER NOT NULL DEFAULT 0,
  sbai REAL NOT NULL DEFAULT 0,
  created_at INTEGER NOT NULL,
  PRIMARY KEY (submission_id, model),
  FOREIGN KEY (submission_id) REFERENCES leaderboard_submissions(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS leaderboard_submission_model_sbai_user_idx
  ON leaderboard_submission_model_sbai (github_id, created_at DESC);

CREATE INDEX IF NOT EXISTS leaderboard_submission_model_sbai_model_idx
  ON leaderboard_submission_model_sbai (model, sbai DESC, profanity_count DESC);
