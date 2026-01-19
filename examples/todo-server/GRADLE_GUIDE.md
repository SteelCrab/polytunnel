# Gradle Setup Guide for Todo Server

This guide explains how to set up Gradle for the `todo-server` example. While Polytunnel (`pt`) handles the build logic natively, using Gradle can be beneficial for specific tasks like running the application with a stable classpath or integrating with other tools.

## 1. Create `build.gradle`

Create a `build.gradle` file in the project root with the following content. This configuration mirrors the dependencies defined in `polytunnel.toml`.

```groovy
plugins {
    id 'application'
    id 'java'
}

group = 'com.example'
version = '0.1.0'

repositories {
    mavenCentral()
}

dependencies {
    // Web Framework
    implementation 'io.javalin:javalin:6.1.3'
    
    // Javalin's runtime dependencies (same as known transitive deps)
    implementation 'org.slf4j:slf4j-simple:2.0.12'
    implementation 'com.fasterxml.jackson.core:jackson-databind:2.16.1'
    implementation 'org.jetbrains.kotlin:kotlin-stdlib:1.9.22'

    // Test Dependencies
    testImplementation 'org.junit.jupiter:junit-jupiter:5.10.1'
    testRuntimeOnly 'org.junit.platform:junit-platform-launcher'
}

application {
    // Main class entry point
    mainClass = 'com.example.todo.TodoApp'
}

test {
    useJUnitPlatform()
}
```

## 2. Generate Gradle Wrapper

If you have Gradle installed globally, run the following command to generate the local wrapper scripts (`gradlew`). This ensures everyone uses the same Gradle version.

```bash
gradle wrapper
```

This will create:
- `gradlew` (Unix script)
- `gradlew.bat` (Windows script)
- `gradle/wrapper/` (Wrapper JAR and properties)

## 3. Configure `.gitignore`

Ensure you ignore Gradle build artifacts but keep the wrapper files. Add these lines to your `.gitignore`:

```gitignore
.gradle/
build/
```

## 4. Usage

Now you can build and run the project using the wrapper:

### Build
```bash
./gradlew build
```

### Run
```bash
./gradlew run
```

This effectively runs the application with the correct classpath, avoiding version conflicts (like conflicting SLF4J bindings) that can occur when manually constructing classpaths from the cache.
