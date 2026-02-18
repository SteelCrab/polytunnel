package com.example.csvstats;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

/**
 * Unit tests for DataRecord.
 */
class DataRecordTest {

    @Test
    void testPutAndGet() {
        DataRecord record = new DataRecord();
        record.put("name", "Alice");
        record.put("age", "30");
        assertEquals("Alice", record.get("name"));
        assertEquals("30", record.get("age"));
    }

    @Test
    void testGetDouble() {
        DataRecord record = new DataRecord();
        record.put("value", "42.5");
        assertEquals(42.5, record.getDouble("value"), 0.001);
    }

    @Test
    void testGetDoubleInvalidThrows() {
        DataRecord record = new DataRecord();
        record.put("value", "not-a-number");
        assertThrows(NumberFormatException.class, () -> record.getDouble("value"));
    }

    @Test
    void testGetDoubleMissingColumnThrows() {
        DataRecord record = new DataRecord();
        assertThrows(IllegalArgumentException.class, () -> record.getDouble("missing"));
    }

    @Test
    void testHasColumn() {
        DataRecord record = new DataRecord();
        record.put("name", "Alice");
        assertTrue(record.hasColumn("name"));
        assertFalse(record.hasColumn("age"));
    }

    @Test
    void testGetMissingColumn() {
        DataRecord record = new DataRecord();
        assertNull(record.get("nonexistent"));
    }

    @Test
    void testColumns() {
        DataRecord record = new DataRecord();
        record.put("a", "1");
        record.put("b", "2");
        record.put("c", "3");
        assertEquals(3, record.columns().size());
        assertTrue(record.columns().contains("a"));
        assertTrue(record.columns().contains("b"));
        assertTrue(record.columns().contains("c"));
    }
}
