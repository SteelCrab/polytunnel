package com.example.csvstats;

/**
 * Holds computed descriptive statistics for a numeric column.
 */
public class StatSummary {

    private final long count;
    private final double min;
    private final double max;
    private final double mean;
    private final double median;
    private final double stdDev;
    private final double q1;
    private final double q3;
    private final double sum;

    public StatSummary(long count, double min, double max, double mean,
                       double median, double stdDev, double q1, double q3, double sum) {
        this.count = count;
        this.min = min;
        this.max = max;
        this.mean = mean;
        this.median = median;
        this.stdDev = stdDev;
        this.q1 = q1;
        this.q3 = q3;
        this.sum = sum;
    }

    /**
     * Create an empty summary (for empty datasets).
     *
     * @return a summary with zero count and NaN values
     */
    public static StatSummary empty() {
        return new StatSummary(0, Double.NaN, Double.NaN, Double.NaN,
                Double.NaN, Double.NaN, Double.NaN, Double.NaN, 0.0);
    }

    public long getCount() { return count; }
    public double getMin() { return min; }
    public double getMax() { return max; }
    public double getMean() { return mean; }
    public double getMedian() { return median; }
    public double getStdDev() { return stdDev; }
    public double getQ1() { return q1; }
    public double getQ3() { return q3; }
    public double getSum() { return sum; }

    /**
     * Format the summary as a human-readable report.
     *
     * @param columnName the name of the column
     * @return formatted string
     */
    public String formatReport(String columnName) {
        if (count == 0) {
            return String.format("Column '%s': no data", columnName);
        }
        StringBuilder sb = new StringBuilder();
        sb.append(String.format("Column: %s%n", columnName));
        sb.append(String.format("  Count:     %d%n", count));
        sb.append(String.format("  Min:       %.4f%n", min));
        sb.append(String.format("  Max:       %.4f%n", max));
        sb.append(String.format("  Mean:      %.4f%n", mean));
        sb.append(String.format("  Median:    %.4f%n", median));
        sb.append(String.format("  Std Dev:   %.4f%n", stdDev));
        sb.append(String.format("  Q1 (25%%):  %.4f%n", q1));
        sb.append(String.format("  Q3 (75%%):  %.4f%n", q3));
        sb.append(String.format("  Sum:       %.4f%n", sum));
        return sb.toString();
    }

    @Override
    public String toString() {
        return String.format("StatSummary{count=%d, mean=%.4f, median=%.4f, stdDev=%.4f}",
                count, mean, median, stdDev);
    }
}
