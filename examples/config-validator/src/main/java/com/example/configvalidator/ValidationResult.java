package com.example.configvalidator;

import java.util.Collections;
import java.util.List;

/**
 * Result of validating a configuration, containing all errors found.
 */
public class ValidationResult {

    private final List<String> errors;

    public ValidationResult(List<String> errors) {
        this.errors = Collections.unmodifiableList(errors);
    }

    /**
     * Check if the configuration is valid (no errors).
     *
     * @return true if no validation errors were found
     */
    public boolean isValid() {
        return errors.isEmpty();
    }

    /**
     * Get all validation errors.
     *
     * @return unmodifiable list of error messages
     */
    public List<String> getErrors() {
        return errors;
    }

    /**
     * Get the number of errors.
     *
     * @return error count
     */
    public int errorCount() {
        return errors.size();
    }

    /**
     * Format all errors as a human-readable report.
     *
     * @return formatted error report, or "Valid" if no errors
     */
    public String formatReport() {
        if (isValid()) {
            return "Configuration is valid.";
        }
        StringBuilder sb = new StringBuilder();
        sb.append(String.format("Configuration has %d error(s):%n", errors.size()));
        for (int i = 0; i < errors.size(); i++) {
            sb.append(String.format("  %d. %s%n", i + 1, errors.get(i)));
        }
        return sb.toString();
    }

    @Override
    public String toString() {
        return isValid() ? "ValidationResult{valid}" : "ValidationResult{errors=" + errors.size() + "}";
    }
}
