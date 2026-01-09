import React, { useState } from "react";
import { TodoItem as TodoItemType } from "../../types";
import TodoItem from "./TodoItem";

interface TodoListProps {
  todos: TodoItemType[];
  addTodo: (text: string) => Promise<void>;
  toggleTodo: (id: number) => Promise<void>;
}

export default function TodoList({ todos, addTodo, toggleTodo }: TodoListProps) {
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
