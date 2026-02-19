# CSV Stats Example

A CSV data analysis tool demonstrating Apache Commons CSV and Commons Math with Polytunnel.

## Features

- **Apache Commons CSV** for robust CSV parsing with header support, trimming, and empty line skipping
- **Apache Commons Math** for descriptive statistics (mean, median, std dev, quartiles, sum)
- **Data pipeline pattern**: parse CSV → extract columns → compute stats → format report
- **Zero transitive dependencies**: both libraries are self-contained

## Structure

```
csv-stats/
├── polytunnel.toml
├── src/
│   ├── main/java/com/example/csvstats/
│   │   ├── DataRecord.java       # Row wrapper with typed column access
│   │   ├── CsvReader.java        # CSV parsing via Apache Commons CSV
│   │   ├── StatCalculator.java   # Statistics via Apache Commons Math
│   │   ├── StatSummary.java      # Immutable stats result with formatted report
│   │   └── StatsApp.java         # Main app: employee data analysis
│   └── test/java/com/example/csvstats/
│       ├── CsvReaderTest.java
│       ├── StatCalculatorTest.java
│       └── DataRecordTest.java
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

- **Commons CSV 1.10.0** - CSV file parsing with header auto-detection
- **Commons Math3 3.6.1** - `DescriptiveStatistics` for numerical analysis
- **JUnit 5.10.1** - Testing framework (test scope)
- **JUnit Platform Console 1.10.1** - Test runner (test scope)

## Learning Points

1. **`CSVFormat` builder** — fluent configuration for header parsing, trimming, empty line handling
2. **`DescriptiveStatistics`** — computing mean, median, percentiles, and standard deviation
3. **Clean data pipeline** — separate parsing, computation, and presentation layers
4. **Floating-point assertions** — using `assertEquals(expected, actual, delta)` for numeric tests
5. **Stream-based column extraction** — `mapToDouble` for converting records to numeric arrays
