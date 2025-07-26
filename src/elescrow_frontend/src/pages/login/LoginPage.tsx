import React, { useState } from "react";
import Button from "../../shared/components/Button";
import Input from "../../shared/components/Input";
import Link from "../../shared/components/Link";

function LoginPage(): JSX.Element {
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");

  const handleSignIn = (event: any) => {
    event.preventDefault();
    console.log("Sign in:", { email, password });
  };

  return (
    <div className="min-h-screen bg-primary flex items-center justify-center">
      <div className="flex flex-col items-center gap-8 w-full max-w-md px-6">
        <div className="flex flex-col items-center gap-2">
          <h1 className="text-4xl font-semibold text-primary-text">EleScrow</h1>
          <p className="text-lg text-primary-text">Sign in to EleScrow.</p>
        </div>

        <div className="flex flex-col gap-6 w-full">
          <Input
            label="Email"
            type="email"
            placeholder="email@gmail.com"
            value={email}
            onChange={setEmail}
          />

          <Input
            label="Password"
            type="password"
            placeholder="password"
            value={password}
            onChange={setPassword}
          />

          <div className="text-left">
            <span className="text-secondary-text">Don't have an account? </span>
            <Link href="/register">Sign Up</Link>
          </div>

          <Button
            variant="outlined"
            className="w-full py-3"
            onClick={handleSignIn}
          >
            Sign In
          </Button>
        </div>
      </div>
    </div>
  );
}

export default LoginPage;
