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

### 1. Build
To resolve dependencies and compile the project, run:

```bash
pt build -v
```
You will see Polytunnel resolving dozens of transitive dependencies (Jetty, Kotlin, etc.) required by Javalin.

### 2. Run
#### Option A: Run via VS Code (Recommended)
This is the easiest way to run the application, as Polytunnel generates the necessary classpath configuration for you.

1. Generate VS Code configuration files:
   ```bash
   pt vscode
   ```
2. Open the `todo-server` folder in VS Code.
3. Open `src/main/java/com/example/todo/TodoApp.java`.
4. Click **Run** or **Debug** (provided by the "Extension Pack for Java").

#### Option B: Run Manually
Currently, `pt` does not produce a "fat jar" or a helper script to run easily from the CLI. You must construct the classpath manually pointing to your local Polytunnel cache (usually `~/.polytunnel/cache`), or rely on the IDE.

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
