//! Tests for POM parser

use polytunnel_maven::{DependencyScope, parse_pom};
use std::collections::HashMap;

#[test]
fn test_parse_simple_pom() {
    let xml = r#"
    <project>
        <groupId>org.example</groupId>
        <artifactId>my-lib</artifactId>
        <version>1.0.0</version>
        <dependencies>
            <dependency>
                <groupId>org.slf4j</groupId>
                <artifactId>slf4j-api</artifactId>
                <version>2.0.9</version>
            </dependency>
        </dependencies>
    </project>
    "#;

    let pom = parse_pom(xml).unwrap();
    assert_eq!(pom.coordinate.group_id, "org.example");
    assert_eq!(pom.coordinate.artifact_id, "my-lib");
    assert_eq!(pom.coordinate.version, "1.0.0");
    assert_eq!(pom.dependencies.len(), 1);
    assert_eq!(pom.dependencies[0].group_id, "org.slf4j");
    assert_eq!(pom.dependencies[0].artifact_id, "slf4j-api");
}

#[test]
fn test_parse_pom_with_parent() {
    let xml = r#"
    <project>
        <parent>
            <groupId>org.springframework.boot</groupId>
            <artifactId>spring-boot-starter-parent</artifactId>
            <version>3.2.0</version>
        </parent>
        <artifactId>my-app</artifactId>
    </project>
    "#;

    let pom = parse_pom(xml).unwrap();
    assert!(pom.parent.is_some());
    let parent = pom.parent.unwrap();
    assert_eq!(parent.group_id, "org.springframework.boot");
}

#[test]
fn test_parse_pom_with_scope() {
    let xml = r#"
    <project>
        <groupId>org.example</groupId>
        <artifactId>my-lib</artifactId>
        <version>1.0.0</version>
        <dependencies>
            <dependency>
                <groupId>org.junit</groupId>
                <artifactId>junit</artifactId>
                <version>5.0.0</version>
                <scope>test</scope>
            </dependency>
        </dependencies>
    </project>
    "#;

    let pom = parse_pom(xml).unwrap();
    assert_eq!(
        pom.dependencies[0].scope,
        polytunnel_maven::DependencyScope::Test
    );
}

#[test]
fn test_parse_pom_with_optional() {
    let xml = r#"
    <project>
        <groupId>org.example</groupId>
        <artifactId>my-lib</artifactId>
        <version>1.0.0</version>
        <dependencies>
            <dependency>
                <groupId>org.optional</groupId>
                <artifactId>optional-lib</artifactId>
                <version>1.0.0</version>
                <optional>true</optional>
            </dependency>
        </dependencies>
    </project>
    "#;

    let pom = parse_pom(xml).unwrap();
    assert!(pom.dependencies[0].optional);
}

#[test]
fn test_parse_html_like_response() {
    let xml = "<!DOCTYPE html><html><body>error</body></html>";

    let result = parse_pom(xml);
    assert!(result.is_err());
}

