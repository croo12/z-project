import type { ReactNode } from "react";
import BottomNav from "./BottomNav";

interface Props {
  children: ReactNode;
  currentTab: "dashboard" | "articles";
  onTabChange: (tab: "dashboard" | "articles") => void;
}

export default function Layout({ children, currentTab, onTabChange }: Props) {
  return (
    <div className="min-h-screen pb-24">
      <main className="max-w-5xl mx-auto px-6 py-10 md:py-20">
        {children}
      </main>
      <BottomNav currentTab={currentTab} onTabChange={onTabChange} />
    </div>
  );
}
