package com.example.todo;

public class Todo {
    public int id;
    public String title;
    public boolean completed;

    public Todo() {
    }

    public Todo(int id, String title, boolean completed) {
        this.id = id;
        this.title = title;
        this.completed = completed;
    }
}
