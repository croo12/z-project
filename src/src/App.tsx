import { useState } from "react";
import Layout from "./components/Layout";
import DashboardView from "./views/DashboardView";
import ArticleView from "./views/ArticleView";
import { useTodos } from "./hooks/useTodos";
import { useWorkLogs } from "./hooks/useWorkLogs";
import { useArticles } from "./hooks/useArticles";
import "./App.css";

function App() {
  const [currentTab, setCurrentTab] = useState<"dashboard" | "articles">(
    "dashboard"
  );

  // Lifted state management to App level to prevent redundant data fetching
  // when switching tabs.
  const { todos, addTodo, toggleTodo } = useTodos();
  const { workLogs, addWorkLog } = useWorkLogs();
  const { articles, refreshArticles } = useArticles();

  return (
    <Layout currentTab={currentTab} onTabChange={setCurrentTab}>
      {currentTab === "dashboard" ? (
        <DashboardView
          todos={todos}
          workLogs={workLogs}
          addTodo={addTodo}
          toggleTodo={toggleTodo}
          addWorkLog={addWorkLog}
        />
      ) : (
        <ArticleView articles={articles} onRefresh={refreshArticles} />
      )}
    </Layout>
  );
}

export default App;
