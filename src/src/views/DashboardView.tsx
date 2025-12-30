import { useState, useEffect, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";
import "../App.css";

interface TodoItem {
  id: number;
  text: string;
  completed: boolean;
}

interface WorkLog {
  id: number;
  project: string;
  hours: number;
  date: string;
}

export default function DashboardView() {
  const [todos, setTodos] = useState<TodoItem[]>([]);
  const [workLogs, setWorkLogs] = useState<WorkLog[]>([]);

  const [todoInput, setTodoInput] = useState("");
  const [workProject, setWorkProject] = useState("");
  const [workHours, setWorkHours] = useState("");

  const refreshTodos = useCallback(async () => {
    setTodos(await invoke("get_todos"));
  }, []);

  const refreshWorkLogs = useCallback(async () => {
    setWorkLogs(await invoke("get_work_logs"));
  }, []);

  useEffect(() => {
    // eslint-disable-next-line react-hooks/exhaustive-deps
    refreshTodos();
    // eslint-disable-next-line react-hooks/exhaustive-deps
    refreshWorkLogs();
  }, []);

  async function handleAddTodo(e: React.FormEvent) {
    e.preventDefault();
    if (!todoInput) return;
    await invoke("add_todo", { text: todoInput });
    setTodoInput("");
    refreshTodos();
  }

  async function handleToggleTodo(id: number) {
    await invoke("toggle_todo", { id });
    refreshTodos();
  }

  async function handleAddWorkLog(e: React.FormEvent) {
    e.preventDefault();
    if (!workProject || !workHours) return;
    await invoke("add_work_log", {
      project: workProject,
      hours: parseFloat(workHours),
    });
    setWorkProject("");
    setWorkHours("");
    refreshWorkLogs();
  }

  return (
    <div className="view-container">
      {/* Todos Section */}
      <section>
        <h2>Todos</h2>
        <form onSubmit={handleAddTodo}>
          <input
            value={todoInput}
            onChange={(e) => setTodoInput(e.target.value)}
            placeholder="New Task..."
          />
          <button type="submit">Add</button>
        </form>
        <div id="todo-list">
          {todos.map((todo) => (
            <div
              key={todo.id}
              className={`card todo-item ${todo.completed ? "completed" : ""}`}
              onClick={() => handleToggleTodo(todo.id)}
            >
              <span>{todo.text}</span>
              <small>{todo.completed ? "Done" : ""}</small>
            </div>
          ))}
        </div>
      </section>

      {/* Work Log Section */}
      <section>
        <h2>Work Log</h2>
        <form onSubmit={handleAddWorkLog}>
          <input
            value={workProject}
            onChange={(e) => setWorkProject(e.target.value)}
            placeholder="Project"
          />
          <input
            type="number"
            value={workHours}
            onChange={(e) => setWorkHours(e.target.value)}
            placeholder="h"
            step="0.5"
            style={{ maxWidth: '60px' }}
          />
          <button type="submit">Log</button>
        </form>
        <div id="work-list">
          {workLogs.map((log) => (
            <div key={log.id} className="card log-item">
              <span className="log-project">{log.project}</span>
              <span className="log-details">
                {log.hours}h ({log.date})
              </span>
            </div>
          ))}
        </div>
      </section>
    </div>
  );
}
