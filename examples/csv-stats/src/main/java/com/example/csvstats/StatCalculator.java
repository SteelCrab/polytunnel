package com.example.csvstats;

import org.apache.commons.math3.stat.descriptive.DescriptiveStatistics;

/**
 * Computes descriptive statistics using Apache Commons Math.
 */
public class StatCalculator {

    /**
     * Compute a full statistical summary for a data array.
     *
     * @param values the numeric data
     * @return a StatSummary with computed statistics
     */
    public StatSummary summarize(double[] values) {
        if (values == null || values.length == 0) {
            return StatSummary.empty();
        }

        DescriptiveStatistics stats = new DescriptiveStatistics();
        for (double v : values) {
            stats.addValue(v);
        }

        return new StatSummary(
                stats.getN(),
                stats.getMin(),
                stats.getMax(),
                stats.getMean(),
                stats.getPercentile(50),
                stats.getStandardDeviation(),
                stats.getPercentile(25),
                stats.getPercentile(75),
                stats.getSum()
        );
    }
}
