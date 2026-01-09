import { useState, useId } from "react";
import { cn } from "../lib/utils";

interface FeedbackFormProps {
  onSubmit: (helpful: boolean, reason: string) => void;
  onCancel: () => void;
}

export default function FeedbackForm({ onSubmit, onCancel }: FeedbackFormProps) {
  const [reason, setReason] = useState("");
  const [showReasonError, setShowReasonError] = useState(false);
  const errorId = useId();

  const handleSubmit = (helpful: boolean) => {
    if (!reason.trim()) {
      setShowReasonError(true);
      return;
    }
    setShowReasonError(false);
    onSubmit(helpful, reason);
  };

  return (
    <div className="flex flex-col gap-2 mt-4 bg-gray-50 p-4 border-2 border-dashed border-foreground/30 rounded-wobbly rotate-1">
      <div className="flex flex-wrap gap-2 items-center">
        <input
          value={reason}
          onChange={(e) => {
            setReason(e.target.value);
            setShowReasonError(false);
          }}
          placeholder="Reason..."
          autoFocus
          aria-label="Reason for feedback"
          aria-invalid={showReasonError}
          aria-describedby={showReasonError ? errorId : undefined}
          className={cn(
            "flex-grow px-3 py-2 bg-white font-sans text-sm outline-none transition-all",
            "border-2 border-foreground rounded-wobbly",
            "focus:border-blue focus:ring-2 focus:ring-blue/10",
            showReasonError ? "border-accent bg-accent/5" : ""
          )}
        />
        <button
          onClick={() => handleSubmit(true)}
          aria-label="Mark as helpful"
          title="Mark as helpful"
          className="p-2 border-2 border-foreground bg-white hover:bg-green-100 rounded-full transition-transform active:scale-95 shadow-[2px_2px_0_0_#2d2d2d] hover:shadow-none hover:translate-x-[2px] hover:translate-y-[2px]"
        >
          👍
        </button>
        <button
          onClick={() => handleSubmit(false)}
          aria-label="Mark as not helpful"
          title="Mark as not helpful"
          className="p-2 border-2 border-foreground bg-white hover:bg-red-100 rounded-full transition-transform active:scale-95 shadow-[2px_2px_0_0_#2d2d2d] hover:shadow-none hover:translate-x-[2px] hover:translate-y-[2px]"
        >
          👎
        </button>
        <button
          onClick={onCancel}
          className="px-3 py-2 text-sm font-sans font-bold text-foreground/60 hover:text-foreground hover:underline decoration-wavy"
        >
          Cancel
        </button>
      </div>
      {showReasonError && (
        <p
          id={errorId}
          className="text-accent text-sm font-bold font-sans flex items-center gap-1"
          role="alert"
        >
          <span className="text-lg">!</span> Please provide a reason.
        </p>
      )}
    </div>
  );
}
