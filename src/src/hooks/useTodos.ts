import { useState, useCallback, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";

export interface TodoItem {
  id: number;
  text: string;
  completed: boolean;
}

export function useTodos() {
  const [todos, setTodos] = useState<TodoItem[]>([]);

  const refreshTodos = useCallback(async () => {
    setTodos(await invoke("get_todos"));
  }, []);

  useEffect(() => {
    // eslint-disable-next-line react-hooks/set-state-in-effect
    void refreshTodos();
  }, [refreshTodos]);

  const addTodo = async (text: string) => {
    if (!text) return;
    await invoke("add_todo", { text });
    await refreshTodos();
  };

  const toggleTodo = async (id: number) => {
    await invoke("toggle_todo", { id });
    await refreshTodos();
  };

  return { todos, addTodo, toggleTodo };
}
