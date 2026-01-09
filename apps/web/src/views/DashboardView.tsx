import TodoList from "../components/dashboard/TodoList";
import WorkLogList from "../components/dashboard/WorkLogList";

export default function DashboardView() {
  return (
    <div className="grid grid-cols-1 md:grid-cols-2 gap-12 mt-4">
      <TodoList />
      <WorkLogList />
    </div>
  );
}
