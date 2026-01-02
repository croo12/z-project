import { useState } from "react";

interface FeedbackFormProps {
  onSubmit: (helpful: boolean, reason: string) => void;
  onCancel: () => void;
}

export default function FeedbackForm({ onSubmit, onCancel }: FeedbackFormProps) {
  const [reason, setReason] = useState("");
  const [showReasonError, setShowReasonError] = useState(false);

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
        />
        <button onClick={() => handleSubmit(true)}>👍</button>
        <button onClick={() => handleSubmit(false)}>👎</button>
        <button onClick={onCancel} style={{ background: "#888" }}>
          Cancel
        </button>
      </div>
      {showReasonError && (
        <p style={{ color: "red", fontSize: "0.8rem", marginTop: "0.25rem" }}>
          Please provide a reason.
        </p>
      )}
    </>
  );
}
