import React from "react";
import { useTranslation } from "react-i18next";
import { SegmentedControl } from "../ui/SegmentedControl";
import { SettingContainer } from "../ui/SettingContainer";
import { useSettings } from "../../hooks/useSettings";

interface EnginePreferenceProps {
  descriptionMode?: "inline" | "tooltip";
  grouped?: boolean;
}

const ENGINE_OPTIONS = [
  { value: "auto", label: "Auto" },
  { value: "whisper", label: "Whisper" },
  { value: "sense_voice", label: "SenseVoice" },
  { value: "parakeet", label: "Parakeet" },
];

export const EnginePreference: React.FC<EnginePreferenceProps> = React.memo(
  ({ descriptionMode = "tooltip", grouped = false }) => {
    const { t } = useTranslation();
    const { getSetting, updateSetting, isUpdating } = useSettings();

    const selectedEngine = (getSetting("preferred_engine") || "auto") as string;

    return (
      <SettingContainer
        title={t("settings.advanced.enginePreference.title", "Preferred Engine")}
        description={t(
          "settings.advanced.enginePreference.description",
          "Select which STT engine to use. Auto uses the model's native engine."
        )}
        descriptionMode={descriptionMode}
        grouped={grouped}
      >
        <SegmentedControl
          options={ENGINE_OPTIONS}
          selectedValue={selectedEngine}
          onSelect={(value) => updateSetting("preferred_engine", value)}
          disabled={isUpdating("preferred_engine")}
        />
      </SettingContainer>
    );
  },
);
