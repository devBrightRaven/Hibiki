import { describe, it, expect } from "vitest";
import { commands } from "@/bindings";

describe("bindings mock", () => {
  it("getAppSettings returns default settings", async () => {
    const result = await commands.getAppSettings();
    expect(result.status).toBe("ok");
    if (result.status === "ok") {
      expect(result.data.push_to_talk).toBe(false);
      expect(result.data.audio_feedback).toBe(true);
      expect(result.data.app_language).toBe("en");
    }
  });

  it("setting commands return ok", async () => {
    const result = await commands.changePttSetting(true);
    expect(result.status).toBe("ok");
  });

  it("getAvailableModels returns empty array", async () => {
    const result = await commands.getAvailableModels();
    expect(result.status).toBe("ok");
    if (result.status === "ok") {
      expect(result.data).toEqual([]);
    }
  });
});
