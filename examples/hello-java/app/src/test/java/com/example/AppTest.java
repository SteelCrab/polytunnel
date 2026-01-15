package com.example;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

/**
 * Unit tests for App class.
 */
class AppTest {
    
    @Test
    void testGreetWithName() {
        App app = new App();
        assertEquals("Hello, World!", app.greet("World"));
    }
    
    @Test
    void testGreetWithNull() {
        App app = new App();
        assertEquals("Hello, Guest!", app.greet(null));
    }
    
    @Test
    void testGreetWithEmpty() {
        App app = new App();
        assertEquals("Hello, Guest!", app.greet(""));
    }
    
    @Test
    void testRepeat() {
        App app = new App();
        assertEquals("abcabcabc", app.repeat("abc", 3));
    }
    
    @Test
    void testRepeatZero() {
        App app = new App();
        assertEquals("", app.repeat("abc", 0));
    }
}
