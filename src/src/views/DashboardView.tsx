import TodoList from "../components/dashboard/TodoList";
import WorkLogList from "../components/dashboard/WorkLogList";
import { TodoItem, WorkLog } from "../types";
import "../App.css";

interface DashboardViewProps {
  todos: TodoItem[];
  workLogs: WorkLog[];
  addTodo: (text: string) => Promise<void>;
  toggleTodo: (id: number) => Promise<void>;
  addWorkLog: (project: string, hours: number) => Promise<void>;
}

export default function DashboardView({
  todos,
  workLogs,
  addTodo,
  toggleTodo,
  addWorkLog,
}: DashboardViewProps) {
  return (
    <div className="view-container">
      <TodoList todos={todos} addTodo={addTodo} toggleTodo={toggleTodo} />
      <WorkLogList workLogs={workLogs} addWorkLog={addWorkLog} />
    </div>
  );
}
