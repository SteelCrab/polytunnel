package com.example.jsontransform;

import java.util.List;
import java.util.stream.Collectors;

/**
 * Applies filter and transformation operations to product lists.
 */
public class ProductFilter {

    /**
     * Filter products by category (case-insensitive).
     *
     * @param products the products to filter
     * @param category the category to match
     * @return filtered list
     */
    public List<Product> byCategory(List<Product> products, String category) {
        return products.stream()
                .filter(p -> p.getCategory().equalsIgnoreCase(category))
                .collect(Collectors.toList());
    }

    /**
     * Filter products that are in stock.
     *
     * @param products the products to filter
     * @return only in-stock products
     */
    public List<Product> inStockOnly(List<Product> products) {
        return products.stream()
                .filter(Product::isInStock)
                .collect(Collectors.toList());
    }

    /**
     * Filter products within a price range (inclusive).
     *
     * @param products the products to filter
     * @param minPrice minimum price (inclusive)
     * @param maxPrice maximum price (inclusive)
     * @return products within the price range
     */
    public List<Product> byPriceRange(List<Product> products, double minPrice, double maxPrice) {
        return products.stream()
                .filter(p -> p.getPrice() >= minPrice && p.getPrice() <= maxPrice)
                .collect(Collectors.toList());
    }

    /**
     * Sort products by price ascending.
     *
     * @param products the products to sort
     * @return sorted list (new list, original unchanged)
     */
    public List<Product> sortByPrice(List<Product> products) {
        return products.stream()
                .sorted((a, b) -> Double.compare(a.getPrice(), b.getPrice()))
                .collect(Collectors.toList());
    }

    /**
     * Calculate average price of a product list.
     *
     * @param products the products
     * @return the average price, or 0.0 if list is empty
     */
    public double averagePrice(List<Product> products) {
        return products.stream()
                .mapToDouble(Product::getPrice)
                .average()
                .orElse(0.0);
    }
}
