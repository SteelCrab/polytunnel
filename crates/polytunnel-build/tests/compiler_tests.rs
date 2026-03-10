//! Tests for Java compiler utilities

use std::path::PathBuf;

#[test]
fn test_format_classpath() {
    let paths = vec![
        PathBuf::from("/usr/lib/lib1.jar"),
        PathBuf::from("/usr/lib/lib2.jar"),
    ];
    let result = polytunnel_build::format_classpath(&paths);
    assert!(result.contains("lib1.jar"));
    assert!(result.contains("lib2.jar"));
    if cfg!(windows) {
        assert!(result.contains(";"));
    } else {
        assert!(result.contains(":"));
    }
}
