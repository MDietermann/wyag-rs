mod common;

#[test]
fn fixture_repo_pins_line_endings_and_modes() {
    let repo = common::fixture_repo();
    for key in ["core.autocrlf", "core.filemode"] {
        assert_eq!(
            common::git(repo.path(), &["config", key]).trim(),
            "false",
            "{key}"
        );
    }
}

#[test]
fn git_is_at_least_2_40() {
    // e.g. "git version 2.47.1.windows.1"
    let out = common::git(&std::env::temp_dir(), &["--version"]);
    let mut parts = out
        .trim()
        .trim_start_matches("git version ")
        .split('.')
        .map(|p| p.parse::<u32>().unwrap_or(0));
    let version = (parts.next().unwrap_or(0), parts.next().unwrap_or(0));
    assert!(version >= (2, 40), "need git >= 2.40, got {out}");
}
