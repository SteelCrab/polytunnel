//! Tests for TestRunner framework detection and pattern matching

use polytunnel_build::{TestFramework, TestRunner};
use std::path::PathBuf;

#[test]
fn test_detect_framework_junit5() {
    let classpath = vec![PathBuf::from("/lib/junit-jupiter-api-5.10.0.jar")];
    let framework = TestRunner::detect_framework(&classpath);
    assert_eq!(framework, Some(TestFramework::JUnit5));
}

#[test]
fn test_detect_framework_junit4() {
    let classpath = vec![PathBuf::from("/lib/junit-4.13.2.jar")];
    let framework = TestRunner::detect_framework(&classpath);
    assert_eq!(framework, Some(TestFramework::JUnit4));
}

#[test]
fn test_detect_framework_testng() {
    let classpath = vec![PathBuf::from("/lib/testng-7.8.0.jar")];
    let framework = TestRunner::detect_framework(&classpath);
    assert_eq!(framework, Some(TestFramework::TestNG));
}

#[test]
fn test_detect_framework_priority() {
    // JUnit 5 should have priority
    let classpath = vec![
        PathBuf::from("/lib/junit-jupiter-api-5.10.0.jar"),
        PathBuf::from("/lib/junit-4.13.2.jar"),
    ];
    let framework = TestRunner::detect_framework(&classpath);
    assert_eq!(framework, Some(TestFramework::JUnit5));
}

#[test]
fn test_matches_test_pattern() {
    let runner = TestRunner::new(
        TestFramework::JUnit5,
        vec![],
        PathBuf::from("target/test-classes"),
    );

    assert!(runner.matches_test_pattern("AppTest"));
    assert!(runner.matches_test_pattern("AppTests"));
    assert!(runner.matches_test_pattern("AppTestCase"));
    assert!(runner.matches_test_pattern("TestApp"));
    assert!(!runner.matches_test_pattern("App"));
}
