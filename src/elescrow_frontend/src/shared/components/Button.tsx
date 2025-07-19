import React from "react";

function Button(props: any): JSX.Element {
  return (
    <button
      className={
        "bg-secondary px-7 py-2 rounded-lg font-medium hover:bg-secondary-hover transition-colors " +
        props.className
      }
      onClick={(event) => (props.click ? props.click(event) : "")}
    >
      {props.children}
    </button>
  );
}

export default Button;
