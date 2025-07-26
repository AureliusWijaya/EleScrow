import React, { useState } from "react";
import Button from "../../shared/components/Button";
import Input from "../../shared/components/Input";
import Link from "../../shared/components/Link";

function RegisterPage(): JSX.Element {
  const [username, setUsername] = useState("");
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [confirmPassword, setConfirmPassword] = useState("");

  const handleSignUp = (event: any) => {
    event.preventDefault();
    console.log("Sign up:", { username, email, password, confirmPassword });
  };

  return (
    <div className="min-h-screen bg-primary flex">
      <div className="w-1/2 bg-gradient-to-t from-secondary to-primary relative flex items-center justify-center">
        <div className="flex flex-col items-center gap-8 text-center z-10">
          <div className="flex flex-col gap-4">
            <h1 className="text-2xl text-primary-text">
              Secure your payments now with{" "}
              <span className="text-secondary font-bold ">EleScrow</span>
            </h1>
            <p className="text-base text-primary-text">
              Create your free account today.
            </p>
          </div>

          <div className="flex flex-col gap-6 relative">
            <div className="relative w-48 h-32 flex items-center justify-center">
              <i className="bi bi-currency-bitcoin text-4xl text-yellow-500 absolute top-8 left-10 transform -rotate-12"></i>
              <i className="mdi mdi-ethereum text-4xl text-white-500 absolute top-5 left-1/2 transform -translate-x-1/2"></i>
              <img
                src="/usdt.svg"
                alt="USDT"
                className="w-8 h-8 absolute top-9 right-10  transform rotate-12"
              />
              <i className="bi bi-credit-card text-6xl text-white absolute bottom-0 left-1/2 transform -translate-x-1/2"></i>
            </div>
          </div>
        </div>

        <div className="absolute bottom-0 left-0 w-full h-32 bg-gradient-to-t from-primary to-transparent"></div>
      </div>

      <div className="w-1/2 bg-primary flex items-center justify-center">
        <div className="flex flex-col items-center gap-8 w-full max-w-md py-40">
          <div className="flex flex-col items-center gap-2">
            <h1 className="text-2xl text-primary-text">EleScrow</h1>
            <p className="text-sm text-primary-text">Sign up to EleScrow.</p>
          </div>

          <div className="flex flex-col gap-6 w-2/3">
            <Input
              label="Username"
              type="text"
              placeholder="Username"
              value={username}
              onChange={setUsername}
              size="sm"
            />

            <Input
              label="Email"
              type="email"
              placeholder="email@gmail.com"
              value={email}
              onChange={setEmail}
              size="sm"
            />

            <Input
              label="Password"
              type="password"
              placeholder="Password"
              value={password}
              onChange={setPassword}
              size="sm"
            />

            <div className="text-left">
              <span className="text-primary-text text-sm">
                Have an account? <Link href="/login">Sign In</Link>
              </span>
            </div>

            <Button
              variant="outlined"
              className="w-full py-3"
              onClick={handleSignUp}
            >
              Sign Up
            </Button>
          </div>
        </div>
      </div>
    </div>
  );
}

export default RegisterPage;
