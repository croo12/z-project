import type { ReactNode } from "react";
import BottomNav from "./BottomNav";

interface Props {
  children: ReactNode;
  currentTab: "dashboard" | "articles";
  onTabChange: (tab: "dashboard" | "articles") => void;
}

export default function Layout({ children, currentTab, onTabChange }: Props) {
  return (
    <div className="app-layout">
      <main className="app-content">{children}</main>
      <BottomNav currentTab={currentTab} onTabChange={onTabChange} />
    </div>
  );
}
