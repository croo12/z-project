import React, { useState } from "react";
import { useWorkLogs } from "../../hooks/useWorkLogs";
import WorkLogItem from "./WorkLogItem";

export default function WorkLogList() {
  const { workLogs, addWorkLog } = useWorkLogs();
  const [workProject, setWorkProject] = useState("");
  const [workHours, setWorkHours] = useState("");

  const handleAddWorkLog = async (e: React.FormEvent) => {
    e.preventDefault();
    await addWorkLog(workProject, parseFloat(workHours));
    setWorkProject("");
    setWorkHours("");
  };

  return (
    <section>
      <h2>Work Log</h2>
      <form onSubmit={handleAddWorkLog}>
        <input
          value={workProject}
          onChange={(e) => setWorkProject(e.target.value)}
          placeholder="Project"
          aria-label="Project name"
        />
        <input
          type="number"
          value={workHours}
          onChange={(e) => setWorkHours(e.target.value)}
          placeholder="h"
          step="0.5"
          style={{ maxWidth: "60px" }}
          aria-label="Hours worked"
        />
        <button type="submit">Log</button>
      </form>
      <div id="work-list">
        {workLogs.length === 0 && (
          <div
            className="empty-state"
            style={{ textAlign: "center", padding: "1rem", color: "#888" }}
          >
            <p>No logs yet 📝</p>
          </div>
        )}
        {workLogs.map((log) => (
          <WorkLogItem key={log.id} log={log} />
        ))}
      </div>
    </section>
  );
}
