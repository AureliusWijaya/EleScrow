import React, { useState, useEffect } from "react";
import { Link, useNavigate } from "react-router-dom";
import Button from "./Button";
import { useLoggedInUserPrincipal, useUserStoreActions } from "../store/user-store";
import { icpService } from "../services/icp-service";

function Navbar(): JSX.Element {
  const navigate = useNavigate();
  const loggedInPrincipal = useLoggedInUserPrincipal();
  const { clearLoggedInUserPrincipal } = useUserStoreActions();
  const [isAuthenticated, setIsAuthenticated] = useState(false);

  useEffect(() => {
    const checkAuth = () => {
      const auth = loggedInPrincipal || icpService.isAuthenticated();
      setIsAuthenticated(auth);
    };
    
    checkAuth();
    // Check auth status every second to catch logout changes
    const interval = setInterval(checkAuth, 1000);
    
    return () => clearInterval(interval);
  }, [loggedInPrincipal]);

  const onSignUpButtonClick = () => {
    navigate("/register");
  };

  const onLogout = async () => {
    try {
      await icpService.logout();
      clearLoggedInUserPrincipal();
      setIsAuthenticated(false); // Force immediate update
      navigate("/");
    } catch (error) {
      console.error("Logout failed:", error);
    }
  };

  // Show different navbar based on authentication status
  if (isAuthenticated) {
    return (
      <div className="flex w-full items-center justify-between py-5 px-7 bg-[#070312] border-[#3F3D3D] border-b z-50 mb-50">
        <Link to="/" className="text-3xl">
          EleScrow
        </Link>
        <div className="flex items-center gap-6">
          <Link
            to="/dashboard"
            className="text-primary-text hover:text-secondary transition-colors"
          >
            Dashboard
          </Link>
          <Link
            to="/transactions"
            className="text-primary-text hover:text-secondary transition-colors"
          >
            Transactions
          </Link>
          <Link
            to="/transaction/create"
            className="text-primary-text hover:text-secondary transition-colors"
          >
            Create Transaction
          </Link>
          <div className="flex items-center gap-2">
            <span className="text-sm text-secondary-text">
              {loggedInPrincipal?.toText().slice(0, 8)}...
            </span>
            <Button variant="outlined" click={onLogout}>
              Logout
            </Button>
          </div>
        </div>
      </div>
    );
  }

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
