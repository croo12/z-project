import { useState } from "react";
import Layout from "./components/Layout";
import DebugOverlay from "./components/Debug/DebugOverlay";
import DashboardView from "./views/DashboardView";
import ArticleView from "./views/ArticleView";
import { useArticles } from "./hooks/useArticles";

function App() {
  const [currentTab, setCurrentTab] = useState<"dashboard" | "articles">(
    "dashboard"
  );

  // Lifted state to persist data across tab switches
  const { articles, refreshArticles, syncArticle } = useArticles();

  return (
    <>
      {import.meta.env.DEV && <DebugOverlay />}
      <Layout currentTab={currentTab} onTabChange={setCurrentTab}>
        {currentTab === "dashboard" ? (
          <DashboardView />
        ) : (
          <ArticleView
            articles={articles}
            onRefresh={refreshArticles}
            onSyncArticle={syncArticle}
          />
        )}
      </Layout>
    </>
  );
}

export default App;
