package com.example.configvalidator;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

import java.util.Map;

/**
 * Unit tests for ConfigLoader.
 */
class ConfigLoaderTest {

    private final ConfigLoader loader = new ConfigLoader();

    @Test
    void testParseSimpleYaml() {
        String yaml = """
                name: myapp
                port: 8080
                debug: true
                """;
        Map<String, Object> config = loader.fromYaml(yaml);
        assertEquals("myapp", config.get("name"));
        assertEquals(8080, config.get("port"));
        assertEquals(true, config.get("debug"));
    }

    @Test
    void testParseNestedYaml() {
        String yaml = """
                database:
                  host: localhost
                  port: 5432
                """;
        Map<String, Object> config = loader.fromYaml(yaml);
        assertTrue(config.get("database") instanceof Map);
        @SuppressWarnings("unchecked")
        Map<String, Object> db = (Map<String, Object>) config.get("database");
        assertEquals("localhost", db.get("host"));
        assertEquals(5432, db.get("port"));
    }

    @Test
    void testToJson() {
        Map<String, Object> config = Map.of("name", "myapp", "port", 8080);
        String json = loader.toJson(config);
        assertTrue(json.contains("\"name\""));
        assertTrue(json.contains("\"myapp\""));
        assertTrue(json.contains("\"port\""));
    }

    @Test
    void testYamlToJsonRoundTrip() {
        String yaml = """
                host: example.com
                port: 443
                """;
        String json = loader.yamlToJson(yaml);
        assertTrue(json.contains("example.com"));
        assertTrue(json.contains("443"));
    }

    @Test
    void testInvalidYamlThrows() {
        // A plain scalar (not a mapping)
        assertThrows(IllegalArgumentException.class, () -> loader.fromYaml("just a string"));
    }
}
