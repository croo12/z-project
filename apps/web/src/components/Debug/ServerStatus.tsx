import React, { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { cn } from "../../lib/utils";

const ServerStatus: React.FC = () => {
  const [isHealthy, setIsHealthy] = useState<boolean | null>(null);
  const [lastChecked, setLastChecked] = useState<string>("-");

  const checkHealth = async () => {
    try {
      const healthy = await invoke<boolean>("check_server_health");
      setIsHealthy(healthy);
      setLastChecked(new Date().toLocaleTimeString());
    } catch (error) {
      console.error("Health check failed:", error);
      setIsHealthy(false);
    }
  };

  useEffect(() => {
    // Initial check
    checkHealth();

    // Poll every 10 seconds
    const interval = setInterval(checkHealth, 10000);
    return () => clearInterval(interval);
  }, []);

  return (
    <div className="p-3 rounded bg-white/5 border border-white/10 space-y-2">
      <h3 className="text-white/60 font-bold uppercase tracking-wider text-[10px] flex justify-between items-center">
        Server Connection
        <button
          onClick={checkHealth}
          className="text-[9px] px-1.5 py-0.5 bg-white/10 hover:bg-white/20 rounded ml-2"
        >
          Refresh
        </button>
      </h3>

      <div className="flex items-center gap-2">
        <div
          className={cn(
            "w-3 h-3 rounded-full shadow-inner",
            isHealthy === true
              ? "bg-green-500 shadow-green-500/50"
              : isHealthy === false
                ? "bg-red-500 shadow-red-500/50"
                : "bg-yellow-500 animate-pulse"
          )}
        />
        <div className="font-mono text-xs">
          {isHealthy === true
            ? "ONLINE"
            : isHealthy === false
              ? "OFFLINE/ERROR"
              : "CHECKING..."}
        </div>
      </div>

      <div className="text-[10px] text-white/40">
        Last checked: {lastChecked}
      </div>
    </div>
  );
};

export default ServerStatus;
