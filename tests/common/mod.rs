// Each test binary compiles this module and uses only part of it
#![allow(dead_code)]

use std::path::Path;
use std::process::Command;

use tempfile::TempDir;

/// Fixed identity and timestamps so commit and tag hashes are deterministic
const ENV: [(&str, &str); 6] = [
    ("GIT_AUTHOR_NAME", "wyag test"),
    ("GIT_AUTHOR_EMAIL", "test@example.com"),
    ("GIT_AUTHOR_DATE", "2026-01-01T00:00:00+00:00"),
    ("GIT_COMMITTER_NAME", "wyag test"),
    ("GIT_COMMITTER_EMAIL", "test@example.com"),
    ("GIT_COMMITTER_DATE", "2026-01-01T00:00:00+00:00"),
];

pub fn git(dir: &Path, args: &[&str]) -> String {
    run(Command::new("git").current_dir(dir).args(args).envs(ENV))
}

pub fn wyag(dir: &Path, args: &[&str]) -> String {
    run(Command::new(env!("CARGO_BIN_EXE_wyag"))
        .current_dir(dir)
        .args(args)
        .envs(ENV))
}

/// Empty repo on branch `main` whose local config overrides anything global
/// that would change bytes on disk or require signing
pub fn fixture_repo() -> TempDir {
    let dir = TempDir::new().expect("failed to create temporary directory");
    git(dir.path(), &["init", "-q", "-b", "main"]);
    for (key, value) in [
        ("core.autocrlf", "false"),
        ("core.filemode", "false"),
        ("commit.gpgsign", "false"),
        ("tag.gpgsign", "false"),
    ] {
        git(dir.path(), &["config", key, value]);
    }
    dir
}

fn run(cmd: &mut Command) -> String {
    let out = cmd.output().expect("failed to spawn command");
    assert!(
        out.status.success(),
        "{cmd:?} failed:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout).expect("utf8")
}
