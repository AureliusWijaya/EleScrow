import React from "react";
import { useNavigate } from "react-router-dom";
import { Button } from "../components/ui/button";

const NotFoundPage: React.FC = () => {
  const navigate = useNavigate();

  return (
    <div className="flex flex-col items-center justify-center h-[80vh] space-y-4 p-6 text-center">
      <h1 className="text-6xl font-bold ">404</h1>
      <p className="text-lg text-muted-foreground">
        Oops! The page you were looking for doesn&apos;t exist.
      </p>
      <Button variant="outline" onClick={() => navigate("/")}>
        Go to Home
      </Button>
    </div>
  );
};

export default NotFoundPage;
