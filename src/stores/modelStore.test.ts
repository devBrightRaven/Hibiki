import { describe, it, expect, beforeEach, vi } from "vitest";
import { useModelStore } from "./modelStore";
import { commands } from "@/bindings";
import type { ModelInfo } from "@/bindings";

const mockedCommands = vi.mocked(commands);

const fakeModel: ModelInfo = {
  id: "whisper-tiny",
  name: "Whisper Tiny",
  engine_type: "whisper",
  size_mb: 75,
  languages: ["en"],
  is_downloaded: true,
  is_downloading: false,
  is_default: false,
  description: "Tiny model",
  download_url: "",
};

beforeEach(() => {
  useModelStore.setState({
    models: [],
    currentModel: "",
    downloadingModels: {},
    extractingModels: {},
    downloadProgress: {},
    downloadStats: {},
    loading: true,
    error: null,
    hasAnyModels: false,
    isFirstRun: false,
    initialized: false,
  });
  vi.clearAllMocks();
});

describe("modelStore", () => {
  describe("loadModels", () => {
    it("populates models from backend", async () => {
      mockedCommands.getAvailableModels.mockResolvedValueOnce({
        status: "ok",
        data: [fakeModel],
      });

      await useModelStore.getState().loadModels();

      expect(useModelStore.getState().models).toHaveLength(1);
      expect(useModelStore.getState().models[0].id).toBe("whisper-tiny");
      expect(useModelStore.getState().loading).toBe(false);
    });

    it("sets error on failure", async () => {
      mockedCommands.getAvailableModels.mockResolvedValueOnce({
        status: "error",
        error: "network error",
      });

      await useModelStore.getState().loadModels();

      expect(useModelStore.getState().error).toContain("network error");
      expect(useModelStore.getState().loading).toBe(false);
    });
  });

  describe("downloadModel", () => {
    it("sets downloading state optimistically", async () => {
      const promise = useModelStore.getState().downloadModel("whisper-tiny");

      expect(useModelStore.getState().isModelDownloading("whisper-tiny")).toBe(
        true,
      );
      expect(
        useModelStore.getState().getDownloadProgress("whisper-tiny"),
      ).toBeDefined();

      await promise;
    });

    it("clears downloading state on failure", async () => {
      mockedCommands.downloadModel.mockResolvedValueOnce({
        status: "error",
        error: "disk full",
      });

      const result = await useModelStore
        .getState()
        .downloadModel("whisper-tiny");

      expect(result).toBe(false);
      expect(useModelStore.getState().isModelDownloading("whisper-tiny")).toBe(
        false,
      );
    });
  });

  describe("selectModel", () => {
    it("updates current model on success", async () => {
      const result = await useModelStore.getState().selectModel("whisper-tiny");

      expect(result).toBe(true);
      expect(useModelStore.getState().currentModel).toBe("whisper-tiny");
      expect(useModelStore.getState().isFirstRun).toBe(false);
    });

    it("sets error on failure", async () => {
      mockedCommands.setActiveModel.mockResolvedValueOnce({
        status: "error",
        error: "model not found",
      });

      const result = await useModelStore.getState().selectModel("invalid");

      expect(result).toBe(false);
      expect(useModelStore.getState().error).toContain("model not found");
    });
  });

  describe("cancelDownload", () => {
    it("clears download state on success", async () => {
      // Start a download first
      useModelStore.setState({
        downloadingModels: { "whisper-tiny": true },
        downloadProgress: {
          "whisper-tiny": {
            model_id: "whisper-tiny",
            downloaded: 50,
            total: 100,
            percentage: 50,
          },
        },
      });

      mockedCommands.getAvailableModels.mockResolvedValueOnce({
        status: "ok",
        data: [],
      });

      const result = await useModelStore
        .getState()
        .cancelDownload("whisper-tiny");

      expect(result).toBe(true);
      expect(useModelStore.getState().isModelDownloading("whisper-tiny")).toBe(
        false,
      );
      expect(
        useModelStore.getState().getDownloadProgress("whisper-tiny"),
      ).toBeUndefined();
    });
  });

  describe("getModelInfo", () => {
    it("returns model by id", () => {
      useModelStore.setState({ models: [fakeModel] });

      expect(useModelStore.getState().getModelInfo("whisper-tiny")).toEqual(
        fakeModel,
      );
    });

    it("returns undefined for unknown id", () => {
      expect(useModelStore.getState().getModelInfo("unknown")).toBeUndefined();
    });
  });

  describe("checkFirstRun", () => {
    it("returns true when no models available", async () => {
      mockedCommands.hasAnyModelsAvailable.mockResolvedValueOnce({
        status: "ok",
        data: false,
      });

      const result = await useModelStore.getState().checkFirstRun();

      expect(result).toBe(true);
      expect(useModelStore.getState().isFirstRun).toBe(true);
    });

    it("returns false when models exist", async () => {
      mockedCommands.hasAnyModelsAvailable.mockResolvedValueOnce({
        status: "ok",
        data: true,
      });

      const result = await useModelStore.getState().checkFirstRun();

      expect(result).toBe(false);
      expect(useModelStore.getState().hasAnyModels).toBe(true);
    });
  });
});
