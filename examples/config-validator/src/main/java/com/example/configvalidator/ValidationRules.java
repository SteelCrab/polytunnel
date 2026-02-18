package com.example.configvalidator;

import java.util.Map;
import java.util.Optional;

/**
 * Factory methods for common validation rules.
 */
public final class ValidationRules {

    private ValidationRules() {
        // utility class
    }

    /**
     * Require that a key exists and is non-null.
     *
     * @param key the required key
     * @return a validation rule
     */
    public static ValidationRule required(String key) {
        return config -> {
            if (!config.containsKey(key) || config.get(key) == null) {
                return Optional.of("Missing required field: '" + key + "'");
            }
            return Optional.empty();
        };
    }

    /**
     * Require that a key, if present, has a value of the expected type.
     *
     * @param key the key to check
     * @param expectedType the expected Java type
     * @return a validation rule
     */
    public static ValidationRule typeCheck(String key, Class<?> expectedType) {
        return config -> {
            Object value = config.get(key);
            if (value != null && !expectedType.isInstance(value)) {
                return Optional.of("Field '" + key + "' must be of type "
                        + expectedType.getSimpleName() + ", got " + value.getClass().getSimpleName());
            }
            return Optional.empty();
        };
    }

    /**
     * Require that a numeric field is within a range (inclusive).
     *
     * @param key the key to check
     * @param min minimum value (inclusive)
     * @param max maximum value (inclusive)
     * @return a validation rule
     */
    public static ValidationRule range(String key, double min, double max) {
        return config -> {
            Object value = config.get(key);
            if (value instanceof Number number) {
                double d = number.doubleValue();
                if (d < min || d > max) {
                    return Optional.of("Field '" + key + "' must be between "
                            + min + " and " + max + ", got " + d);
                }
            }
            return Optional.empty();
        };
    }

    /**
     * Require that a string field matches a regex pattern.
     *
     * @param key the key to check
     * @param pattern the regex pattern
     * @param description human-readable description of the expected format
     * @return a validation rule
     */
    public static ValidationRule pattern(String key, String pattern, String description) {
        return config -> {
            Object value = config.get(key);
            if (value instanceof String str) {
                if (!str.matches(pattern)) {
                    return Optional.of("Field '" + key + "' must match " + description
                            + ", got '" + str + "'");
                }
            }
            return Optional.empty();
        };
    }

    /**
     * Require that a string field is one of the allowed values.
     *
     * @param key the key to check
     * @param allowedValues the allowed values
     * @return a validation rule
     */
    public static ValidationRule oneOf(String key, String... allowedValues) {
        return config -> {
            Object value = config.get(key);
            if (value instanceof String str) {
                for (String allowed : allowedValues) {
                    if (allowed.equals(str)) {
                        return Optional.empty();
                    }
                }
                return Optional.of("Field '" + key + "' must be one of ["
                        + String.join(", ", allowedValues) + "], got '" + str + "'");
            }
            return Optional.empty();
        };
    }
}
