package com.example.jsontransform;

import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

import java.time.LocalDate;
import java.util.List;

/**
 * Unit tests for ProductFilter.
 */
class ProductFilterTest {

    private ProductFilter filter;
    private List<Product> products;

    @BeforeEach
    void setUp() {
        filter = new ProductFilter();
        products = List.of(
                new Product(1, "Laptop", "Electronics", 999.99, true, LocalDate.of(2024, 1, 1)),
                new Product(2, "Chair", "Furniture", 249.99, true, LocalDate.of(2024, 2, 1)),
                new Product(3, "Phone", "Electronics", 699.99, false, LocalDate.of(2024, 3, 1)),
                new Product(4, "Desk", "Furniture", 199.99, true, LocalDate.of(2024, 4, 1)),
                new Product(5, "Tablet", "Electronics", 449.99, true, LocalDate.of(2024, 5, 1))
        );
    }

    @Test
    void testByCategoryElectronics() {
        List<Product> result = filter.byCategory(products, "Electronics");
        assertEquals(3, result.size());
        assertTrue(result.stream().allMatch(p -> p.getCategory().equals("Electronics")));
    }

    @Test
    void testByCategoryCaseInsensitive() {
        List<Product> result = filter.byCategory(products, "electronics");
        assertEquals(3, result.size());
    }

    @Test
    void testByCategoryNoMatch() {
        List<Product> result = filter.byCategory(products, "Clothing");
        assertTrue(result.isEmpty());
    }

    @Test
    void testInStockOnly() {
        List<Product> result = filter.inStockOnly(products);
        assertEquals(4, result.size());
        assertTrue(result.stream().allMatch(Product::isInStock));
    }

    @Test
    void testByPriceRange() {
        List<Product> result = filter.byPriceRange(products, 200.0, 700.0);
        assertEquals(3, result.size()); // Chair 249.99, Phone 699.99, Tablet 449.99
    }

    @Test
    void testByPriceRangeNoMatch() {
        List<Product> result = filter.byPriceRange(products, 1500.0, 2000.0);
        assertTrue(result.isEmpty());
    }

    @Test
    void testSortByPrice() {
        List<Product> sorted = filter.sortByPrice(products);
        assertEquals(199.99, sorted.get(0).getPrice(), 0.001);
        assertEquals(999.99, sorted.get(sorted.size() - 1).getPrice(), 0.001);
    }

    @Test
    void testAveragePrice() {
        double avg = filter.averagePrice(products);
        double expected = (999.99 + 249.99 + 699.99 + 199.99 + 449.99) / 5.0;
        assertEquals(expected, avg, 0.01);
    }

    @Test
    void testAveragePriceEmpty() {
        double avg = filter.averagePrice(List.of());
        assertEquals(0.0, avg, 0.001);
    }

    @Test
    void testChainedFilters() {
        // In-stock electronics sorted by price
        List<Product> result = filter.sortByPrice(
                filter.inStockOnly(filter.byCategory(products, "Electronics"))
        );
        assertEquals(2, result.size()); // Tablet(449.99) and Laptop(999.99); Phone is out of stock
        assertEquals("Tablet", result.get(0).getName());
        assertEquals("Laptop", result.get(1).getName());
    }
}
