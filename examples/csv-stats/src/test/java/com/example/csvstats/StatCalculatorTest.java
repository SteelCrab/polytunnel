package com.example.csvstats;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

/**
 * Unit tests for StatCalculator.
 */
class StatCalculatorTest {

    private final StatCalculator calculator = new StatCalculator();

    @Test
    void testBasicStatistics() {
        double[] data = {10.0, 20.0, 30.0, 40.0, 50.0};
        StatSummary summary = calculator.summarize(data);
        assertEquals(5, summary.getCount());
        assertEquals(10.0, summary.getMin(), 0.001);
        assertEquals(50.0, summary.getMax(), 0.001);
        assertEquals(30.0, summary.getMean(), 0.001);
        assertEquals(30.0, summary.getMedian(), 0.001);
        assertEquals(150.0, summary.getSum(), 0.001);
    }

    @Test
    void testStandardDeviation() {
        double[] data = {2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0};
        StatSummary summary = calculator.summarize(data);
        assertEquals(5.0, summary.getMean(), 0.001);
        assertEquals(2.0, summary.getStdDev(), 0.01);
    }

    @Test
    void testSingleValue() {
        double[] data = {42.0};
        StatSummary summary = calculator.summarize(data);
        assertEquals(1, summary.getCount());
        assertEquals(42.0, summary.getMin(), 0.001);
        assertEquals(42.0, summary.getMax(), 0.001);
        assertEquals(42.0, summary.getMean(), 0.001);
        assertEquals(42.0, summary.getMedian(), 0.001);
    }

    @Test
    void testEmptyArray() {
        StatSummary summary = calculator.summarize(new double[0]);
        assertEquals(0, summary.getCount());
        assertTrue(Double.isNaN(summary.getMean()));
        assertTrue(Double.isNaN(summary.getMedian()));
    }

    @Test
    void testNullArray() {
        StatSummary summary = calculator.summarize(null);
        assertEquals(0, summary.getCount());
    }

    @Test
    void testQuartiles() {
        double[] data = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12};
        StatSummary summary = calculator.summarize(data);
        assertEquals(6.5, summary.getMean(), 0.001);
        assertEquals(6.5, summary.getMedian(), 0.5);
        assertTrue(summary.getQ1() >= 2.5 && summary.getQ1() <= 4.0);
        assertTrue(summary.getQ3() >= 9.0 && summary.getQ3() <= 10.5);
    }
}
