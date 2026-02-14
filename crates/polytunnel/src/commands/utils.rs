use colored::*;

/// Helper for formatted status output
pub fn print_status(status: &str, message: &str, color: Color) {
    println!("{:>12} {}", status.color(color).bold(), message);
}
