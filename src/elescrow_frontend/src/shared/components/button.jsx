import React from "react";
import { useNavigate } from "react-router-dom";

function Button(props) {
  const navigate = useNavigate();

  const navigateToLink = () => {
    navigate(props.to);
  };

  return (
    <button
      className="bg-secondary px-7 py-2 rounded-lg font-medium hover:bg-secondary-hover transition-colors"
      onClick={navigateToLink}
    >
      {props.children}
    </button>
  );
}

export default Button;
