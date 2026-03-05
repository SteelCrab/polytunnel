package com.example.jsontransform;

import java.io.IOException;
import java.time.LocalDate;
import java.util.List;

/**
 * Main application demonstrating JSON transformation pipeline.
 */
public class TransformApp {

    public static void main(String[] args) throws IOException {
        JsonProcessor processor = new JsonProcessor();
        ProductFilter filter = new ProductFilter();

        // Create sample product data
        List<Product> products = List.of(
                new Product(1, "Wireless Mouse", "Electronics", 29.99, true, LocalDate.of(2024, 1, 15)),
                new Product(2, "USB-C Hub", "Electronics", 49.99, true, LocalDate.of(2024, 3, 10)),
                new Product(3, "Standing Desk", "Furniture", 399.99, false, LocalDate.of(2023, 11, 1)),
                new Product(4, "Mechanical Keyboard", "Electronics", 89.99, true, LocalDate.of(2024, 2, 20)),
                new Product(5, "Desk Lamp", "Furniture", 34.99, true, LocalDate.of(2024, 4, 5)),
                new Product(6, "Monitor Arm", "Furniture", 129.99, true, LocalDate.of(2023, 9, 12)),
                new Product(7, "Webcam HD", "Electronics", 59.99, false, LocalDate.of(2024, 1, 30))
        );

        // Serialize to JSON
        String json = processor.toJson(products);
        System.out.println("=== All Products (JSON) ===");
        System.out.println(json);

        // Round-trip: parse back from JSON
        List<Product> parsed = processor.parseProducts(json);
        System.out.println("\n=== Parsed " + parsed.size() + " products from JSON ===");

        // Filter: electronics only
        List<Product> electronics = filter.byCategory(parsed, "Electronics");
        System.out.println("\n=== Electronics (" + electronics.size() + " items) ===");
        electronics.forEach(p -> System.out.println("  " + p));

        // Filter: in-stock, sorted by price
        List<Product> available = filter.sortByPrice(filter.inStockOnly(parsed));
        System.out.println("\n=== In-Stock (sorted by price) ===");
        available.forEach(p -> System.out.println("  " + p));

        // Filter: price range $30-$100
        List<Product> midRange = filter.byPriceRange(parsed, 30.0, 100.0);
        System.out.println("\n=== Price $30-$100 (" + midRange.size() + " items) ===");
        midRange.forEach(p -> System.out.println("  " + p));

        // Stats
        System.out.println("\n=== Statistics ===");
        System.out.printf("  Average price (all): $%.2f%n", filter.averagePrice(parsed));
        System.out.printf("  Average price (electronics): $%.2f%n", filter.averagePrice(electronics));
    }
}
