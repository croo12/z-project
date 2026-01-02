import { useState, useCallback } from "react";
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
    try {
      setWorkLogs(await invoke("get_work_logs"));
    } catch (e) {
      console.error("Failed to fetch work logs", e);
    }
  }, []);

  const addWorkLog = useCallback(async (project: string, hours: number) => {
    try {
      setWorkLogs(
        await invoke("add_work_log", {
          project,
          hours,
        })
      );
    } catch (e) {
      console.error("Failed to add work log", e);
    }
  }, []);

  return { workLogs, refreshWorkLogs, addWorkLog };
}
