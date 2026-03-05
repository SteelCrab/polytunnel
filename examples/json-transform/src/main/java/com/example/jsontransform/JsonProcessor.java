package com.example.jsontransform;

import com.fasterxml.jackson.core.type.TypeReference;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.databind.SerializationFeature;
import com.fasterxml.jackson.datatype.jsr310.JavaTimeModule;

import java.io.IOException;
import java.util.List;

/**
 * Handles JSON serialization and deserialization of product data.
 */
public class JsonProcessor {

    private final ObjectMapper mapper;

    public JsonProcessor() {
        this.mapper = new ObjectMapper();
        this.mapper.registerModule(new JavaTimeModule());
        this.mapper.disable(SerializationFeature.WRITE_DATES_AS_TIMESTAMPS);
        this.mapper.enable(SerializationFeature.INDENT_OUTPUT);
    }

    /**
     * Parse a JSON string into a list of products.
     *
     * @param json the JSON string
     * @return list of products
     * @throws IOException if parsing fails
     */
    public List<Product> parseProducts(String json) throws IOException {
        return mapper.readValue(json, new TypeReference<List<Product>>() {});
    }

    /**
     * Serialize a list of products to a JSON string.
     *
     * @param products the products to serialize
     * @return pretty-printed JSON string
     * @throws IOException if serialization fails
     */
    public String toJson(List<Product> products) throws IOException {
        return mapper.writeValueAsString(products);
    }

    /**
     * Parse a single product from a JSON string.
     *
     * @param json the JSON string
     * @return the parsed product
     * @throws IOException if parsing fails
     */
    public Product parseProduct(String json) throws IOException {
        return mapper.readValue(json, Product.class);
    }

    /**
     * Serialize a single product to a JSON string.
     *
     * @param product the product to serialize
     * @return pretty-printed JSON string
     * @throws IOException if serialization fails
     */
    public String toJson(Product product) throws IOException {
        return mapper.writeValueAsString(product);
    }
}
