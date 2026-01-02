import { useState, useCallback, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";

export interface WorkLog {
  id: number;
  project: string;
  hours: number;
  date: string;
}

export function useWorkLogs() {
  const [logs, setLogs] = useState<WorkLog[]>([]);

  const fetchLogs = useCallback(async () => {
    const data = await invoke<WorkLog[]>("get_work_logs");
    setLogs(data);
  }, []);

  const addLog = useCallback(
    async (project: string, hours: number) => {
      const data = await invoke<WorkLog[]>("add_work_log", { project, hours });
      setLogs(data);
    },
    []
  );

  useEffect(() => {
    // eslint-disable-next-line react-hooks/set-state-in-effect
    void fetchLogs();
  }, [fetchLogs]);

  return { logs, addLog };
}
