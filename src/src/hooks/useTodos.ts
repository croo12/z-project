import { useState, useCallback, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";

export interface Todo {
  id: number;
  text: string;
  completed: boolean;
}

export function useTodos() {
  const [todos, setTodos] = useState<Todo[]>([]);

  const fetchTodos = useCallback(async () => {
    const data = await invoke<Todo[]>("get_todos");
    setTodos(data);
  }, []);

  const addTodo = useCallback(
    async (text: string) => {
      const data = await invoke<Todo[]>("add_todo", { text });
      setTodos(data);
    },
    []
  );

  const toggleTodo = useCallback(
    async (id: number) => {
      const data = await invoke<Todo[]>("toggle_todo", { id });
      setTodos(data);
    },
    []
  );

  const deleteTodo = useCallback(
    async (id: number) => {
      const data = await invoke<Todo[]>("delete_todo", { id });
      setTodos(data);
    },
    []
  );

  useEffect(() => {
    // eslint-disable-next-line react-hooks/set-state-in-effect
    void fetchTodos();
  }, [fetchTodos]);

  return { todos, addTodo, toggleTodo, deleteTodo };
}
