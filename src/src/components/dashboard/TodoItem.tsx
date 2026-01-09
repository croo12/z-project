import { memo } from "react";
import type { TodoItem as TodoItemType } from "../../hooks/useTodos";
import { cn } from "../../lib/utils";

interface TodoItemProps {
  todo: TodoItemType;
  onToggle: (id: number) => void;
}

// Optimization: Memoize the component to prevent unnecessary re-renders
// when the parent list updates but this item hasn't changed.
const TodoItem = memo(function TodoItem({ todo, onToggle }: TodoItemProps) {
  return (
    <div
      className={cn(
        "group relative bg-white border-2 border-foreground p-4 transition-all duration-200",
        "shadow-[2px_2px_0_0_#2d2d2d] hover:shadow-[4px_4px_0_0_#2d2d2d] hover:-translate-y-[1px]",
        "rounded-wobbly",
        todo.completed ? "opacity-60 bg-gray-50" : ""
      )}
    >
      <label className="flex items-center w-full cursor-pointer gap-4">
        <div className="relative flex items-center justify-center">
            <input
                type="checkbox"
                checked={todo.completed}
                onChange={() => onToggle(todo.id)}
                className="peer appearance-none w-6 h-6 border-2 border-foreground rounded-full bg-white checked:bg-blue checked:border-blue cursor-pointer transition-colors"
                aria-label={`Mark ${todo.text} as ${todo.completed ? "incomplete" : "complete"}`}
            />
            <span className="absolute text-white hidden peer-checked:block pointer-events-none font-bold text-sm">✓</span>
        </div>

        <span
          className={cn(
            "flex-1 font-sans text-lg transition-all",
            todo.completed ? "line-through text-foreground/50 decoration-wavy decoration-2" : "text-foreground"
          )}
        >
          {todo.text}
        </span>

        {todo.completed && (
          <span className="font-heading font-bold text-sm text-green-600 rotate-[-5deg] border-2 border-green-600 px-2 rounded-lg">
            Done!
          </span>
        )}
      </label>
    </div>
  );
});

export default TodoItem;
