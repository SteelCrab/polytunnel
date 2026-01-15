# Polytunnel Examples

Example projects demonstrating Polytunnel features.

## Available Examples

| Example | Description |
|---------|-------------|
| [hello-java](hello-java/) | Simple Java project with Guava and JUnit 5 |

## Usage

```bash
cd examples/hello-java
pt build
pt test
```

## Creating a New Example

1. Create a new directory: `mkdir my-example`
2. Initialize: `cd my-example && pt init my-example`
3. Add source files to `src/main/java/`
4. Add tests to `src/test/java/`
5. Run: `pt build`
