import React from "react";

function Button(props: any): JSX.Element {
  const baseClasses = "px-7 py-2 rounded-lg font-medium transition-colors";
  
  const getVariantClasses = () => {
    if (props.variant === "outlined") {
      return "border border-secondary text-secondary hover:bg-secondary hover:text-primary-text";
    }
    return "bg-secondary hover:bg-secondary-hover text-primary-text";
  };

  return (
    <button
      className={`${baseClasses} ${getVariantClasses()} ${props.className || ""}`}
      onClick={(event) => (props.click ? props.click(event) : "")}
    >
      {props.children}
    </button>
  );
}

export default Button;
