package com.example.configvalidator;

import java.util.Map;

/**
 * Main application demonstrating YAML configuration loading and validation.
 */
public class ValidatorApp {

    public static void main(String[] args) {
        ConfigLoader loader = new ConfigLoader();

        // Define validation rules for a "server config" schema
        ConfigValidator validator = ConfigValidator.builder()
                .required("host")
                .required("port")
                .typeCheck("port", Integer.class)
                .range("port", 1, 65535)
                .required("environment")
                .oneOf("environment", "development", "staging", "production")
                .pattern("host", "^[a-zA-Z0-9.-]+$", "a valid hostname")
                .range("max_connections", 1, 10000)
                .build();

        // --- Valid configuration ---
        String validYaml = """
                host: api.example.com
                port: 8080
                environment: production
                max_connections: 500
                debug: false
                features:
                  - auth
                  - logging
                  - metrics
                """;

        System.out.println("=== Valid Configuration ===");
        Map<String, Object> validConfig = loader.fromYaml(validYaml);
        ValidationResult validResult = validator.validate(validConfig);
        System.out.println(validResult.formatReport());

        // Export as JSON
        System.out.println("JSON export:");
        System.out.println(loader.toJson(validConfig));

        // --- Invalid configuration ---
        String invalidYaml = """
                port: 99999
                environment: local
                max_connections: -5
                """;

        System.out.println("\n=== Invalid Configuration ===");
        Map<String, Object> invalidConfig = loader.fromYaml(invalidYaml);
        ValidationResult invalidResult = validator.validate(invalidConfig);
        System.out.println(invalidResult.formatReport());

        // --- YAML to JSON conversion ---
        String appYaml = """
                database:
                  driver: postgresql
                  host: db.example.com
                  port: 5432
                  name: myapp
                cache:
                  enabled: true
                  ttl: 300
                """;

        System.out.println("=== YAML to JSON Conversion ===");
        System.out.println(loader.yamlToJson(appYaml));
    }
}
