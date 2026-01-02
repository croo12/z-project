import { useState, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";

export interface TodoItem {
  id: number;
  text: string;
  completed: boolean;
}

export function useTodos() {
  const [todos, setTodos] = useState<TodoItem[]>([]);

  const refreshTodos = useCallback(async () => {
    try {
      setTodos(await invoke("get_todos"));
    } catch (e) {
      console.error("Failed to fetch todos", e);
    }
  }, []);

  const addTodo = useCallback(async (text: string) => {
    try {
      setTodos(await invoke("add_todo", { text }));
    } catch (e) {
      console.error("Failed to add todo", e);
    }
  }, []);

  const toggleTodo = useCallback(async (id: number) => {
    try {
      setTodos(await invoke("toggle_todo", { id }));
    } catch (e) {
      console.error("Failed to toggle todo", e);
    }
  }, []);

  const deleteTodo = useCallback(async (id: number) => {
    try {
      setTodos(await invoke("delete_todo", { id }));
    } catch (e) {
      console.error("Failed to delete todo", e);
    }
  }, []);

  return { todos, refreshTodos, addTodo, toggleTodo, deleteTodo };
}
