# Config Validator Example

A configuration file validation library demonstrating SnakeYAML and Gson with Polytunnel.

## Features

- **SnakeYAML** for parsing YAML configuration files into typed Java maps
- **Gson** for exporting validated configs as pretty-printed JSON
- **Builder pattern** for constructing validators with chained rules
- **Multiple validation rules**: required, type check, range, pattern, one-of, and custom
- **Error accumulation** — all errors reported, not just the first

## Structure

```
config-validator/
├── polytunnel.toml
├── src/
│   ├── main/java/com/example/configvalidator/
│   │   ├── ValidationRule.java     # Functional interface for rules
│   │   ├── ValidationRules.java    # Factory: required, typeCheck, range, pattern, oneOf
│   │   ├── ValidationResult.java   # Immutable error list with formatted report
│   │   ├── ConfigValidator.java    # Builder pattern, accumulates and runs rules
│   │   ├── ConfigLoader.java       # YAML → Map, Map → JSON, YAML → JSON
│   │   └── ValidatorApp.java       # Main app: validate good/bad configs, export JSON
│   └── test/java/com/example/configvalidator/
│       ├── ConfigValidatorTest.java
│       ├── ConfigLoaderTest.java
│       └── ValidationResultTest.java
└── target/                         # Build outputs (auto-generated)
```

## Usage

```bash
# Build and run tests
pt build

# Run tests only
pt test
```

## Dependencies

- **SnakeYAML 2.2** - YAML parsing (zero transitive dependencies)
- **Gson 2.10.1** - JSON serialization (zero transitive dependencies)
- **JUnit 5.10.1** - Testing framework (test scope)
- **JUnit Platform Console 1.10.1** - Test runner (test scope)

## Learning Points

1. **SnakeYAML** `Yaml.load()` for parsing YAML into `Map<String, Object>`
2. **Gson** `GsonBuilder.setPrettyPrinting()` for readable JSON output
3. **Builder pattern** for fluent, chainable API construction
4. **Functional interfaces** for defining custom validation rules as lambdas
5. **Error accumulation** — run all rules, collect all errors (vs. fail-fast)
6. **YAML-to-JSON** format conversion in a single call
