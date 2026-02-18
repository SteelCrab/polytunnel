package com.example.configvalidator;

import com.google.gson.Gson;
import com.google.gson.GsonBuilder;
import org.yaml.snakeyaml.Yaml;

import java.util.Map;

/**
 * Loads configuration from YAML and exports to JSON.
 */
public class ConfigLoader {

    private final Yaml yaml;
    private final Gson gson;

    public ConfigLoader() {
        this.yaml = new Yaml();
        this.gson = new GsonBuilder().setPrettyPrinting().create();
    }

    /**
     * Parse a YAML string into a configuration map.
     *
     * @param yamlContent the YAML content
     * @return configuration as a Map
     * @throws IllegalArgumentException if the YAML content is not a mapping
     */
    @SuppressWarnings("unchecked")
    public Map<String, Object> fromYaml(String yamlContent) {
        Object loaded = yaml.load(yamlContent);
        if (loaded instanceof Map) {
            return (Map<String, Object>) loaded;
        }
        throw new IllegalArgumentException("YAML content must be a mapping, got: "
                + (loaded == null ? "null" : loaded.getClass().getSimpleName()));
    }

    /**
     * Serialize a configuration map to a pretty-printed JSON string.
     *
     * @param config the configuration map
     * @return JSON string
     */
    public String toJson(Map<String, Object> config) {
        return gson.toJson(config);
    }

    /**
     * Convert YAML content to JSON in one step.
     *
     * @param yamlContent the YAML input
     * @return JSON string
     */
    public String yamlToJson(String yamlContent) {
        Map<String, Object> config = fromYaml(yamlContent);
        return toJson(config);
    }
}
