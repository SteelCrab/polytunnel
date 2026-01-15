# Hello Java Example

A simple Java project demonstrating Polytunnel's build and test capabilities.

## Structure

```
hello-java/
├── polytunnel.toml              # Project configuration
├── src/
│   ├── main/java/com/example/
│   │   └── App.java             # Main application class
│   └── test/java/com/example/
│       └── AppTest.java         # JUnit 5 unit tests
└── target/                      # Build outputs (auto-generated)
    ├── classes/                 # Compiled main classes
    └── test-classes/            # Compiled test classes
```

## Usage

```bash
# Build and run tests
pt build

# Run tests only
pt test

# Verbose output
pt build -v

# Clean build
pt build --clean
```

## Dependencies

- **Apache Commons Lang 3.14.0** - Utility library
  - Used in `App.greet()` for string utility
    
- **JUnit 5 (Jupiter) 5.10.1** - Modern testing framework (test scope only)
  - 5 unit tests demonstrating various test cases

- **JUnit Platform Console 1.10.1** - Test runner (test scope only)
  - Required for executing tests via CLI

## App.java

The `App` class demonstrates:
- Importing external dependencies (Commons Lang)
- Simple string manipulation with utility functions
- Clean, readable code structure

```java
package com.example;
import org.apache.commons.lang3.StringUtils;

public class App {
    public static void main(String[] args) {
        App app = new App();
        System.out.println(app.greet("World"));
    }

    public String greet(String name) {
        if (StringUtils.isBlank(name)) {
            return "Hello, Guest!";
        }
        return "Hello, " + name + "!";
    }

    public String repeat(String text, int count) {
        return StringUtils.repeat(text, count);
    }
}
```

## AppTest.java

The `AppTest` class contains 5 unit tests:

1. **testGreetWithName()** - Normal input case
2. **testGreetWithNull()** - Null handling
3. **testGreetWithEmpty()** - Empty string handling
4. **testRepeat()** - Normal string repetition
5. **testRepeatZero()** - Edge case (zero repetitions)

All tests use JUnit 5's `@Test` annotation and assertion methods from `org.junit.jupiter.api.Assertions`.

## How It Works

### 1. Configuration (polytunnel.toml)

```toml
[project]
name = "hello-java"
java_version = "17"

[build]
source_dirs = ["src/main/java"]
test_source_dirs = ["src/test/java"]
output_dir = "target/classes"
test_output_dir = "target/test-classes"

[dependencies]
"org.apache.commons:commons-lang3" = "3.14.0"

[dependencies."org.junit.jupiter:junit-jupiter"]
version = "5.10.1"
scope = "test"

[dependencies."org.junit.platform:junit-platform-console-standalone"]
version = "1.10.1"
scope = "test"
```

### 2. Build Process (pt build)

```
pt build -v
├── Resolve dependencies → Downloads Commons Lang, JUnit, etc.
├── Compile src/main/java → target/classes/
├── Compile src/test/java → target/test-classes/
├── Run tests using JUnit 5
└── Display summary
```

### 3. Test Execution (pt test)

```
pt test
├── Compile test sources (if needed)
├── Detect JUnit 5 from classpath
├── Execute tests
└── Report pass/fail results
```

## Expected Output

### CLI Output (Full Process)

```
$ pt build -v
   Resolving dependencies
   Downloading org.junit.jupiter:junit-jupiter:5.10.1
   ... (download logs) ...
   Compiling hello-java v0.1.0
    Finished dev [unoptimized + debuginfo] target(s) in 1.19s
   Compiling hello-java v0.1.0 (test)

     Running unittests (target/test-classes)
     Testing hello-java ...
Detected test framework: JUnit 5
[JUnit Tree Output details...]

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.41s
```

============================================================
Build Summary:
============================================================
Compiled: 2 files
Time: X.XXs

------------------------------------------------------------
Test Summary:
------------------------------------------------------------
Total: 5
Passed: 5
Failed: 0
Skipped: 0
============================================================
```

### Test Output

```
============================================================
Test Results:
============================================================
Total: 5
Passed: 5 (100%)
Failed: 0
Skipped: 0
Time: X.XXs
============================================================
```

## Learning Points

This example demonstrates:

1. **Maven Standard Structure** - Polytunnel follows `src/main/java` and `src/test/java` conventions
2. **Dependency Management** - Simple TOML format for declaring dependencies
3. **Scope Handling** - Test dependencies are excluded from production builds
4. **Test Framework Detection** - Polytunnel automatically detects JUnit 5
5. **Incremental Builds** - Subsequent builds are faster with caching
6. **Clean Error Messages** - Clear feedback on compilation and test failures

## Troubleshooting

### Compilation fails: "cannot find symbol"
- Check that dependencies are correctly listed in `polytunnel.toml`
- Ensure Java files are in the correct package structure

### Tests don't run
- Make sure test classes end with `Test` suffix (e.g., `AppTest.java`)
- Check that JUnit 5 dependency is added with `scope = "test"`

### Build is slow first time
- First build downloads dependencies from Maven Central
- Subsequent builds use cached JARs and are much faster
- Use `pt build --clean` only when necessary
