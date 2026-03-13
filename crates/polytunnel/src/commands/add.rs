use super::utils::print_status;
use color_eyre::eyre::{Result, bail};
use colored::Color;
use polytunnel_core::{DependencyScope, add_dependency_to_file, parse_add_coordinate};
use std::path::Path;

pub fn cmd_add(dependency: &str, scope: Option<&str>) -> Result<()> {
    do_add(dependency, scope, Path::new("polytunnel.toml"))
}

pub(crate) fn do_add(dependency: &str, scope: Option<&str>, config_path: &Path) -> Result<()> {
    // 1. Validate config exists
    if !config_path.exists() {
        bail!("polytunnel.toml not found. Run `pt init` first.");
    }

    // 2. Parse and validate coordinate
    let (ga_key, version) = parse_add_coordinate(dependency)?;

    // 3. Parse scope if provided
    let dep_scope = match scope {
        Some(s) => Some(parse_scope(s)?),
        None => None,
    };

    // 4. Add to file (format-preserving)
    add_dependency_to_file(config_path, &ga_key, &version, dep_scope)?;

    // 5. Print success message
    let scope_suffix = match dep_scope {
        Some(s) => format!(" (scope: {})", scope_str(s)),
        None => String::new(),
    };
    print_status(
        "Added",
        &format!("{}:{}{}", ga_key, version, scope_suffix),
        Color::Green,
    );

    Ok(())
}

fn parse_scope(s: &str) -> Result<DependencyScope> {
    match s.to_lowercase().as_str() {
        "compile" => Ok(DependencyScope::Compile),
        "runtime" => Ok(DependencyScope::Runtime),
        "test" => Ok(DependencyScope::Test),
        "provided" => Ok(DependencyScope::Provided),
        _ => bail!(
            "Invalid scope '{}'. Valid: compile, runtime, test, provided",
            s
        ),
    }
}

fn scope_str(scope: DependencyScope) -> &'static str {
    match scope {
        DependencyScope::Compile => "compile",
        DependencyScope::Runtime => "runtime",
        DependencyScope::Test => "test",
        DependencyScope::Provided => "provided",
    }
}
