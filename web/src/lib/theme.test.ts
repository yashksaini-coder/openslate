import { describe, it, expect, vi, beforeEach } from "vitest";

// Mock localStorage globally since Node.js/jsdom might not expose it directly in all scopes
const localStorageMock = (() => {
  let store: Record<string, string> = {};
  return {
    getItem: vi.fn((key: string) => store[key] || null),
    setItem: vi.fn((key: string, value: string) => {
      store[key] = value.toString();
    }),
    removeItem: vi.fn((key: string) => {
      delete store[key];
    }),
    clear: vi.fn(() => {
      store = {};
    }),
  };
})();

vi.stubGlobal("localStorage", localStorageMock);

// Mock the API wrapper
vi.mock("$lib/api", () => ({
  api: vi.fn(),
}));

describe("Theme State Module", () => {
  beforeEach(() => {
    vi.resetModules(); // Reset module state between tests
    localStorageMock.clear();
    if (typeof document !== "undefined" && document.documentElement) {
      document.documentElement.removeAttribute("data-theme");
    }
    vi.clearAllMocks();
  });

  it("should initialize with 'light' if localStorage is empty", async () => {
    const { getTheme } = await import("./theme.svelte");
    expect(getTheme()).toBe("light");
    expect(document.documentElement.getAttribute("data-theme")).toBe("light");
  });

  it("should initialize with cached theme from localStorage", async () => {
    localStorageMock.setItem("openslate-theme", "dark");
    const { getTheme } = await import("./theme.svelte");
    expect(getTheme()).toBe("dark");
    expect(document.documentElement.getAttribute("data-theme")).toBe("dark");
  });

  it("setTheme should update state, localStorage, DOM, and call API", async () => {
    const { api } = await import("$lib/api");
    const { setTheme, getTheme } = await import("./theme.svelte");

    // Mock API call to succeed
    vi.mocked(api).mockResolvedValueOnce(new Response());

    await setTheme("nord");

    expect(getTheme()).toBe("nord");
    expect(localStorageMock.setItem).toHaveBeenCalledWith("openslate-theme", "nord");
    expect(document.documentElement.getAttribute("data-theme")).toBe("nord");
    expect(api).toHaveBeenCalledWith("/api/preferences", {
      method: "PUT",
      body: JSON.stringify({ theme: "nord" }),
    });
  });

  it("loadFromServer should update local state when server has a valid theme", async () => {
    const { api } = await import("$lib/api");
    const { loadFromServer, getTheme } = await import("./theme.svelte");

    // Mock API call returning a theme
    vi.mocked(api).mockResolvedValueOnce(
      new Response(JSON.stringify({ theme: "tokyo-night" }))
    );

    await loadFromServer();

    expect(getTheme()).toBe("tokyo-night");
    expect(localStorageMock.setItem).toHaveBeenCalledWith("openslate-theme", "tokyo-night");
    expect(document.documentElement.getAttribute("data-theme")).toBe("tokyo-night");
  });

  it("loadFromServer should keep cached theme if server call fails or is invalid", async () => {
    localStorageMock.setItem("openslate-theme", "sepia");

    const { api } = await import("$lib/api");
    const { loadFromServer, getTheme } = await import("./theme.svelte");

    // Mock API failure
    vi.mocked(api).mockRejectedValueOnce(new Error("Network Error"));

    await loadFromServer();

    expect(getTheme()).toBe("sepia");
  });
});
