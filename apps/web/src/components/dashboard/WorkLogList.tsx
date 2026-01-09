import React, { useState } from "react";
import { useWorkLogs } from "../../hooks/useWorkLogs";
import WorkLogItem from "./WorkLogItem";
import { cn } from "../../lib/utils";

export default function WorkLogList() {
  const { workLogs, addWorkLog } = useWorkLogs();
  const [workProject, setWorkProject] = useState("");
  const [workHours, setWorkHours] = useState("");

  const handleAddWorkLog = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!workProject.trim() || !workHours) return;
    await addWorkLog(workProject, parseFloat(workHours));
    setWorkProject("");
    setWorkHours("");
  };

  return (
    <section className="relative">
      <h2 className="text-4xl font-heading font-bold mb-6 rotate-1 inline-block bg-white px-4 py-1 shadow-hard border-2 border-foreground transform translate-x-2">
        Work Log
      </h2>

      <form onSubmit={handleAddWorkLog} className="flex gap-2 mb-8 items-stretch">
        <input
          value={workProject}
          onChange={(e) => setWorkProject(e.target.value)}
          placeholder="Project..."
          aria-label="Project name"
          className={cn(
            "flex-[3] px-4 py-3 bg-white font-sans text-lg outline-none transition-all",
            "border-2 border-foreground rounded-wobbly",
            "shadow-hard focus:shadow-hard-hover focus:translate-x-[2px] focus:translate-y-[2px]",
            "placeholder:text-foreground/40"
          )}
        />
        <input
          type="number"
          value={workHours}
          onChange={(e) => setWorkHours(e.target.value)}
          placeholder="Hrs"
          step="0.5"
          aria-label="Hours worked"
          className={cn(
            "flex-1 min-w-[80px] px-2 py-3 bg-white font-sans text-lg text-center outline-none transition-all",
            "border-2 border-foreground rounded-wobbly",
            "shadow-hard focus:shadow-hard-hover focus:translate-x-[2px] focus:translate-y-[2px]",
            "placeholder:text-foreground/40"
          )}
        />
        <button
          type="submit"
          className={cn(
            "px-6 font-heading font-bold text-lg bg-white border-[3px] border-foreground",
            "rounded-wobbly",
            "shadow-hard hover:bg-blue hover:text-white hover:shadow-hard-hover hover:translate-x-[2px] hover:translate-y-[2px]",
            "active:shadow-none active:translate-x-[4px] active:translate-y-[4px]",
            "transition-all duration-100"
          )}
        >
          Log
        </button>
      </form>

      <div id="work-list" className="space-y-3">
        {workLogs.length === 0 && (
          <div className="text-center py-12 border-2 border-dashed border-foreground/30 rounded-[20px] bg-white/50">
            <p className="font-heading text-2xl text-foreground/50 rotate-1">No logs yet 📝</p>
          </div>
        )}
        {workLogs.map((log) => (
          <WorkLogItem key={log.id} log={log} />
        ))}
      </div>
    </section>
  );
}
