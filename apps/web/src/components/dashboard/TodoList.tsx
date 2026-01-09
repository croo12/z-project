import React, { useState } from "react";
import { useTodos } from "../../hooks/useTodos";
import TodoItem from "./TodoItem";
import { cn } from "../../lib/utils";

export default function TodoList() {
  const { todos, addTodo, toggleTodo } = useTodos();
  const [todoInput, setTodoInput] = useState("");

  const handleAddTodo = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!todoInput.trim()) return;
    await addTodo(todoInput);
    setTodoInput("");
  };

  return (
    <section className="relative">
      <h2 className="text-4xl font-heading font-bold mb-6 rotate-[-1deg] inline-block bg-post-it px-4 py-1 shadow-hard border-2 border-foreground transform -translate-x-2">
        Todos
      </h2>

      <form onSubmit={handleAddTodo} className="flex gap-3 mb-8">
        <input
          value={todoInput}
          onChange={(e) => setTodoInput(e.target.value)}
          placeholder="New Task..."
          aria-label="New task name"
          className={cn(
            "flex-grow px-4 py-3 bg-white font-sans text-lg outline-none transition-all",
            "border-2 border-foreground rounded-wobbly",
            "shadow-hard focus:shadow-hard-hover focus:translate-x-[2px] focus:translate-y-[2px]",
            "placeholder:text-foreground/40"
          )}
        />
        <button
          type="submit"
          className={cn(
            "px-6 font-heading font-bold text-lg bg-white border-[3px] border-foreground",
            "rounded-wobbly",
            "shadow-hard hover:bg-accent hover:text-white hover:shadow-hard-hover hover:translate-x-[2px] hover:translate-y-[2px]",
            "active:shadow-none active:translate-x-[4px] active:translate-y-[4px]",
            "transition-all duration-100"
          )}
        >
          Add
        </button>
      </form>

      <div id="todo-list" className="space-y-4">
        {todos.length === 0 && (
          <div className="text-center py-12 border-2 border-dashed border-foreground/30 rounded-[20px] bg-white/50">
            <p className="font-heading text-2xl text-foreground/50 rotate-[-2deg]">All caught up! 🎉</p>
          </div>
        )}
        {todos.map((todo) => (
          <TodoItem key={todo.id} todo={todo} onToggle={toggleTodo} />
        ))}
      </div>
    </section>
  );
}
