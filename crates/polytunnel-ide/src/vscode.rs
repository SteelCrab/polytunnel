use crate::error::{IdeError, Result};
use colored::*;
use polytunnel_build::BuildOrchestrator;
use polytunnel_core::ProjectConfig;
use std::path::Path;

pub async fn generate(config: &ProjectConfig) -> Result<()> {
    print_status("Generating", "VS Code configuration...", Color::Cyan);

    let name = config.project.name.clone();

    // Create orchestrator and resolve dependencies
    let mut orchestrator = BuildOrchestrator::new(config.clone()).map_err(IdeError::Build)?;
    print_status("Resolving", "dependencies...", Color::Cyan);
    orchestrator
        .resolve_dependencies(false)
        .await
        .map_err(IdeError::Build)?;

    let classpath_result = orchestrator.get_resolved_classpath();

    // 1. Generate .project
    let project_xml = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<projectDescription>
	<name>{}</name>
	<comment></comment>
	<projects>
	</projects>
	<buildSpec>
		<buildCommand>
			<name>org.eclipse.jdt.core.javabuilder</name>
			<arguments>
			</arguments>
		</buildCommand>
	</buildSpec>
	<natures>
		<nature>org.eclipse.jdt.core.javanature</nature>
	</natures>
</projectDescription>
"#,
        name
    );
    std::fs::write(".project", project_xml)?;
    print_status("Created", ".project", Color::Green);

    // 2. Generate .classpath
    let mut classpath_xml = String::from(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<classpath>
	<classpathentry kind="src" path="src/main/java"/>
	<classpathentry kind="src" output="target/test-classes" path="src/test/java">
		<attributes>
			<attribute name="test" value="true"/>
		</attributes>
	</classpathentry>
	<classpathentry kind="con" path="org.eclipse.jdt.launching.JRE_CONTAINER"/>
	<classpathentry kind="output" path="target/classes"/>
"#,
    );

    // Filter duplicates: keep track of added paths
    let mut added_paths = std::collections::HashSet::new();

    // Add compile dependencies
    for path in classpath_result.compile_classpath {
        let path_str = path.to_string_lossy().to_string();
        if added_paths.insert(path_str.clone()) {
            classpath_xml.push_str(&format!(
                "\t<classpathentry kind=\"lib\" path=\"{}\"/>\n",
                path_str
            ));
        }
    }

    // Add test dependencies (only if not already added)
    for path in classpath_result.test_classpath {
        let path_str = path.to_string_lossy().to_string();
        if added_paths.insert(path_str.clone()) {
            classpath_xml.push_str(&format!(
                r#"	<classpathentry kind="lib" path="{}">
		<attributes>
			<attribute name="test" value="true"/>
		</attributes>
	</classpathentry>
"#,
                path_str
            ));
        }
    }

    classpath_xml.push_str("</classpath>\n");
    std::fs::write(".classpath", classpath_xml)?;
    print_status("Created", ".classpath", Color::Green);

    // 3. Generate .vscode/settings.json
    let vscode_dir = Path::new(".vscode");
    if !vscode_dir.exists() {
        std::fs::create_dir(vscode_dir)?;
    }

    let settings_json = r#"{
    "java.configuration.updateBuildConfiguration": "disabled"
}
"#;
    std::fs::write(".vscode/settings.json", settings_json)?;
    print_status("Created", ".vscode/settings.json", Color::Green);

    Ok(())
}

fn print_status(status: &str, message: &str, color: Color) {
    println!("{:>12} {}", status.color(color).bold(), message);
}
