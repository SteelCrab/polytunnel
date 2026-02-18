package com.example.configvalidator;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

import java.util.List;

/**
 * Unit tests for ValidationResult.
 */
class ValidationResultTest {

    @Test
    void testValidResult() {
        ValidationResult result = new ValidationResult(List.of());
        assertTrue(result.isValid());
        assertEquals(0, result.errorCount());
        assertTrue(result.getErrors().isEmpty());
    }

    @Test
    void testInvalidResult() {
        ValidationResult result = new ValidationResult(List.of("Error 1", "Error 2"));
        assertFalse(result.isValid());
        assertEquals(2, result.errorCount());
    }

    @Test
    void testFormatReportValid() {
        ValidationResult result = new ValidationResult(List.of());
        assertEquals("Configuration is valid.", result.formatReport());
    }

    @Test
    void testFormatReportWithErrors() {
        ValidationResult result = new ValidationResult(List.of("Missing field: name"));
        String report = result.formatReport();
        assertTrue(report.contains("1 error(s)"));
        assertTrue(report.contains("Missing field: name"));
    }

    @Test
    void testErrorsAreUnmodifiable() {
        ValidationResult result = new ValidationResult(List.of("Error"));
        assertThrows(UnsupportedOperationException.class, () -> result.getErrors().add("another"));
    }
}
