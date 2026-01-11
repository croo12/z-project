import React, { useState } from "react";
import { cn } from "../../lib/utils";
import ServerStatus from "./ServerStatus";
import AITester from "./AITester";

const DebugOverlay: React.FC = () => {
  const [isOpen, setIsOpen] = useState(false);

  return (
    <div className="fixed top-0 left-0 w-full z-[9999] pointer-events-none font-mono text-xs">
      {/* Header / Toggle Button */}
      <div className="flex justify-center pointer-events-auto">
        <button
          onClick={() => setIsOpen(!isOpen)}
          className={cn(
            "px-3 py-1 bg-black/80 text-white rounded-b-lg shadow-lg backdrop-blur-sm border-x border-b border-white/20 transition-all duration-200",
            "hover:bg-blue-600/90 hover:scale-105 active:scale-95"
          )}
        >
          {isOpen ? "Close Debug" : "🛠️ Debug"}
        </button>
      </div>

      {/* Expanded Panel */}
      {isOpen && (
        <div className="bg-black/85 text-white backdrop-blur-md border-b border-white/10 shadow-2xl transition-all animate-in slide-in-from-top-2 pointer-events-auto">
          <div className="max-w-md mx-auto p-4 space-y-4 max-h-[50vh] overflow-y-auto">
            {/* Section 1: Server Status */}
            <ServerStatus />

            {/* Section 2: AI Test */}
            <AITester />

            {/* Environment Info */}
            <div className="text-[10px] text-white/30 text-center mt-2">
              Env: {import.meta.env.MODE} | Platform: Android
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

export default DebugOverlay;
