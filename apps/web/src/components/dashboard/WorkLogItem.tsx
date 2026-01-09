import { memo } from "react";
import type { WorkLog } from "../../hooks/useWorkLogs";
import { cn } from "../../lib/utils";

interface WorkLogItemProps {
  log: WorkLog;
}

const WorkLogItem = memo(function WorkLogItem({ log }: WorkLogItemProps) {
  return (
    <div
      className={cn(
        "flex justify-between items-center bg-white border-2 border-foreground p-4 transition-all duration-200",
        "shadow-[2px_2px_0_0_#2d2d2d] hover:shadow-[3px_3px_0_0_#2d2d2d] hover:-translate-y-[1px]",
        // Slightly less wobbly for list items to stack better
        "rounded-wobbly-md"
      )}
    >
      <span className="font-heading font-bold text-lg">{log.project}</span>
      <div className="flex items-center gap-3">
        <span className="font-sans text-sm text-foreground/60">{log.date}</span>
        <span className="inline-block bg-blue text-white font-bold font-sans px-2 py-1 rounded-sm rotate-2 shadow-sm border border-foreground">
            {log.hours}h
        </span>
      </div>
    </div>
  );
});

export default WorkLogItem;
