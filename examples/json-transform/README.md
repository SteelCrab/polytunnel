# JSON Transform Example

A JSON data transformation tool demonstrating Jackson-based data processing with Polytunnel.

## Features

- **Jackson Databind** for JSON serialization and deserialization
- **Java Time Support** via Jackson JSR310 module (`LocalDate` handling)
- **Stream-based Filtering** with chained filter operations
- **Round-trip JSON Processing** (parse → transform → serialize)

## Structure

```
json-transform/
├── polytunnel.toml
├── src/
│   ├── main/java/com/example/jsontransform/
│   │   ├── Product.java          # POJO with @JsonProperty, LocalDate field
│   │   ├── ProductFilter.java    # Stream-based filter and sort operations
│   │   ├── JsonProcessor.java    # ObjectMapper with JavaTimeModule
│   │   └── TransformApp.java     # Main app demonstrating the pipeline
│   └── test/java/com/example/jsontransform/
│       ├── JsonProcessorTest.java
│       └── ProductFilterTest.java
└── target/                       # Build outputs (auto-generated)
```

## Usage

```bash
# Build and run tests
pt build

# Run tests only
pt test
```

## Dependencies

- **Jackson Databind 2.16.1** - JSON processing core
- **Jackson Annotations 2.16.1** - `@JsonProperty`, `@JsonIgnoreProperties`
- **Jackson Datatype JSR310 2.16.1** - Java 8+ date/time support (`LocalDate`)
- **JUnit 5.10.1** - Testing framework (test scope)
- **JUnit Platform Console 1.10.1** - Test runner (test scope)

## Learning Points

1. **Jackson ObjectMapper** configuration with `JavaTimeModule`
2. **TypeReference** for generic collection deserialization (`List<Product>`)
3. **`@JsonProperty`** for mapping Java field names to JSON keys (e.g., `inStock` → `in_stock`)
4. **`@JsonIgnoreProperties(ignoreUnknown = true)`** for forward-compatible parsing
5. **Java Streams** for filtering and sorting product lists
6. **Chained operations** for complex multi-step data queries
