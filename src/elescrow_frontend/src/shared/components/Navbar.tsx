import React from "react";
import { Link, useNavigate } from "react-router-dom";
import Button from "./Button";

function Navbar(): JSX.Element {
  const navigate = useNavigate();

  const onSignUpButtonClick = () => {
    navigate("/register");
  };

  // Unauthorized
  return (
    <div className="flex w-full items-center justify-between py-5 px-7 bg-[#070312] border-[#3F3D3D] border-b z-50 mb-50">
      <Link to="/" className="text-3xl">
        EleScrow
      </Link>
      <div className="flex items-center gap-6">
        <Link
          to="/login"
          className="underline hover:text-primary-text-hover transition-colors"
        >
          Have an account?
        </Link>
        <Button click={onSignUpButtonClick}>Sign Up</Button>
      </div>
    </div>
  );
}

export default Navbar;
