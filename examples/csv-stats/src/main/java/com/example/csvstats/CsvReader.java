package com.example.csvstats;

import org.apache.commons.csv.CSVFormat;
import org.apache.commons.csv.CSVParser;
import org.apache.commons.csv.CSVRecord;

import java.io.IOException;
import java.io.StringReader;
import java.util.ArrayList;
import java.util.List;

/**
 * Reads CSV data into DataRecord objects using Apache Commons CSV.
 */
public class CsvReader {

    /**
     * Parse CSV content with headers into a list of DataRecords.
     *
     * @param csvContent the CSV string (first row is headers)
     * @return list of data records
     * @throws IOException if parsing fails
     */
    public List<DataRecord> parse(String csvContent) throws IOException {
        List<DataRecord> records = new ArrayList<>();
        CSVFormat format = CSVFormat.DEFAULT.builder()
                .setHeader()
                .setSkipHeaderRecord(true)
                .setTrim(true)
                .setIgnoreEmptyLines(true)
                .build();

        try (CSVParser parser = format.parse(new StringReader(csvContent))) {
            List<String> headers = parser.getHeaderNames();
            for (CSVRecord csvRecord : parser) {
                DataRecord record = new DataRecord();
                for (String header : headers) {
                    record.put(header, csvRecord.get(header));
                }
                records.add(record);
            }
        }
        return records;
    }

    /**
     * Extract a numeric column from parsed records.
     *
     * @param records the data records
     * @param column the column name
     * @return array of double values
     * @throws NumberFormatException if any value is not numeric
     */
    public double[] extractColumn(List<DataRecord> records, String column) {
        return records.stream()
                .mapToDouble(r -> r.getDouble(column))
                .toArray();
    }
}