#[test]
fn test_parse_pom_with_properties_and_scopes() {
    let xml = r#"
    <project>
        <modelVersion>4.0.0</modelVersion>
        <groupId>com.example</groupId>
        <artifactId>service</artifactId>
        <version>1.0.0</version>
        <packaging>jar</packaging>
        <properties>
            <managed.version>2.0.0</managed.version>
            <runtime.version>${managed.version}</runtime.version>
        </properties>
        <dependencyManagement>
            <dependencies>
                <dependency>
                    <groupId>com.example</groupId>
                    <artifactId>managed-dep</artifactId>
                    <version>${managed.version}</version>
                </dependency>
            </dependencies>
        </dependencyManagement>
        <dependencies>
            <dependency>
                <groupId>org.system</groupId>
                <artifactId>system-lib</artifactId>
                <version>1.0.0</version>
                <scope>system</scope>
            </dependency>
            <dependency>
                <groupId>org.import</groupId>
                <artifactId>import-lib</artifactId>
                <version>1.0.0</version>
                <scope>import</scope>
            </dependency>
            <dependency>
                <groupId>${missing.group}</groupId>
                <artifactId>${missing.artifact}</artifactId>
                <version>${managed.version}</version>
                <scope>runtime</scope>
            </dependency>
            <dependency>
                <groupId>com.example</groupId>
                <artifactId>managed-dep</artifactId>
            </dependency>
            <dependency>
                <groupId>com.example</groupId>
                <artifactId>runtime-dep</artifactId>
                <version>${runtime.version}</version>
                <scope>provided</scope>
            </dependency>
            <dependency>
                <groupId>org.unknown</groupId>
                <artifactId>default-scoped</artifactId>
                <version>1.0.0</version>
                <scope>does-not-exist</scope>
            </dependency>
        </dependencies>
    </project>
    "#;

    let pom = parse_pom(xml).unwrap();

    assert_eq!(pom.coordinate.version, "1.0.0");
    assert_eq!(pom.properties["project.version"], "1.0.0");
    assert_eq!(pom.dependencies.len(), 6);

    let scopes: Vec<_> = pom
        .dependencies
        .iter()
        .map(|dep| dep.scope.clone())
        .collect();

    assert!(scopes.contains(&DependencyScope::System));
    assert!(scopes.contains(&DependencyScope::Import));
    assert!(scopes.contains(&DependencyScope::Runtime));
    assert!(scopes.contains(&DependencyScope::Provided));
    assert!(scopes.contains(&DependencyScope::Compile));

    let managed_dep = pom
        .dependencies
        .iter()
        .find(|dep| dep.group_id == "com.example" && dep.artifact_id == "managed-dep")
        .expect("managed dependency should be present");
    assert_eq!(managed_dep.version, None);

    let managed_dm = pom
        .dependency_management
        .iter()
        .find(|dep| dep.group_id == "com.example" && dep.artifact_id == "managed-dep")
        .expect("managed dependency entry should be present");
    assert_eq!(managed_dm.version.as_deref(), Some("2.0.0"));

    let runtime_dep = pom
        .dependencies
        .iter()
        .find(|dep| dep.group_id == "com.example" && dep.artifact_id == "runtime-dep")
        .expect("runtime dependency should be present");
    assert_eq!(runtime_dep.version.as_deref(), Some("2.0.0"));
}

#[test]
fn test_pom_property_helpers() {
    let mut pom = parse_pom(
        r#"
    <project>
        <modelVersion>4.0.0</modelVersion>
        <groupId>com.example</groupId>
        <artifactId>service</artifactId>
        <version>1.0.0</version>
        <properties>
            <base.version>${project.version}</base.version>
        </properties>
        <dependencies>
            <dependency>
                <groupId>com.example</groupId>
                <artifactId>nested</artifactId>
                <version>${base.version}</version>
            </dependency>
        </dependencies>
    </project>
    "#,
    )
    .unwrap();

    assert_eq!(pom.resolve_property("${base.version}"), "1.0.0");
    assert_eq!(
        pom.resolve_property("${base.version-${missing}}"),
        "${base.version-${missing}}"
    );
    assert_eq!(pom.resolve_property("${base.version"), "${base.version");
    assert_eq!(pom.resolve_property("${nested}"), "${nested}");

    let mut extra: HashMap<String, String> = HashMap::new();
    extra.insert("resolved.group".to_string(), "com.extra".to_string());
    extra.insert(
        "resolved.artifact".to_string(),
        "artifact-extra".to_string(),
    );
    extra.insert("resolved.version".to_string(), "9.9.9".to_string());

    pom.merge_properties(&extra);
    pom.fill_missing_versions();

    assert_eq!(pom.dependencies[0].version.as_deref(), Some("1.0.0"));

    let mut dm = Vec::new();
    let parent_dependency = polytunnel_maven::PomDependency {
        group_id: "com.example".to_string(),
        artifact_id: "managed".to_string(),
        version: Some("${base.version}".to_string()),
        scope: DependencyScope::Compile,
        optional: false,
        exclusions: Vec::new(),
    };
    dm.push(parent_dependency);

    let child_dependency = polytunnel_maven::PomDependency {
        group_id: "com.example".to_string(),
        artifact_id: "managed".to_string(),
        version: None,
        scope: DependencyScope::Compile,
        optional: true,
        exclusions: Vec::new(),
    };
    pom.dependencies.push(child_dependency);

    pom.merge_dependency_management(dm);
    pom.fill_missing_versions();
    pom.merge_properties(&extra);

    assert_eq!(
        pom.dependencies.last().unwrap().version.as_deref(),
        Some("1.0.0")
    );
}
