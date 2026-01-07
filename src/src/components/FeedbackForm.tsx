import { useState, useId } from "react";

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
    <>
      <div style={{ display: "flex", gap: "0.5rem", marginTop: "0.5rem" }}>
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
        />
        <button
          onClick={() => handleSubmit(true)}
          aria-label="Mark as helpful"
          title="Mark as helpful"
        >
          👍
        </button>
        <button
          onClick={() => handleSubmit(false)}
          aria-label="Mark as not helpful"
          title="Mark as not helpful"
        >
          👎
        </button>
        <button onClick={onCancel} style={{ background: "#888" }}>
          Cancel
        </button>
      </div>
      {showReasonError && (
        <p
          id={errorId}
          style={{ color: "red", fontSize: "0.8rem", marginTop: "0.25rem" }}
          role="alert"
        >
          Please provide a reason.
        </p>
      )}
    </>
  );
}
