import React from "react";

interface SegmentOption {
  value: string;
  label: string;
}

interface SegmentedControlProps {
  options: SegmentOption[];
  selectedValue: string;
  onSelect: (value: string) => void;
  disabled?: boolean;
}

export const SegmentedControl: React.FC<SegmentedControlProps> = ({
  options,
  selectedValue,
  onSelect,
  disabled = false,
}) => {
  return (
    <div
      className="inline-flex rounded-lg bg-mid-gray/20 p-0.5"
      role="radiogroup"
    >
      {options.map((option) => (
        <button
          key={option.value}
          role="radio"
          aria-checked={option.value === selectedValue}
          className={`px-3 py-1.5 text-sm font-medium rounded-md transition-colors focus:outline-none focus:ring-2 focus:ring-logo-primary ${
            option.value === selectedValue
              ? "bg-background-ui text-white shadow-sm"
              : "text-light-gray hover:text-white"
          } ${disabled ? "opacity-50 cursor-not-allowed" : "cursor-pointer"}`}
          onClick={() => !disabled && onSelect(option.value)}
          disabled={disabled}
          type="button"
        >
          {option.label}
        </button>
      ))}
    </div>
  );
};
