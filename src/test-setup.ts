import "@testing-library/jest-dom/vitest";
import { vi } from "vitest";

// Mock Tauri bindings globally — all tests get the mock by default.
// Individual tests can override specific commands with vi.mocked(...).
vi.mock("@/bindings");

// Stub window.__TAURI_INTERNALS__ to prevent runtime errors
// from any code that checks for the Tauri environment.
Object.defineProperty(window, "__TAURI_INTERNALS__", {
  value: {
    invoke: vi.fn(),
    transformCallback: vi.fn(),
  },
  writable: true,
});

// Stub Tauri OS plugin to prevent runtime errors from useOsType.
vi.mock("@tauri-apps/plugin-os", () => ({
  type: () => "windows",
  platform: () => "win32",
  arch: () => "x86_64",
  version: () => "10.0.0",
  locale: () => "en-US",
}));

// Stub Tauri event API used by stores
vi.mock("@tauri-apps/api/event", () => ({
  listen: vi.fn(() => Promise.resolve(() => {})),
  emit: vi.fn(),
  once: vi.fn(() => Promise.resolve(() => {})),
}));

// Stub i18next to return keys as-is (avoids loading translation files in tests).
vi.mock("react-i18next", () => ({
  useTranslation: () => ({
    t: (key: string) => key,
    i18n: { changeLanguage: vi.fn(), language: "en" },
  }),
  Trans: ({ children }: { children: React.ReactNode }) => children,
  initReactI18next: { type: "3rdParty", init: vi.fn() },
}));
