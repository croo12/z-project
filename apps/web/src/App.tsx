import { useState } from "react";
import Layout from "./components/Layout";
import DashboardView from "./views/DashboardView";
import ArticleView from "./views/ArticleView";
import { useArticles } from "./hooks/useArticles";

function App() {
  const [currentTab, setCurrentTab] = useState<"dashboard" | "articles">(
    "dashboard"
  );

  // Lifted state to persist data across tab switches
  const { articles, refreshArticles } = useArticles();

  return (
    <Layout currentTab={currentTab} onTabChange={setCurrentTab}>
      {currentTab === "dashboard" ? (
        <DashboardView />
      ) : (
        <ArticleView articles={articles} onRefresh={refreshArticles} />
      )}
    </Layout>
  );
}

export default App;
