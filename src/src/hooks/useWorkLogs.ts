import { useState, useCallback, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";

export interface WorkLog {
  id: number;
  project: string;
  hours: number;
  date: string;
}

export function useWorkLogs() {
  const [workLogs, setWorkLogs] = useState<WorkLog[]>([]);

  const refreshWorkLogs = useCallback(async () => {
    setWorkLogs(await invoke("get_work_logs"));
  }, []);

  useEffect(() => {
    // eslint-disable-next-line react-hooks/set-state-in-effect
    void refreshWorkLogs();
  }, [refreshWorkLogs]);

  const addWorkLog = async (project: string, hours: number) => {
    if (!project || !hours) return;
    await invoke("add_work_log", { project, hours });
    await refreshWorkLogs();
  };

  return { workLogs, addWorkLog };
}
