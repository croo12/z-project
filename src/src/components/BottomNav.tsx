import "../App.css";

interface Props {
  currentTab: "dashboard" | "articles";
  onTabChange: (tab: "dashboard" | "articles") => void;
}

export default function BottomNav({ currentTab, onTabChange }: Props) {
  return (
    <nav className="bottom-nav">
      <button
        className={`nav-item ${currentTab === "dashboard" ? "active" : ""}`}
        onClick={() => onTabChange("dashboard")}
      >
        <span className="icon">📝</span>
        <span className="label">Dashboard</span>
      </button>
      <button
        className={`nav-item ${currentTab === "articles" ? "active" : ""}`}
        onClick={() => onTabChange("articles")}
      >
        <span className="icon">📰</span>
        <span className="label">Articles</span>
      </button>
    </nav>
  );
}
