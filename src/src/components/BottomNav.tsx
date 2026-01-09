interface Props {
  currentTab: "dashboard" | "articles";
  onTabChange: (tab: "dashboard" | "articles") => void;
}

export default function BottomNav({ currentTab, onTabChange }: Props) {
  return (
    <div className="fixed bottom-6 left-0 right-0 z-50 flex justify-center px-4 pointer-events-none">
      <nav className="flex items-center gap-4 bg-white p-2 pointer-events-auto border-[3px] border-foreground shadow-hard rounded-wobbly rotate-1">
        <button
          onClick={() => onTabChange("dashboard")}
          className={`flex flex-col items-center justify-center px-6 py-2 transition-transform duration-100 ${
            currentTab === "dashboard"
              ? "-translate-y-1 rotate-[-1deg]"
              : "hover:-translate-y-0.5"
          }`}
        >
          <span className="text-2xl mb-1 filter drop-shadow-[2px_2px_0_rgba(0,0,0,0.2)]">📝</span>
          <span className={`text-sm font-bold font-sans ${currentTab === "dashboard" ? "text-blue decoration-wavy underline" : "text-foreground"}`}>
            Dashboard
          </span>
        </button>

        <div className="w-0.5 h-8 bg-foreground opacity-20 rotate-3 rounded-full"></div>

        <button
          onClick={() => onTabChange("articles")}
          className={`flex flex-col items-center justify-center px-6 py-2 transition-transform duration-100 ${
            currentTab === "articles"
              ? "-translate-y-1 rotate-1"
              : "hover:-translate-y-0.5"
          }`}
        >
          <span className="text-2xl mb-1 filter drop-shadow-[2px_2px_0_rgba(0,0,0,0.2)]">📰</span>
          <span className={`text-sm font-bold font-sans ${currentTab === "articles" ? "text-blue decoration-wavy underline" : "text-foreground"}`}>
            Articles
          </span>
        </button>
      </nav>
    </div>
  );
}
