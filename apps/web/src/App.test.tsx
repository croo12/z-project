import { render, screen } from "@testing-library/react";
import { describe, it, expect, vi } from "vitest";
import App from "./App";

// Mock Tauri invoke
vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(() => Promise.resolve([])),
}));

describe("App", () => {
  it("renders without crashing", () => {
    render(<App />);
    // Just verify something exists or just that it doesn't crash
    expect(screen).toBeDefined();
  });
});
