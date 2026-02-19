package com.example.configvalidator;

import java.util.ArrayList;
import java.util.List;
import java.util.Map;
import java.util.Optional;

/**
 * Validates a configuration map against a set of rules.
 * Uses the Builder pattern to accumulate rules.
 */
public class ConfigValidator {

    private final List<ValidationRule> rules;

    private ConfigValidator(List<ValidationRule> rules) {
        this.rules = rules;
    }

    /**
     * Create a new validator builder.
     *
     * @return a new Builder instance
     */
    public static Builder builder() {
        return new Builder();
    }

    /**
     * Validate a configuration map against all registered rules.
     * All rules are checked; errors are accumulated (not short-circuited).
     *
     * @param config the configuration map to validate
     * @return validation result containing all errors found
     */
    public ValidationResult validate(Map<String, Object> config) {
        List<String> errors = new ArrayList<>();
        for (ValidationRule rule : rules) {
            Optional<String> error = rule.validate(config);
            error.ifPresent(errors::add);
        }
        return new ValidationResult(errors);
    }

    /**
     * Builder for constructing a ConfigValidator with a chain of rules.
     */
    public static class Builder {

        private final List<ValidationRule> rules = new ArrayList<>();

        /**
         * Add a custom validation rule.
         *
         * @param rule the rule to add
         * @return this builder
         */
        public Builder addRule(ValidationRule rule) {
            rules.add(rule);
            return this;
        }

        /**
         * Add a required-field rule.
         *
         * @param key the required key
         * @return this builder
         */
        public Builder required(String key) {
            rules.add(ValidationRules.required(key));
            return this;
        }

        /**
         * Add a type-check rule.
         *
         * @param key the key to check
         * @param type the expected type
         * @return this builder
         */
        public Builder typeCheck(String key, Class<?> type) {
            rules.add(ValidationRules.typeCheck(key, type));
            return this;
        }

        /**
         * Add a numeric range rule.
         *
         * @param key the key to check
         * @param min minimum value
         * @param max maximum value
         * @return this builder
         */
        public Builder range(String key, double min, double max) {
            rules.add(ValidationRules.range(key, min, max));
            return this;
        }

        /**
         * Add a regex pattern rule.
         *
         * @param key the key to check
         * @param pattern the regex pattern
         * @param description human-readable format description
         * @return this builder
         */
        public Builder pattern(String key, String pattern, String description) {
            rules.add(ValidationRules.pattern(key, pattern, description));
            return this;
        }

        /**
         * Add a one-of (enum) rule.
         *
         * @param key the key to check
         * @param allowedValues the allowed values
         * @return this builder
         */
        public Builder oneOf(String key, String... allowedValues) {
            rules.add(ValidationRules.oneOf(key, allowedValues));
            return this;
        }

        /**
         * Build the ConfigValidator.
         *
         * @return configured ConfigValidator instance
         */
        public ConfigValidator build() {
            return new ConfigValidator(new ArrayList<>(rules));
        }
    }
}
