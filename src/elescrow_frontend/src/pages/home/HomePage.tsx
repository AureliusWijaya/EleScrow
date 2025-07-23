import React, { useState } from "react";
import Button from "../../shared/components/Button";
import { Stepper } from "@mantine/core";

import { AuthClient } from "@dfinity/auth-client";
import { Principal } from '@dfinity/principal';

function HomePage(): JSX.Element {
  const [active, setActive] = useState(0);


const loginICP = async () => {
  console.log("Login initiated");
  try {
    const client = await AuthClient.create();
    console.log("AuthClient created");

    await client.login({
      identityProvider: "https://identity.ic0.app/#authorize",
      onSuccess: async () => {
        const identity = client.getIdentity();
        const principal = identity.getPrincipal();
        console.log("Login successful", principal.toString()," " ,identity);


        document.cookie = `PeanutFundtoken=${principal.toString()}; path=/; secure; samesite=strict`;
      },
      onError: (err) => {
        console.error("Login error in callback", err);
      },
    });
  } catch (error) {
    console.error("LoginICP Error", error);
  }
};


  return (
    <div className="w-full">
      <div className="flex flex-col min-h-screen items-center justify-center relative">
        <div className="flex flex-col gap-10 items-center justify-center px-20 py-32 relative">
          <div className="flex flex-col gap-2 text-center text-6xl font-semibold text-nowrap z-10">
            <span className="w-full">Secure, Controlled Payments.</span>
            <span className="w-full">Powered by Web3.</span>
          </div>

          <div className="flex flex-col text-center text-lg text-secondary-text z-10">
            <span className="w-full">
              EleScrow is a decentralized escrow platform that safeguards your
              transactions.
            </span>
            <span className="w-full">
              Send with confidence. Funds are only released when both parties
              agree.
            </span>
          </div>

          <Button className="!px-16 !py-4 z-10" click={loginICP}>Connect Wallet</Button>

          <div className="absolute w-full h-full bg-secondary z-0 rounded-full blur-3xl opacity-30"></div>
        </div>

        <div className="absolute w-[25vw] h-[25vw] -left-[15%] grid-bg rounded-full"></div>
      </div>

      <div className="w-full h-[50vh] relative">
        <div className="absolute w-[25vw] h-[25vw] -right-[15%] grid-bg rounded-full"></div>
      </div>

      <div className="flex flex-col min-h-screen mt-20">
        <div className="flex lg:flex-row flex-col h-1/2 lg:justify-center lg:gap-52 gap-20 items-center">
          <div className="flex flex-col gap-4 items-center">
            <span className="w-full text-5xl font-medium">How It Works</span>
            <i className="bi bi-boxes text-7xl"></i>
          </div>

          <Stepper
            active={active}
            onStepClick={setActive}
            orientation="vertical"
            classNames={{
              stepLabel: "mt-3",
              stepIcon: "!bg-secondary !border-none !text-primary-text",
              step: "!border !border-solid !border-red-950 !h-fit !min-h-0 !px-4 !py-2 rounded-full",
              steps: "gap-4",
              stepWrapper: "h-fit",
              stepBody: "h-fit",
              verticalSeparator: "hidden",
            }}
          >
            <Stepper.Step
              label="Connect Your Identity"
              className={
                active === 0
                  ? "shadow-[rgba(0,0,15,0.5)_0px_0px_20px_0px] shadow-secondary"
                  : ""
              }
            />
            <Stepper.Step
              label="Create a Transaction"
              className={
                active === 1
                  ? "shadow-[rgba(0,0,15,0.5)_0px_0px_20px_0px] shadow-secondary"
                  : ""
              }
            />
            <Stepper.Step
              label="Admin Approval Layer"
              className={
                active === 2
                  ? "shadow-[rgba(0,0,15,0.5)_0px_0px_20px_0px] shadow-secondary"
                  : ""
              }
            />
            <Stepper.Step
              label="Everyone Wins"
              className={
                active === 3
                  ? "shadow-[rgba(0,0,15,0.5)_0px_0px_20px_0px] shadow-secondary"
                  : ""
              }
            />
          </Stepper>
        </div>

        <div className="w-full h-1/2 relative">
          <div className="absolute w-[25vw] h-[25vw] -left-[15%] grid-bg rounded-full"></div>
        </div>
      </div>

      <div className="flex flex-col min-h-screen gap-10 items-center relative">
        <span className="text-5xl">Why EleScrow?</span>

        <div className="flex w-full justify-center flex-wrap gap-10">
          <div className="flex flex-col bg-[#1A1B23] p-6 items-center justify-center rounded-xl gap-4 w-96 h-72 text-center">
            <i className="bi bi-patch-check text-5xl"></i>
            <span className="text-4xl font-semibold">Trustless by Design</span>
            <span className="text-secondary-text">
              Smart contracts eliminate the need for blind trust.
            </span>
          </div>

          <div className="flex flex-col bg-[#1A1B23] p-6 items-center justify-center rounded-xl gap-4 w-96 h-72 text-center">
            <i className="bi bi-shield-check text-5xl"></i>
            <span className="text-4xl font-semibold">
              Transparent & Auditable
            </span>
            <span className="text-secondary-text">
              Every transaction is verifiable on-chain.
            </span>
          </div>

          <div className="flex flex-col bg-[#1A1B23] p-6 items-center justify-center rounded-xl gap-4 w-96 h-72 text-center">
            <i className="bi bi-shield-check text-5xl"></i>
            <span className="text-4xl font-semibold">
              Admin-Controlled Flow
            </span>
            <span className="text-secondary-text">
              A safety net: transactions only proceed with admin confirmation.
            </span>
          </div>
        </div>

        <div className="absolute w-[25vw] h-[25vw] -right-[15%] bottom-[20%] grid-bg rounded-full"></div>
      </div>
    </div>
  );
}

export default HomePage;
