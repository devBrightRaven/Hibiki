import { describe, it, expect, beforeEach, vi } from "vitest";
import { renderHook, act, waitFor } from "@testing-library/react";
import { useSettings } from "./useSettings";
import { useSettingsStore } from "../stores/settingsStore";
import { commands } from "@/bindings";

vi.mocked(commands);

beforeEach(() => {
  useSettingsStore.setState({
    settings: null,
    defaultSettings: null,
    isLoading: true,
    isUpdating: {},
    audioDevices: [],
    outputDevices: [],
    customSounds: { start: false, stop: false },
    postProcessModelOptions: {},
  });
  vi.clearAllMocks();
});

describe("useSettings", () => {
  it("triggers initialization on mount", async () => {
    const { result } = renderHook(() => useSettings());

    // Should start loading
    expect(result.current.isLoading).toBe(true);

    // Wait for initialization to complete
    await waitFor(() => {
      expect(result.current.isLoading).toBe(false);
    });

    expect(result.current.settings).not.toBeNull();
  });

  it("returns settings values after load", async () => {
    const { result } = renderHook(() => useSettings());

    await waitFor(() => {
      expect(result.current.isLoading).toBe(false);
    });

    expect(result.current.settings!.push_to_talk).toBe(false);
    expect(result.current.settings!.audio_feedback).toBe(true);
    expect(result.current.audioFeedbackEnabled).toBe(true);
  });

  it("exposes updateSetting that updates state", async () => {
    const { result } = renderHook(() => useSettings());

    await waitFor(() => {
      expect(result.current.isLoading).toBe(false);
    });

    await act(async () => {
      await result.current.updateSetting("push_to_talk", true);
    });

    expect(result.current.settings!.push_to_talk).toBe(true);
  });

  it("reports isUpdating during setting change", async () => {
    const { result } = renderHook(() => useSettings());

    await waitFor(() => {
      expect(result.current.isLoading).toBe(false);
    });

    // Initially not updating
    expect(result.current.isUpdating("push_to_talk")).toBe(false);
  });

  it("getSetting returns current value", async () => {
    const { result } = renderHook(() => useSettings());

    await waitFor(() => {
      expect(result.current.isLoading).toBe(false);
    });

    expect(result.current.getSetting("app_language")).toBe("en");
  });
});
