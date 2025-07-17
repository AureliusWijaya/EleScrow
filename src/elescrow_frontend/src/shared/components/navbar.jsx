import React from "react";
import { Link } from "react-router-dom";
import Button from "./button";

function Navbar() {
  // Unauthorized
  return (
    <div className="flex items-center justify-between py-5 px-7 bg-[#070312] border-[#3F3D3D] border-b sticky">
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
        <Button to="/register">Sign Up</Button>
      </div>
    </div>
  );
}

export default Navbar;
