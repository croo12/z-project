import { render, screen, fireEvent } from "@testing-library/react";
import { describe, it, expect, vi } from "vitest";
import ArticleList from "./ArticleList";
import type { Article } from "../types";

// Mock Tauri invoke
vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(() => Promise.resolve([])),
}));

const mockArticles: Article[] = [
  {
    id: "1",
    title: "Test Article 1",
    summary: "Summary 1",
    url: "http://example.com/1",
    tags: ["Rust"],
    published_at: "2023-10-27T10:00:00Z",
    image_url: "http://example.com/img1.jpg",
    author: "Author 1",
    feedback: null,
  },
  {
    id: "2",
    title: "Test Article 2",
    summary: "Summary 2",
    url: "http://example.com/2",
    tags: ["React"],
    published_at: "2023-10-27T11:00:00Z",
    image_url: "http://example.com/img2.jpg",
    author: "Author 2",
    feedback: null,
  },
];

describe("ArticleList", () => {
  it("renders articles", () => {
    render(
      <ArticleList
        articles={mockArticles}
        onRefresh={() => {}}
        onFeedbackUpdate={() => {}}
        onSyncArticle={() => {}}
      />
    );
    expect(screen.getByText("Test Article 1")).toBeDefined();
    expect(screen.getByText("Test Article 2")).toBeDefined();
  });

  it("filters articles", () => {
    render(
      <ArticleList
        articles={mockArticles}
        onRefresh={() => {}}
        onFeedbackUpdate={() => {}}
        onSyncArticle={() => {}}
      />
    );

    // Find the Rust filter button
    const rustBtn = screen.getByRole("button", { name: "Rust" });
    fireEvent.click(rustBtn);

    expect(screen.getByText("Test Article 1")).toBeDefined();
    expect(screen.queryByText("Test Article 2")).toBeNull();
  });

  it("uses lazy loading for images", () => {
    render(
      <ArticleList
        articles={mockArticles}
        onRefresh={() => {}}
        onFeedbackUpdate={() => {}}
        onSyncArticle={() => {}}
      />
    );

    const imgs = screen.getAllByRole("img");
    const img1 = imgs.find(
      (img) => img.getAttribute("src") === "http://example.com/img1.jpg"
    );

    expect(img1).toBeDefined();
    expect(img1?.getAttribute("loading")).toBe("lazy");
  });
});
