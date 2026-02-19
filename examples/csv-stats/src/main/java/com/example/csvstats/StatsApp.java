package com.example.csvstats;

import java.io.IOException;
import java.util.List;

/**
 * Main application demonstrating CSV parsing and statistical analysis.
 */
public class StatsApp {

    public static void main(String[] args) throws IOException {
        String csvData = """
                name,department,salary,years_experience,performance_score
                Alice,Engineering,95000,8,4.5
                Bob,Marketing,72000,5,3.8
                Carol,Engineering,105000,12,4.7
                Dave,Marketing,68000,3,3.2
                Eve,Engineering,88000,6,4.1
                Frank,Sales,78000,7,3.9
                Grace,Engineering,112000,15,4.9
                Hank,Sales,65000,2,3.0
                Iris,Marketing,82000,9,4.3
                Jack,Sales,71000,4,3.5
                """;

        CsvReader reader = new CsvReader();
        StatCalculator calculator = new StatCalculator();

        // Parse CSV data
        List<DataRecord> records = reader.parse(csvData);
        System.out.println("=== CSV Data Loaded ===");
        System.out.println("Records: " + records.size());
        System.out.println("Columns: " + records.get(0).columns());

        // Analyze numeric columns
        String[] numericColumns = {"salary", "years_experience", "performance_score"};
        System.out.println("\n=== Statistical Summary ===\n");

        for (String column : numericColumns) {
            double[] values = reader.extractColumn(records, column);
            StatSummary summary = calculator.summarize(values);
            System.out.println(summary.formatReport(column));
        }

        // Department breakdown
        System.out.println("=== Engineering Department ===\n");
        List<DataRecord> engineering = records.stream()
                .filter(r -> "Engineering".equals(r.get("department")))
                .toList();

        double[] engSalaries = reader.extractColumn(engineering, "salary");
        StatSummary engSummary = calculator.summarize(engSalaries);
        System.out.println(engSummary.formatReport("salary (Engineering)"));
    }
}
