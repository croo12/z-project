import React, { memo } from "react";
import { TodoItem as TodoItemType } from "../../hooks/useTodos";

interface TodoItemProps {
  todo: TodoItemType;
  toggleTodo: (id: number) => void;
}

const TodoItem = memo(({ todo, toggleTodo }: TodoItemProps) => {
  return (
    <div
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
  );
});

TodoItem.displayName = "TodoItem";

export default TodoItem;
