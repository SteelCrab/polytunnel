package com.example.todo;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.assertEquals;

public class TodoTest {

    @Test
    public void testTodoModel() {
        Todo todo = new Todo(1, "Test", false);
        assertEquals(1, todo.id);
        assertEquals("Test", todo.title);
        assertEquals(false, todo.completed);
    }
}
