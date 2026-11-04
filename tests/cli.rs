mod common;

#[test]
fn prints_version() {
    let out = common::wyag(&std::env::temp_dir(), &["--version"]);
    assert_eq!(out.trim(), format!("wyag {}", env!("CARGO_PKG_VERSION")));
}
