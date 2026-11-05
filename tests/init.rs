mod common;

#[test]
fn git_accepts_wyag_initialized_repo() {
    let dir = tempfile::TempDir::new().unwrap();
    common::wyag(dir.path(), &["init"]);
    common::git(dir.path(), &["status"]);
    let git_dir = common::git(dir.path(), &["rev-parse", "--git-dir"]);
    assert_eq!(git_dir.trim(), ".git");
}
