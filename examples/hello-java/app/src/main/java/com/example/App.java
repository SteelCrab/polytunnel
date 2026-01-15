package com.example;

import com.google.common.base.Strings;

/**
 * Simple greeting application demonstrating Polytunnel.
 */
public class App {
    
    public static void main(String[] args) {
        App app = new App();
        System.out.println(app.greet("World"));
    }
    
    /**
     * Generate a greeting message.
     * 
     * @param name the name to greet
     * @return formatted greeting
     */
    public String greet(String name) {
        if (Strings.isNullOrEmpty(name)) {
            return "Hello, Guest!";
        }
        return "Hello, " + name + "!";
    }
    
    /**
     * Repeat a string n times using Guava.
     * 
     * @param text the text to repeat
     * @param count number of repetitions
     * @return repeated string
     */
    public String repeat(String text, int count) {
        return Strings.repeat(text, count);
    }
}
