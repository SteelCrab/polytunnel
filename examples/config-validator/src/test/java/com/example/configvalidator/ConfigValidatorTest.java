package com.example.configvalidator;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

import java.util.HashMap;
import java.util.Map;
import java.util.Optional;

/**
 * Unit tests for ConfigValidator.
 */
class ConfigValidatorTest {

    @Test
    void testValidConfig() {
        ConfigValidator validator = ConfigValidator.builder()
                .required("name")
                .required("port")
                .build();

        Map<String, Object> config = Map.of("name", "myapp", "port", 8080);
        ValidationResult result = validator.validate(config);
        assertTrue(result.isValid());
        assertEquals(0, result.errorCount());
    }

    @Test
    void testMissingRequiredField() {
        ConfigValidator validator = ConfigValidator.builder()
                .required("name")
                .required("port")
                .build();

        Map<String, Object> config = Map.of("name", "myapp");
        ValidationResult result = validator.validate(config);
        assertFalse(result.isValid());
        assertEquals(1, result.errorCount());
        assertTrue(result.getErrors().get(0).contains("port"));
    }

    @Test
    void testMultipleErrors() {
        ConfigValidator validator = ConfigValidator.builder()
                .required("name")
                .required("host")
                .required("port")
                .build();

        Map<String, Object> config = Map.of("name", "myapp");
        ValidationResult result = validator.validate(config);
        assertFalse(result.isValid());
        assertEquals(2, result.errorCount());
    }

    @Test
    void testTypeCheckPass() {
        ConfigValidator validator = ConfigValidator.builder()
                .typeCheck("port", Integer.class)
                .build();

        Map<String, Object> config = Map.of("port", 8080);
        ValidationResult result = validator.validate(config);
        assertTrue(result.isValid());
    }

    @Test
    void testTypeCheckFail() {
        ConfigValidator validator = ConfigValidator.builder()
                .typeCheck("port", Integer.class)
                .build();

        Map<String, Object> config = Map.of("port", "not-a-number");
        ValidationResult result = validator.validate(config);
        assertFalse(result.isValid());
        assertTrue(result.getErrors().get(0).contains("Integer"));
    }

    @Test
    void testRangePass() {
        ConfigValidator validator = ConfigValidator.builder()
                .range("port", 1, 65535)
                .build();

        Map<String, Object> config = Map.of("port", 8080);
        ValidationResult result = validator.validate(config);
        assertTrue(result.isValid());
    }

    @Test
    void testRangeFail() {
        ConfigValidator validator = ConfigValidator.builder()
                .range("port", 1, 65535)
                .build();

        Map<String, Object> config = Map.of("port", 99999);
        ValidationResult result = validator.validate(config);
        assertFalse(result.isValid());
        assertTrue(result.getErrors().get(0).contains("between"));
    }

    @Test
    void testOneOfPass() {
        ConfigValidator validator = ConfigValidator.builder()
                .oneOf("env", "dev", "staging", "prod")
                .build();

        Map<String, Object> config = Map.of("env", "prod");
        ValidationResult result = validator.validate(config);
        assertTrue(result.isValid());
    }

    @Test
    void testOneOfFail() {
        ConfigValidator validator = ConfigValidator.builder()
                .oneOf("env", "dev", "staging", "prod")
                .build();

        Map<String, Object> config = Map.of("env", "local");
        ValidationResult result = validator.validate(config);
        assertFalse(result.isValid());
        assertTrue(result.getErrors().get(0).contains("one of"));
    }

    @Test
    void testPatternPass() {
        ConfigValidator validator = ConfigValidator.builder()
                .pattern("email", "^.+@.+\\..+$", "a valid email")
                .build();

        Map<String, Object> config = Map.of("email", "user@example.com");
        ValidationResult result = validator.validate(config);
        assertTrue(result.isValid());
    }

    @Test
    void testPatternFail() {
        ConfigValidator validator = ConfigValidator.builder()
                .pattern("email", "^.+@.+\\..+$", "a valid email")
                .build();

        Map<String, Object> config = Map.of("email", "not-an-email");
        ValidationResult result = validator.validate(config);
        assertFalse(result.isValid());
        assertTrue(result.getErrors().get(0).contains("must match"));
    }

    @Test
    void testNullValueForRequired() {
        ConfigValidator validator = ConfigValidator.builder()
                .required("name")
                .build();

        Map<String, Object> config = new HashMap<>();
        config.put("name", null);
        ValidationResult result = validator.validate(config);
        assertFalse(result.isValid());
    }

    @Test
    void testEmptyRulesAlwaysValid() {
        ConfigValidator validator = ConfigValidator.builder().build();
        ValidationResult result = validator.validate(Map.of());
        assertTrue(result.isValid());
    }

    @Test
    void testCustomRule() {
        ConfigValidator validator = ConfigValidator.builder()
                .addRule(config -> {
                    Object min = config.get("min");
                    Object max = config.get("max");
                    if (min instanceof Number && max instanceof Number) {
                        if (((Number) min).doubleValue() > ((Number) max).doubleValue()) {
                            return Optional.of("'min' must be less than or equal to 'max'");
                        }
                    }
                    return Optional.empty();
                })
                .build();

        Map<String, Object> config = Map.of("min", 100, "max", 50);
        ValidationResult result = validator.validate(config);
        assertFalse(result.isValid());
        assertTrue(result.getErrors().get(0).contains("min"));
    }
}
