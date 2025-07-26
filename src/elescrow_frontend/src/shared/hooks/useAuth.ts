import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import { useLoggedInUserPrincipal } from "../store/user-store";
import { icpService } from "../services/icp-service";
import { notifications } from "@mantine/notifications";

export const useAuth = (redirectTo: string = "/") => {
  const navigate = useNavigate();
  const loggedInPrincipal = useLoggedInUserPrincipal();
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

  useEffect(() => {
    if (!isAuthenticated) {
      notifications.show({
        title: "Authentication Required",
        message: "Please connect your wallet first",
        color: "yellow",
        icon: "🔐",
      });
      navigate(redirectTo);
    }
  }, [isAuthenticated, navigate, redirectTo]);

  return {
    isAuthenticated,
    loggedInPrincipal,
  };
};