import { useState } from "react";
import Layout from "./components/Layout";
import DashboardView from "./views/DashboardView";
import ArticleView from "./views/ArticleView";
import "./App.css";

function App() {
  const [currentTab, setCurrentTab] = useState<"dashboard" | "articles">("dashboard");

  return (
    <Layout currentTab={currentTab} onTabChange={setCurrentTab}>
      {currentTab === "dashboard" ? <DashboardView /> : <ArticleView />}
    </Layout>
  );
}

export default App;
