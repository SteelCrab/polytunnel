use crate::error::{IdeError, Result};
use colored::*;
use polytunnel_build::BuildOrchestrator;
use polytunnel_core::ProjectConfig;
use std::path::Path;

pub async fn generate(config: &ProjectConfig, root_path: &Path) -> Result<()> {
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
    std::fs::write(root_path.join(".project"), project_xml)?;
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
    std::fs::write(root_path.join(".classpath"), classpath_xml)?;
    print_status("Created", ".classpath", Color::Green);

    // 3. Generate .vscode/settings.json
    let vscode_dir = root_path.join(".vscode");
    if !vscode_dir.exists() {
        std::fs::create_dir(&vscode_dir)?;
    }

    let settings_json = r#"{
    "java.configuration.updateBuildConfiguration": "disabled"
}
"#;
    std::fs::write(vscode_dir.join("settings.json"), settings_json)?;
    print_status("Created", ".vscode/settings.json", Color::Green);

    // 4. Update .gitignore
    update_gitignore(root_path)?;

    Ok(())
}

fn update_gitignore(root_path: &Path) -> Result<()> {
    let gitignore_path = root_path.join(".gitignore");
    let mut current_content = if gitignore_path.exists() {
        std::fs::read_to_string(&gitignore_path).map_err(IdeError::Io)?
    } else {
        String::new()
    };

    let mut updated = false;
    if !current_content.contains(".project") {
        if !current_content.ends_with('\n') && !current_content.is_empty() {
            current_content.push('\n');
        }
        current_content.push_str(".project\n");
        updated = true;
    }
    if !current_content.contains(".classpath") {
        if !current_content.ends_with('\n') && !current_content.is_empty() {
            current_content.push('\n');
        }
        current_content.push_str(".classpath\n");
        updated = true;
    }

    if updated {
        std::fs::write(gitignore_path, current_content).map_err(IdeError::Io)?;
        print_status("Updated", ".gitignore", Color::Green);
    }

    Ok(())
}

fn print_status(status: &str, message: &str, color: Color) {
    println!("{:>12} {}", status.color(color).bold(), message);
}
