# Todo API Server Example

A real-world example demonstrating a REST API server built with **Javalin** and **Jackson**, managed by Polytunnel.

## Features

- **Complex Dependencies**: Resolves deep dependency trees (Javalin → Jetty, Slf4j, Jackson, Kotlin Stdlib, etc.)
- **Web Server**: Runs a lightweight HTTP server on port 7000
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

### 1. Build and Run Tests
```bash
pt build -v
```
You will see Polytunnel resolving dozens of transitive dependencies (Jetty, Kotlin, etc.) required by Javalin.

### 2. Run the Server
(Note: Polytunnel doesn't have a `run` command yet, so use `java` directly with the classpath)

```bash
# Simulating run (manual classpath construction is hard, future feature!)
# For now, this example mainly checks build success and test passing.
```

## Dependencies
This example proves Polytunnel's robustness by handling:
- **io.javalin:javalin** (Simplicity-first web framework)
- **org.slf4j:slf4j-simple** (Logging)
- **com.fasterxml.jackson.core:jackson-databind** (JSON processing)
