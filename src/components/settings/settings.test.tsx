import { describe, it, expect, beforeEach, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { AudioFeedback } from "./AudioFeedback";
import { GeneralSettings } from "./general/GeneralSettings";
import { useSettingsStore } from "../../stores/settingsStore";
import { useModelStore } from "../../stores/modelStore";
import { commands } from "@/bindings";

const mockedCommands = vi.mocked(commands);

// Seed stores with realistic data before each test
beforeEach(async () => {
  vi.clearAllMocks();

  // Initialize settings store so components have data
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
  await useSettingsStore.getState().initialize();

  // Seed model store with no model selected (simplest case)
  useModelStore.setState({
    models: [],
    currentModel: "",
    downloadingModels: {},
    extractingModels: {},
    downloadProgress: {},
    downloadStats: {},
    loading: false,
    error: null,
    hasAnyModels: false,
    isFirstRun: false,
    initialized: true,
  });
});

describe("AudioFeedback", () => {
  it("renders with toggle in correct state", () => {
    render(<AudioFeedback />);

    // The label text comes from i18n mock (returns the key)
    expect(
      screen.getByText("settings.sound.audioFeedback.label"),
    ).toBeInTheDocument();

    // The toggle should be checked (default mock: audio_feedback = true)
    const checkbox = screen.getByRole("checkbox");
    expect(checkbox).toBeChecked();
  });

  it("calls updateSetting when toggled off", async () => {
    const user = userEvent.setup();
    render(<AudioFeedback />);

    const checkbox = screen.getByRole("checkbox");
    await user.click(checkbox);

    expect(mockedCommands.changeAudioFeedbackSetting).toHaveBeenCalledWith(
      false,
    );
  });
});

describe("GeneralSettings", () => {
  it("renders section titles", () => {
    render(<GeneralSettings />);

    // i18n mock returns keys as-is
    expect(screen.getByText("settings.general.title")).toBeInTheDocument();
    expect(screen.getByText("settings.sound.title")).toBeInTheDocument();
  });

  it("renders PushToTalk toggle", () => {
    render(<GeneralSettings />);

    expect(
      screen.getByText("settings.general.pushToTalk.label"),
    ).toBeInTheDocument();
  });

  it("renders microphone and audio feedback settings", () => {
    render(<GeneralSettings />);

    expect(
      screen.getByText("settings.sound.audioFeedback.label"),
    ).toBeInTheDocument();
    expect(
      screen.getByText("settings.sound.microphone.title"),
    ).toBeInTheDocument();
  });

  it("contains accessible checkboxes", () => {
    render(<GeneralSettings />);

    const checkboxes = screen.getAllByRole("checkbox");
    // Should have at least PushToTalk, AudioFeedback, MuteWhileRecording
    expect(checkboxes.length).toBeGreaterThanOrEqual(3);
  });
});
