import React, { useState } from "react";
import type { TodoItem } from "../../hooks/useTodos";

interface TodoListProps {
  todos: TodoItem[];
  onAdd: (text: string) => void;
  onToggle: (id: number) => void;
  onDelete?: (id: number) => void;
}

export default function TodoList({ todos, onAdd, onToggle, onDelete }: TodoListProps) {
  const [todoInput, setTodoInput] = useState("");

  const handleAdd = (e: React.FormEvent) => {
    e.preventDefault();
    if (!todoInput) return;
    onAdd(todoInput);
    setTodoInput("");
  };

  return (
    <section>
      <h2>Todos</h2>
      <form onSubmit={handleAdd}>
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
            style={{ display: "flex", alignItems: "center", justifyContent: "space-between" }}
          >
            <label
              style={{
                display: "flex",
                alignItems: "center",
                flex: 1,
                cursor: "pointer",
              }}
            >
              <input
                type="checkbox"
                checked={todo.completed}
                onChange={() => onToggle(todo.id)}
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
                  textDecoration: todo.completed ? "line-through" : "none",
                  color: todo.completed ? "#888" : "inherit",
                }}
              >
                {todo.text}
              </span>
            </label>
            {onDelete && todo.completed && (
                <button
                    onClick={() => onDelete(todo.id)}
                    style={{ background: "transparent", border: "none", color: "red", cursor: "pointer", fontSize: "0.8rem" }}
                    aria-label="Delete task"
                >
                    ✕
                </button>
            )}
            {!onDelete && todo.completed && (
                 <small style={{ color: "#888" }}>Done</small>
            )}
          </div>
        ))}
      </div>
    </section>
  );
}
