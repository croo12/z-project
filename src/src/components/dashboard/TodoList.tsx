import React, { useState } from "react";
import { useTodos } from "../../hooks/useTodos";

export default function TodoList() {
  const { todos, addTodo, toggleTodo } = useTodos();
  const [todoInput, setTodoInput] = useState("");

  const handleAddTodo = async (e: React.FormEvent) => {
    e.preventDefault();
    await addTodo(todoInput);
    setTodoInput("");
  };

  return (
    <section>
      <h2>Todos</h2>
      <form onSubmit={handleAddTodo}>
        <input
          value={todoInput}
          onChange={(e) => setTodoInput(e.target.value)}
          placeholder="New Task..."
          aria-label="New task name"
        />
        <button type="submit">Add</button>
      </form>
      <div id="todo-list">
        {todos.length === 0 && (
          <div
            className="empty-state"
            style={{ textAlign: "center", padding: "1rem", color: "#888" }}
          >
            <p>All caught up! 🎉</p>
          </div>
        )}
        {todos.map((todo) => (
          <div
            key={todo.id}
            className={`card todo-item ${todo.completed ? "completed" : ""}`}
          >
            <label
              style={{
                display: "flex",
                alignItems: "center",
                width: "100%",
                cursor: "pointer",
              }}
            >
              <input
                type="checkbox"
                checked={todo.completed}
                onChange={() => toggleTodo(todo.id)}
                style={{
                  marginRight: "10px",
                  width: "20px",
                  height: "20px",
                }}
                aria-label={`Mark ${todo.text} as ${
                  todo.completed ? "incomplete" : "complete"
                }`}
              />
              <span
                style={{
                  flex: 1,
                  textDecoration: todo.completed ? "line-through" : "none",
                  color: todo.completed ? "#888" : "inherit",
                }}
              >
                {todo.text}
              </span>
              <small style={{ color: "#888" }}>
                {todo.completed ? "Done" : ""}
              </small>
            </label>
          </div>
        ))}
      </div>
    </section>
  );
}
