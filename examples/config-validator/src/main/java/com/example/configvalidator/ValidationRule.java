package com.example.configvalidator;

import java.util.Map;
import java.util.Optional;

/**
 * A single validation rule that checks a configuration map.
 */
public interface ValidationRule {

    /**
     * Validate the configuration map.
     *
     * @param config the configuration key-value map
     * @return empty if valid, or an error message if invalid
     */
    Optional<String> validate(Map<String, Object> config);
}
