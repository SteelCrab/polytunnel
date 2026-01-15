//! POM XML parser

use crate::coordinate::Coordinate;
use crate::error::{MavenError, Result};
use quick_xml::Reader;
use quick_xml::events::Event;
use serde::{Deserialize, Serialize};

/// Parsed POM file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pom {
    pub coordinate: Coordinate,
    #[serde(default = "default_packaging")]
    pub packaging: String,
    pub parent: Option<Coordinate>,
    pub dependencies: Vec<PomDependency>,
    pub dependency_management: Vec<PomDependency>,
    pub properties: std::collections::HashMap<String, String>,
}

fn default_packaging() -> String {
    "jar".to_string()
}

/// Dependency entry in POM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PomDependency {
    pub group_id: String,
    pub artifact_id: String,
    pub version: Option<String>,
    #[serde(default)]
    pub scope: DependencyScope,
    #[serde(default)]
    pub optional: bool,
    pub exclusions: Vec<Exclusion>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DependencyScope {
    #[default]
    Compile,
    Runtime,
    Test,
    Provided,
    System,
    Import,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Exclusion {
    pub group_id: String,
    pub artifact_id: String,
}

/// Parse POM XML content
fn inject_project_properties(pom: &mut Pom) {
    if !pom.coordinate.version.is_empty() {
        pom.properties.insert(
            "project.version".to_string(),
            pom.coordinate.version.clone(),
        );
        pom.properties
            .insert("pom.version".to_string(), pom.coordinate.version.clone());
        pom.properties
            .insert("version".to_string(), pom.coordinate.version.clone());
    }
    if !pom.coordinate.group_id.is_empty() {
        pom.properties.insert(
            "project.groupId".to_string(),
            pom.coordinate.group_id.clone(),
        );
        pom.properties
            .insert("pom.groupId".to_string(), pom.coordinate.group_id.clone());
        pom.properties
            .insert("groupId".to_string(), pom.coordinate.group_id.clone());
    }
    if !pom.coordinate.artifact_id.is_empty() {
        pom.properties.insert(
            "project.artifactId".to_string(),
            pom.coordinate.artifact_id.clone(),
        );
        pom.properties.insert(
            "pom.artifactId".to_string(),
            pom.coordinate.artifact_id.clone(),
        );
        pom.properties
            .insert("artifactId".to_string(), pom.coordinate.artifact_id.clone());
    }
}

pub fn parse_pom(xml: &str) -> Result<Pom> {
    // Detect if response is HTML (likely an error page from Maven Central)
    let trimmed = xml.trim();
    if trimmed.starts_with("<!DOCTYPE") || trimmed.starts_with("<html") {
        return Err(MavenError::XmlParse {
            message: "Received HTML response instead of POM XML (likely 404 or server error from Maven Central)".to_string(),
        });
    }

    let mut reader = Reader::from_str(xml);
    // ... existing initialization ...
    reader.config_mut().trim_text(true);

    let mut pom = Pom {
        coordinate: Coordinate::new("", "", ""),
        packaging: "jar".to_string(),
        parent: None,
        dependencies: Vec::new(),
        dependency_management: Vec::new(),
        properties: std::collections::HashMap::new(),
    };

    // ... existing parsing loop ...

    // START OF REPLACEMENT FOR END OF FUNCTION
    let mut current_path: Vec<String> = Vec::new();
    let mut current_text = String::new();
    let mut group_id = String::new();
    let mut artifact_id = String::new();
    let mut version = String::new();
    let mut scope = DependencyScope::Compile;
    let mut optional = false;

    let mut in_dependency = false;
    let mut in_parent = false;
    let mut in_properties = false;
    let mut in_dependency_management = false;
    let mut in_exclusion = false;
    let mut prop_name = String::new();

    loop {
        match reader.read_event() {
            Ok(Event::Start(e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                current_path.push(name.clone());

                match name.as_str() {
                    "dependency" => {
                        in_dependency = true;
                        group_id.clear();
                        artifact_id.clear();
                        version.clear();
                        scope = DependencyScope::Compile;
                        optional = false;
                    }
                    "exclusion" => in_exclusion = true,
                    "parent" => in_parent = true,
                    "properties" => in_properties = true,
                    "dependencyManagement" => in_dependency_management = true,
                    _ => {
                        if in_properties && current_path.len() == 3 {
                            prop_name = name;
                        }
                    }
                }
            }
            Ok(Event::End(e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).to_string();

                match name.as_str() {
                    "dependency" if in_dependency => {
                        let dep = PomDependency {
                            group_id: group_id.clone(),
                            artifact_id: artifact_id.clone(),
                            version: if version.is_empty() {
                                None
                            } else {
                                Some(version.clone())
                            },
                            scope: scope.clone(),
                            optional,
                            exclusions: Vec::new(),
                        };

                        if in_dependency_management {
                            pom.dependency_management.push(dep);
                        } else {
                            pom.dependencies.push(dep);
                        }
                        in_dependency = false;
                    }
                    "exclusion" => in_exclusion = false,
                    "parent" => {
                        pom.parent = Some(Coordinate::new(&group_id, &artifact_id, &version));
                        in_parent = false;
                        group_id.clear();
                        artifact_id.clear();
                        version.clear();
                    }
                    "properties" => in_properties = false,
                    "dependencyManagement" => in_dependency_management = false,
                    _ => {
                        if in_properties && !prop_name.is_empty() {
                            pom.properties
                                .insert(prop_name.clone(), current_text.clone());
                            prop_name.clear();
                        }
                    }
                }

                current_path.pop();
                current_text.clear();
            }
            Ok(Event::Text(e)) => {
                current_text = e.unescape().unwrap_or_default().to_string();

                if let Some(current_elem) = current_path.last() {
                    match current_elem.as_str() {
                        "packaging" => {
                            if current_path.len() == 2 {
                                pom.packaging = current_text.clone();
                            }
                        }
                        "groupId" => {
                            if in_exclusion {
                                // Ignore exclusion groupId
                            } else if in_dependency || in_parent {
                                group_id = current_text.clone();
                            } else if current_path.len() == 2 {
                                pom.coordinate.group_id = current_text.clone();
                            }
                        }
                        "artifactId" => {
                            if in_exclusion {
                                // Ignore exclusion artifactId
                            } else if in_dependency || in_parent {
                                artifact_id = current_text.clone();
                            } else if current_path.len() == 2 {
                                pom.coordinate.artifact_id = current_text.clone();
                            }
                        }
                        "version" => {
                            if in_dependency || in_parent {
                                version = current_text.clone();
                            } else if current_path.len() == 2 {
                                pom.coordinate.version = current_text.clone();
                            }
                        }
                        "scope" if in_dependency => {
                            scope = match current_text.as_str() {
                                "runtime" => DependencyScope::Runtime,
                                "test" => DependencyScope::Test,
                                "provided" => DependencyScope::Provided,
                                "system" => DependencyScope::System,
                                "import" => DependencyScope::Import,
                                _ => DependencyScope::Compile,
                            };
                        }
                        "optional" if in_dependency => {
                            optional = current_text == "true";
                        }
                        _ => {}
                    }
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => {
                return Err(MavenError::XmlParse {
                    message: format!("XML parse error: {}", e),
                });
            }
            _ => {}
        }
    }

    // Inject implicit project properties
    inject_project_properties(&mut pom);

    // Apply property substitution to dependencies
    let properties = &pom.properties;
    for dep in &mut pom.dependencies {
        dep.group_id = resolve_value(&dep.group_id, properties);
        dep.artifact_id = resolve_value(&dep.artifact_id, properties);
        if let Some(v) = &dep.version {
            dep.version = Some(resolve_value(v, properties));
        }
    }

    for dep in &mut pom.dependency_management {
        dep.group_id = resolve_value(&dep.group_id, properties);
        dep.artifact_id = resolve_value(&dep.artifact_id, properties);
        if let Some(v) = &dep.version {
            dep.version = Some(resolve_value(v, properties));
        }
    }

    Ok(pom)
}

fn resolve_value(value: &str, properties: &std::collections::HashMap<String, String>) -> String {
    let mut current = value.to_string();
    // Iteratively resolve to handle nested properties like ${a} -> ${b} -> value
    for _ in 0..10 {
        let next = resolve_single_pass(&current, properties);
        if next == current {
            return next;
        }
        current = next;
    }
    current
}

fn resolve_single_pass(
    value: &str,
    properties: &std::collections::HashMap<String, String>,
) -> String {
    let mut result = String::new();
    let mut chars = value.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '$' && chars.peek() == Some(&'{') {
            chars.next(); // consume '{'
            let mut key = String::new();
            let mut closed = false;

            for k_char in &mut chars {
                if k_char == '}' {
                    closed = true;
                    break;
                }
                key.push(k_char);
            }

            if closed {
                if let Some(val) = properties.get(&key) {
                    result.push_str(val);
                } else {
                    // Property not found, keep original text
                    result.push_str("${");
                    result.push_str(&key);
                    result.push('}');
                }
            } else {
                // Unclosed variable, just push what we have
                result.push_str("${");
                result.push_str(&key);
            }
        } else {
            result.push(c);
        }
    }

    result
}

impl Pom {
    /// Resolve property placeholders like ${project.version}
    pub fn resolve_property(&self, value: &str) -> String {
        resolve_value(value, &self.properties)
    }

    /// Merge properties from another source (e.g., parent POM) and re-resolve dependencies
    pub fn merge_properties(&mut self, extra_props: &std::collections::HashMap<String, String>) {
        // 1. Add extra properties if not present (child overrides parent)
        for (k, v) in extra_props {
            self.properties.entry(k.clone()).or_insert(v.clone());
        }

        // 2. Re-resolve properties in dependencies
        let props = &self.properties;

        for dep in &mut self.dependencies {
            // Resolve groupId and artifactId if they contain variables
            if dep.group_id.contains("${") {
                dep.group_id = resolve_value(&dep.group_id, props);
            }
            if dep.artifact_id.contains("${") {
                dep.artifact_id = resolve_value(&dep.artifact_id, props);
            }
            if let Some(v) = &dep.version {
                // Only try to resolve if it still looks like a variable
                if v.contains("${") {
                    dep.version = Some(resolve_value(v, props));
                }
            }
        }

        for dep in &mut self.dependency_management {
            // Resolve groupId and artifactId if they contain variables
            if dep.group_id.contains("${") {
                dep.group_id = resolve_value(&dep.group_id, props);
            }
            if dep.artifact_id.contains("${") {
                dep.artifact_id = resolve_value(&dep.artifact_id, props);
            }
            if let Some(v) = &dep.version
                && v.contains("${") {
                    dep.version = Some(resolve_value(v, props));
                }
        }
    }

    pub fn merge_dependency_management(&mut self, parent_dm: Vec<PomDependency>) {
        self.dependency_management.extend(parent_dm);
    }

    pub fn fill_missing_versions(&mut self) {
        for dep in &mut self.dependencies {
            if dep.version.is_none() {
                // Find in dependency_management
                for dm in &self.dependency_management {
                    if dm.group_id == dep.group_id && dm.artifact_id == dep.artifact_id
                        && let Some(v) = &dm.version {
                            dep.version = Some(v.clone());
                            break;
                        }
                }
            }
        }
    }
}
