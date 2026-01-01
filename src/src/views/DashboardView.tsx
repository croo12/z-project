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
            <div
              key={todo.id}
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
                  onChange={() => handleToggleTodo(todo.id)}
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
            aria-label="Project name"
          />
          <input
            type="number"
            value={workHours}
            onChange={(e) => setWorkHours(e.target.value)}
            placeholder="h"
            step="0.5"
            style={{ maxWidth: "60px" }}
            aria-label="Hours worked"
          />
          <button type="submit">Log</button>
        </form>
        <div id="work-list">
          {workLogs.length === 0 && (
            <div
              className="empty-state"
              style={{ textAlign: "center", padding: "1rem", color: "#888" }}
            >
              <p>No logs yet 📝</p>
            </div>
          )}
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
