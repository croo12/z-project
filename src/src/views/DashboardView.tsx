import { useEffect } from "react";
import "../App.css";
import { useTodos } from "../hooks/useTodos";
import { useWorkLogs } from "../hooks/useWorkLogs";
import TodoList from "../components/dashboard/TodoList";
import WorkLogList from "../components/dashboard/WorkLogList";

export default function DashboardView() {
  const { todos, refreshTodos, addTodo, toggleTodo, deleteTodo } = useTodos();
  const { workLogs, refreshWorkLogs, addWorkLog } = useWorkLogs();

  useEffect(() => {
    refreshTodos();
    refreshWorkLogs();
  }, [refreshTodos, refreshWorkLogs]);

  return (
    <div className="view-container">
      <TodoList
        todos={todos}
        onAdd={addTodo}
        onToggle={toggleTodo}
        onDelete={deleteTodo}
      />
      <WorkLogList
        logs={workLogs}
        onAdd={addWorkLog}
      />
    </div>
  );
}
