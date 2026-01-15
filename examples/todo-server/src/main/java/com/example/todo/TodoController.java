package com.example.todo;

import io.javalin.http.Context;
import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.atomic.AtomicInteger;

public class TodoController {
    private static final List<Todo> todos = new ArrayList<>();
    private static final AtomicInteger lastId = new AtomicInteger(0);

    // Initial data
    static {
        todos.add(new Todo(lastId.incrementAndGet(), "Learn Polytunnel", true));
        todos.add(new Todo(lastId.incrementAndGet(), "Build Todo API", false));
    }

    public static void getAll(Context ctx) {
        ctx.json(todos);
    }

    public static void create(Context ctx) {
        Todo todo = ctx.bodyAsClass(Todo.class);
        todo.id = lastId.incrementAndGet();
        todos.add(todo);
        ctx.status(201).json(todo);
    }
}
