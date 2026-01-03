import TodoList from "../components/dashboard/TodoList";
import WorkLogList from "../components/dashboard/WorkLogList";
import "../App.css";

export default function DashboardView() {
  return (
    <div className="view-container">
      <TodoList />
      <WorkLogList />
    </div>
  );
}
