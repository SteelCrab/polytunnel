package com.example.jsontransform;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

import java.io.IOException;
import java.time.LocalDate;
import java.util.List;

/**
 * Unit tests for JsonProcessor.
 */
class JsonProcessorTest {

    private final JsonProcessor processor = new JsonProcessor();

    @Test
    void testSerializeSingleProduct() throws IOException {
        Product product = new Product(1, "Widget", "Tools", 9.99, true, LocalDate.of(2024, 6, 15));
        String json = processor.toJson(product);
        assertTrue(json.contains("\"name\" : \"Widget\""));
        assertTrue(json.contains("\"release_date\" : \"2024-06-15\""));
    }

    @Test
    void testParseSingleProduct() throws IOException {
        String json = """
                {
                  "id": 1,
                  "name": "Widget",
                  "category": "Tools",
                  "price": 9.99,
                  "in_stock": true,
                  "release_date": "2024-06-15"
                }
                """;
        Product product = processor.parseProduct(json);
        assertEquals(1, product.getId());
        assertEquals("Widget", product.getName());
        assertEquals("Tools", product.getCategory());
        assertEquals(9.99, product.getPrice(), 0.001);
        assertTrue(product.isInStock());
        assertEquals(LocalDate.of(2024, 6, 15), product.getReleaseDate());
    }

    @Test
    void testRoundTrip() throws IOException {
        List<Product> original = List.of(
                new Product(1, "Alpha", "A", 10.0, true, LocalDate.of(2024, 1, 1)),
                new Product(2, "Beta", "B", 20.0, false, LocalDate.of(2024, 2, 2))
        );
        String json = processor.toJson(original);
        List<Product> parsed = processor.parseProducts(json);
        assertEquals(2, parsed.size());
        assertEquals("Alpha", parsed.get(0).getName());
        assertEquals("Beta", parsed.get(1).getName());
        assertEquals(20.0, parsed.get(1).getPrice(), 0.001);
    }

    @Test
    void testIgnoresUnknownFields() throws IOException {
        String json = """
                {
                  "id": 1,
                  "name": "Thing",
                  "category": "Misc",
                  "price": 5.0,
                  "in_stock": true,
                  "release_date": "2024-01-01",
                  "unknown_field": "should be ignored"
                }
                """;
        Product product = processor.parseProduct(json);
        assertEquals("Thing", product.getName());
    }

    @Test
    void testEmptyList() throws IOException {
        String json = processor.toJson(List.of());
        List<Product> parsed = processor.parseProducts(json);
        assertTrue(parsed.isEmpty());
    }
}
