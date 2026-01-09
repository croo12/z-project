import { useState, useCallback, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { TodoItem } from "../types";

export function useTodos() {
  const [todos, setTodos] = useState<TodoItem[]>([]);

  const refreshTodos = useCallback(async () => {
    setTodos(await invoke("get_todos"));
  }, []);

  useEffect(() => {
    // eslint-disable-next-line react-hooks/set-state-in-effect
    void refreshTodos();
  }, [refreshTodos]);

  // Optimization: Wrap in useCallback to ensure stable function references
  // This prevents child components (like TodoItem) from re-rendering unnecessarily
  const addTodo = useCallback(
    async (text: string) => {
      if (!text) return;
      await invoke("add_todo", { text });
      await refreshTodos();
    },
    [refreshTodos]
  );

  const toggleTodo = useCallback(
    async (id: number) => {
      await invoke("toggle_todo", { id });
      await refreshTodos();
    },
    [refreshTodos]
  );

  return { todos, addTodo, toggleTodo };
}
