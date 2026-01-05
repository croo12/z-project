import React, { useState } from "react";
import { useTodos } from "../../hooks/useTodos";
import TodoItem from "./TodoItem";

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
          <TodoItem key={todo.id} todo={todo} onToggle={toggleTodo} />
        ))}
      </div>
    </section>
  );
}
