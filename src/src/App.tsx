import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import ArticleList from "./components/ArticleList";
import type { Article } from "./types";
import "./App.css";

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

function App() {
  const [articles, setArticles] = useState<Article[]>([]);
  const [todos, setTodos] = useState<TodoItem[]>([]);
  const [workLogs, setWorkLogs] = useState<WorkLog[]>([]);

  const [todoInput, setTodoInput] = useState("");
  const [workProject, setWorkProject] = useState("");
  const [workHours, setWorkHours] = useState("");

  useEffect(() => {
    refreshArticles();
    refreshTodos();
    refreshWorkLogs();
  }, []);

  async function refreshArticles() {
    console.log("Refreshing articles...");
    try {
        const data = await invoke<Article[]>("get_recommended_articles");
        setArticles(data);
    } catch (e) {
        console.error("Failed to fetch articles", e);
    }
  }

  async function refreshTodos() {
    setTodos(await invoke("get_todos"));
  }

  async function refreshWorkLogs() {
    setWorkLogs(await invoke("get_work_logs"));
  }

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
    <div className="container">
      <h1>My Dev Dashboard</h1>

      <div className="row">
        {/* News Section */}
        <section>
          <h2>Dev Recommendations</h2>
          <ArticleList 
            articles={articles} 
            onRefresh={refreshArticles} 
            onFeedbackUpdate={refreshArticles} 
          />
        </section>

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
              placeholder="Hours"
              step="0.5"
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
    </div>
  );
}

export default App;
