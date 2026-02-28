import { describe, it, expect, beforeEach, vi } from "vitest";
import { useSettingsStore } from "./settingsStore";
import { commands } from "@/bindings";

const mockedCommands = vi.mocked(commands);

// Reset the zustand store between tests
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

describe("settingsStore", () => {
  describe("initialize", () => {
    it("loads settings, defaults, and custom sounds", async () => {
      await useSettingsStore.getState().initialize();

      expect(mockedCommands.getAppSettings).toHaveBeenCalledOnce();
      expect(mockedCommands.getDefaultSettings).toHaveBeenCalledOnce();
      expect(mockedCommands.checkCustomSounds).toHaveBeenCalledOnce();

      const state = useSettingsStore.getState();
      expect(state.isLoading).toBe(false);
      expect(state.settings).not.toBeNull();
      expect(state.defaultSettings).not.toBeNull();
    });

    it("sets isLoading to false even on error", async () => {
      mockedCommands.getAppSettings.mockResolvedValueOnce({
        status: "error",
        error: "fail",
      });

      await useSettingsStore.getState().initialize();

      expect(useSettingsStore.getState().isLoading).toBe(false);
    });
  });

  describe("refreshSettings", () => {
    it("normalizes nullable fields to defaults", async () => {
      mockedCommands.getAppSettings.mockResolvedValueOnce({
        status: "ok",
        data: {
          ...mockedCommands.getAppSettings.mock.results[0]
            ? (await mockedCommands.getAppSettings()).data as any
            : {},
          always_on_microphone: undefined as any,
          selected_microphone: null,
          clamshell_microphone: null,
          selected_output_device: null,
        },
      } as any);

      // Seed a valid default first so the spread works
      await useSettingsStore.getState().initialize();

      // Now refresh with the nullable mock
      mockedCommands.getAppSettings.mockResolvedValueOnce({
        status: "ok",
        data: {
          ...(await mockedCommands.getAppSettings()).data,
          always_on_microphone: undefined as any,
          selected_microphone: null,
          selected_output_device: null,
        },
      } as any);

      await useSettingsStore.getState().refreshSettings();

      const s = useSettingsStore.getState().settings!;
      expect(s.always_on_microphone).toBe(false);
      expect(s.selected_microphone).toBe("Default");
      expect(s.selected_output_device).toBe("Default");
    });
  });

  describe("updateSetting", () => {
    it("calls the correct command and updates state optimistically", async () => {
      await useSettingsStore.getState().initialize();

      await useSettingsStore.getState().updateSetting("push_to_talk", true);

      expect(mockedCommands.changePttSetting).toHaveBeenCalledWith(true);
      expect(useSettingsStore.getState().settings!.push_to_talk).toBe(true);
    });

    it("rolls back on command failure", async () => {
      await useSettingsStore.getState().initialize();

      const original = useSettingsStore.getState().settings!.audio_feedback;

      mockedCommands.changeAudioFeedbackSetting.mockRejectedValueOnce(
        new Error("network error"),
      );

      await useSettingsStore
        .getState()
        .updateSetting("audio_feedback", !original);

      // Should roll back to original
      expect(useSettingsStore.getState().settings!.audio_feedback).toBe(
        original,
      );
    });

    it("tracks updating state during the call", async () => {
      await useSettingsStore.getState().initialize();

      // Make the command hang
      let resolve: () => void;
      mockedCommands.changePttSetting.mockReturnValueOnce(
        new Promise((r) => {
          resolve = () => r({ status: "ok", data: null });
        }),
      );

      const promise = useSettingsStore
        .getState()
        .updateSetting("push_to_talk", true);

      expect(useSettingsStore.getState().isUpdating["push_to_talk"]).toBe(true);

      resolve!();
      await promise;

      expect(useSettingsStore.getState().isUpdating["push_to_talk"]).toBe(
        false,
      );
    });
  });

  describe("getSetting", () => {
    it("returns undefined when settings not loaded", () => {
      expect(useSettingsStore.getState().getSetting("push_to_talk")).toBeUndefined();
    });

    it("returns the current value after initialization", async () => {
      await useSettingsStore.getState().initialize();

      expect(useSettingsStore.getState().getSetting("push_to_talk")).toBe(false);
      expect(useSettingsStore.getState().getSetting("app_language")).toBe("en");
    });
  });

  describe("refreshAudioDevices", () => {
    it("prepends Default device", async () => {
      mockedCommands.getAvailableMicrophones.mockResolvedValueOnce({
        status: "ok",
        data: [
          { index: "1", name: "USB Mic", is_default: false },
        ],
      });

      await useSettingsStore.getState().refreshAudioDevices();

      const devices = useSettingsStore.getState().audioDevices;
      expect(devices[0].name).toBe("Default");
      expect(devices).toHaveLength(2);
    });

    it("falls back to Default-only list on error", async () => {
      mockedCommands.getAvailableMicrophones.mockRejectedValueOnce(
        new Error("fail"),
      );

      await useSettingsStore.getState().refreshAudioDevices();

      expect(useSettingsStore.getState().audioDevices).toHaveLength(1);
      expect(useSettingsStore.getState().audioDevices[0].name).toBe("Default");
    });
  });
});
