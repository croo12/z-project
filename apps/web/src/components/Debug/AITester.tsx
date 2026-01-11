import React, { useState } from "react";
import { invoke } from "@tauri-apps/api/core";

const AITester: React.FC = () => {
  const [isLoading, setIsLoading] = useState(false);
  const [result, setResult] = useState<string | null>(null);

  const testAI = async () => {
    setIsLoading(true);
    setResult(null);
    try {
      // In Rust: test_ai_connection
      const response = await invoke<string>("test_ai_connection");
      setResult(response);
    } catch (error) {
      console.error("AI Test failed:", error);
      setResult("❌ Error invoking command: " + String(error));
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div className="p-3 rounded bg-white/5 border border-white/10 space-y-2">
      <h3 className="text-white/60 font-bold uppercase tracking-wider text-[10px]">
        AI Configuration Check
      </h3>

      <div className="flex flex-col gap-2">
        <button
          onClick={testAI}
          disabled={isLoading}
          className="bg-purple-600 hover:bg-purple-700 disabled:opacity-50 text-white text-xs py-1.5 px-3 rounded shadow transition-colors font-mono"
        >
          {isLoading ? "Checking..." : "Verify Gemini Key"}
        </button>

        {result && (
          <div className="p-2 bg-black/50 rounded border border-white/10 text-[10px] font-mono whitespace-pre-wrap break-all">
            {result}
          </div>
        )}
      </div>
    </div>
  );
};

export default AITester;
