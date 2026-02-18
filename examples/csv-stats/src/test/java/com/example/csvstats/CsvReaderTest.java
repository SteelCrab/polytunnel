package com.example.csvstats;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

import java.io.IOException;
import java.util.List;

/**
 * Unit tests for CsvReader.
 */
class CsvReaderTest {

    private final CsvReader reader = new CsvReader();

    @Test
    void testParseBasicCsv() throws IOException {
        String csv = """
                name,age,score
                Alice,30,95.5
                Bob,25,87.3
                """;
        List<DataRecord> records = reader.parse(csv);
        assertEquals(2, records.size());
        assertEquals("Alice", records.get(0).get("name"));
        assertEquals("25", records.get(1).get("age"));
    }

    @Test
    void testParseTrimWhitespace() throws IOException {
        String csv = """
                name , value
                Alice , 100
                Bob , 200
                """;
        List<DataRecord> records = reader.parse(csv);
        assertEquals("Alice", records.get(0).get("name"));
        assertEquals("100", records.get(0).get("value"));
    }

    @Test
    void testExtractNumericColumn() throws IOException {
        String csv = """
                item,price
                A,10.5
                B,20.0
                C,30.75
                """;
        List<DataRecord> records = reader.parse(csv);
        double[] prices = reader.extractColumn(records, "price");
        assertEquals(3, prices.length);
        assertEquals(10.5, prices[0], 0.001);
        assertEquals(20.0, prices[1], 0.001);
        assertEquals(30.75, prices[2], 0.001);
    }

    @Test
    void testParseSkipsEmptyLines() throws IOException {
        String csv = """
                name,value

                Alice,100

                Bob,200
                """;
        List<DataRecord> records = reader.parse(csv);
        assertEquals(2, records.size());
    }

    @Test
    void testParseSingleRow() throws IOException {
        String csv = """
                col1,col2
                hello,world
                """;
        List<DataRecord> records = reader.parse(csv);
        assertEquals(1, records.size());
        assertEquals("hello", records.get(0).get("col1"));
        assertEquals("world", records.get(0).get("col2"));
    }
}
