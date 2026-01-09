import { memo } from "react";
import type { TodoItem as TodoItemType } from "../../types";

interface TodoItemProps {
  todo: TodoItemType;
  onToggle: (id: number) => void;
}

// Optimization: Memoize the component to prevent unnecessary re-renders
// when the parent list updates but this item hasn't changed.
const TodoItem = memo(function TodoItem({ todo, onToggle }: TodoItemProps) {
  return (
    <div className={`card todo-item ${todo.completed ? "completed" : ""}`}>
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
            flex: 1,
            textDecoration: todo.completed ? "line-through" : "none",
            color: todo.completed ? "#888" : "inherit",
          }}
        >
          {todo.text}
        </span>
        <small style={{ color: "#888" }}>{todo.completed ? "Done" : ""}</small>
      </label>
    </div>
  );
});

export default TodoItem;
