use colored::*;
use polytunnel_build::TestResult;

/// Helper for formatted status output
pub fn print_status(status: &str, message: &str, color: Color) {
    println!("{:>12} {}", status.color(color).bold(), message);
}

/// Print test result summary line
pub fn print_test_result(result: &TestResult, duration_secs: f64) {
    let (status_text, status_color) = if result.failed > 0 {
        ("FAILED", Color::Red)
    } else {
        ("ok", Color::Green)
    };

    println!(
        "\ntest result: {}. {} passed; {} failed; {} ignored; 0 measured; 0 filtered out; finished in {:.2}s\n",
        status_text.color(status_color),
        result.passed,
        result.failed,
        result.skipped,
        duration_secs
    );
}

/// Return error if tests failed
pub fn check_test_failures(result: &TestResult) -> color_eyre::eyre::Result<()> {
    if result.failed > 0 {
        return Err(polytunnel_build::BuildError::TestExecutionFailed {
            message: format!("{} test(s) failed", result.failed),
        }
        .into());
    }
    Ok(())
}
