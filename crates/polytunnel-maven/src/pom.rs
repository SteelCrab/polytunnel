//! POM XML parser

use crate::coordinate::Coordinate;
use polytunnel_core::Result;
use quick_xml::Reader;
use quick_xml::events::Event;
use serde::{Deserialize, Serialize};

/// Parsed POM file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pom {
    pub coordinate: Coordinate,
    pub parent: Option<Coordinate>,
    pub dependencies: Vec<PomDependency>,
    pub dependency_management: Vec<PomDependency>,
    pub properties: std::collections::HashMap<String, String>,
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
pub fn parse_pom(xml: &str) -> Result<Pom> {
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);

    let mut pom = Pom {
        coordinate: Coordinate::new("", "", ""),
        parent: None,
        dependencies: Vec::new(),
        dependency_management: Vec::new(),
        properties: std::collections::HashMap::new(),
    };

    let mut current_path: Vec<String> = Vec::new();
    let mut current_text = String::new();

    // Temporary storage for parsing
    let mut group_id = String::new();
    let mut artifact_id = String::new();
    let mut version = String::new();
    let mut scope = DependencyScope::Compile;
    let mut optional = false;

    let mut in_dependency = false;
    let mut in_parent = false;
    let mut in_properties = false;
    let mut in_dependency_management = false;
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
                        "groupId" => {
                            if in_dependency || in_parent {
                                group_id = current_text.clone();
                            } else if current_path.len() == 2 {
                                pom.coordinate.group_id = current_text.clone();
                            }
                        }
                        "artifactId" => {
                            if in_dependency || in_parent {
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
                return Err(polytunnel_core::AppError::Io(std::io::Error::other(
                    format!("XML parse error: {}", e),
                )));
            }
            _ => {}
        }
    }

    Ok(pom)
}

impl Pom {
    /// Resolve property placeholders like ${project.version}
    pub fn resolve_property(&self, value: &str) -> String {
        let mut result = value.to_string();

        // Replace ${...} patterns
        while let Some(start) = result.find("${") {
            if let Some(end) = result[start..].find('}') {
                let key = &result[start + 2..start + end];
                let replacement = self
                    .properties
                    .get(key)
                    .cloned()
                    .unwrap_or_else(|| format!("${{{}}}", key));
                result = format!(
                    "{}{}{}",
                    &result[..start],
                    replacement,
                    &result[start + end + 1..]
                );
            } else {
                break;
            }
        }

        result
    }
}
