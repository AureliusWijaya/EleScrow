import React from "react";

function Select(props: any): JSX.Element {
  const getSizeClasses = () => {
    switch (props.size) {
      case "sm":
        return "px-3 py-2 text-xs";
      case "lg":
        return "px-6 py-4 text-base";
      case "xl":
        return "px-8 py-5 text-lg";
      default:
        return "px-4 py-3 text-sm";
    }
  };

  const getLabelSizeClasses = () => {
    switch (props.size) {
      case "sm":
        return "text-xs";
      case "lg":
        return "text-base";
      case "xl":
        return "text-lg";
      default:
        return "text-sm";
    }
  };

  return (
    <div className={`flex flex-col gap-2 ${props.className || ""}`}>
      {props.label && (
        <label className={`text-primary-text font-medium ${getLabelSizeClasses()}`}>
          {props.label}
        </label>
      )}
      <select
        value={props.value}
        onChange={(e) => props.onChange(e.target.value)}
        className={`w-full bg-transparent border border-secondary-text rounded-lg text-primary-text focus:outline-none focus:border-secondary-hover ${getSizeClasses()}`}
      >
        {props.children}
      </select>
    </div>
  );
}

export default Select; 