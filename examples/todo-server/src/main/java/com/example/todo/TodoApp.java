package com.example.todo;

import io.javalin.Javalin;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public class TodoApp {
    private static final Logger log = LoggerFactory.getLogger(TodoApp.class);

    public static void main(String[] args) {
        Javalin app = Javalin.create().start(8080);

        app.get("/", ctx -> ctx.result("Hello Polytunnel!"));
        app.get("/todos", TodoController::getAll);
        app.post("/todos", TodoController::create);

        log.info("Server started on port 8080");
    }
}
