//! Tests for POM parser

use polytunnel_maven::parse_pom;

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
