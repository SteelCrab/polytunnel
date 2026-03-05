package com.example.csvstats;

import java.util.LinkedHashMap;
import java.util.Map;
import java.util.Set;

/**
 * Represents a single row of CSV data with named columns.
 */
public class DataRecord {

    private final Map<String, String> fields;

    public DataRecord() {
        this.fields = new LinkedHashMap<>();
    }

    /**
     * Set a field value by column name.
     *
     * @param column the column name
     * @param value the string value
     */
    public void put(String column, String value) {
        fields.put(column, value);
    }

    /**
     * Get a field value as a string.
     *
     * @param column the column name
     * @return the string value, or null if not present
     */
    public String get(String column) {
        return fields.get(column);
    }

    /**
     * Get a field value as a double.
     *
     * @param column the column name
     * @return the numeric value
     * @throws NumberFormatException if the value is not a valid number
     * @throws IllegalArgumentException if the column does not exist
     */
    public double getDouble(String column) {
        String value = fields.get(column);
        if (value == null) {
            throw new IllegalArgumentException("Column not found: " + column);
        }
        return Double.parseDouble(value.trim());
    }

    /**
     * Check if the record contains a given column.
     *
     * @param column the column name
     * @return true if the column exists
     */
    public boolean hasColumn(String column) {
        return fields.containsKey(column);
    }

    /**
     * Get all column names.
     *
     * @return the column names in insertion order
     */
    public Set<String> columns() {
        return fields.keySet();
    }

    @Override
    public String toString() {
        return fields.toString();
    }
}
