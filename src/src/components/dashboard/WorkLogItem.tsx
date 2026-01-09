import { memo } from "react";
import type { WorkLog } from "../../types";

interface WorkLogItemProps {
  log: WorkLog;
}

const WorkLogItem = memo(function WorkLogItem({ log }: WorkLogItemProps) {
  return (
    <div className="card log-item">
      <span className="log-project">{log.project}</span>
      <span className="log-details">
        {log.hours}h ({log.date})
      </span>
    </div>
  );
});

export default WorkLogItem;
