# Todo API Server Example

A real-world example demonstrating a REST API server built with **Javalin** and **Jackson**, managed by Polytunnel.

## Features

- **Complex Dependencies**: Resolves deep dependency trees (Javalin → Jetty, Slf4j, Jackson, Kotlin Stdlib, etc.)
- **Web Server**: Runs a lightweight HTTP server on port 8080
- **JSON API**: Handles JSON serialization/deserialization automatically

## Structure

```
todo-server/
├── polytunnel.toml
├── src/
│   ├── main/java/com/example/todo/
│   │   ├── TodoApp.java        # Main Entry Point
│   │   ├── TodoController.java # API Logic
│   │   └── Todo.java           # Data Model
│   └── test/java/com/example/todo/
│       └── TodoTest.java       # Unit Tests
```

## Usage

### 1. Build
To resolve dependencies and compile the project, run:

```bash
pt build -v
```
You will see Polytunnel resolving dozens of transitive dependencies (Jetty, Kotlin, etc.) required by Javalin.

## Running

The recommended way to run the server is using **Gradle** (which wraps the build process and handles classpath correctly) or **VS Code**.

### Option 1: Using Gradle (Recommended via Command Line)

We have included a Gradle wrapper. You can build and run the server easily:

For details on how this was set up, see [GRADLE_GUIDE.md](GRADLE_GUIDE.md).

```bash
# Build
./gradlew build

# Run
./gradlew run
```

### Option 2: Using VS Code

1.  Open this folder (`examples/todo-server`) in VS Code.
2.  If you haven't already, run `pt vscode` in the `examples/todo-server` directory to generate the configuration files.
3.  Open `src/main/java/com/example/todo/TodoApp.java`.
4.  Press `F5` or click "Run".

> **Note on Manual Execution**: Running `java` manually is difficult because you must construct a classpath that includes all JARs in `.polytunnel/cache`. Using `pt`'s cache blindly (e.g. `find .polytunnel/cache ...`) is **not recommended** as it may include conflicting versions of libraries (e.g. SLF4J 1.7 vs 2.0). Gradle or VS Code handles this isolation correctly.

### 3. Run Tests
To execute unit tests:

```bash
pt test
```

## Dependencies
This example proves Polytunnel's robustness by handling:
- **io.javalin:javalin** (Simplicity-first web framework)
- **org.slf4j:slf4j-simple** (Logging)
- **com.fasterxml.jackson.core:jackson-databind** (JSON processing)
