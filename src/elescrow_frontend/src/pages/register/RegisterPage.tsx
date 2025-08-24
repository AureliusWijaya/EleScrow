import React, { useState } from "react";
import Button from "../../shared/components/Button";
import Input from "../../shared/components/input/Input";
import Link from "../../shared/components/Link";
import { useLoginToIcp } from "../../shared/service/auth-service";
import { useIsAuthenticated } from "../../shared/store/auth-store";
import Form from "../../shared/components/form/Form";
import { FormValidators } from "../../shared/utils/form/form-validators.c";
import { ToastUtils } from "../../shared/utils/toast/toast-utils";
import SubmitButton from "../../shared/components/SubmitButton";

function RegisterPage(): JSX.Element {
    const { loginToICP } = useLoginToIcp();
    const isAuthenticated = useIsAuthenticated();

    const handleLoginToICP = async (): Promise<void> => {
        await loginToICP();
    };

    const handleSignUp = (data: any): void => {
        console.log("Sign up:", data);
        ToastUtils.createErrorToast("test");
    };

    const icpButton = (): JSX.Element => {
        if (isAuthenticated) {
            return (
                <Button className="w-full py-3 !bg-icp hover:!bg-icp">
                    <div className="flex gap-3 items-center justify-center">
                        Logged In to ICP
                        <i className="bi bi-check2 text-white text-xl"></i>
                    </div>
                </Button>
            );
        }

        return (
            <Button
                className="w-full py-3 !bg-icp hover:!bg-icp-hover"
                onClick={handleLoginToICP}
            >
                <div className="flex gap-3 items-center justify-center">
                    Login to ICP
                    <svg
                        className="text-white"
                        fill="currentColor"
                        width="32"
                        height="32"
                        viewBox="-1.6 -1.6 35.20 35.20"
                        xmlns="http://www.w3.org/2000/svg"
                        stroke="currentColor"
                    >
                        <g id="SVGRepo_bgCarrier" strokeWidth="0"></g>
                        <g
                            id="SVGRepo_tracerCarrier"
                            strokeLinecap="round"
                            strokeLinejoin="round"
                        ></g>
                        <g id="SVGRepo_iconCarrier">
                            <path d="M16 0c8.837 0 16 7.163 16 16s-7.163 16-16 16S0 24.837 0 16 7.163 0 16 0zM9.83 10.5h-.006l-.248.006c-2.626.117-4.808 1.873-5.41 4.198A5.156 5.156 0 004 15.999C4 19.033 6.615 21.5 9.83 21.5c1.34 0 2.803-.667 4.348-1.982a19.024 19.024 0 001.84-1.82v-.001l.13.14c1.595 1.703 3.685 3.523 5.786 3.655l.242.008.248-.006c2.63-.117 4.813-1.877 5.413-4.207l.003-.01c.105-.41.16-.837.16-1.276 0-3.034-2.615-5.501-5.83-5.501-1.34 0-2.803.667-4.348 1.982a19.024 19.024 0 00-1.84 1.82l.026.03-.156-.168c-1.595-1.703-3.685-3.524-5.78-3.656l-.243-.008zm12.34 2.193c1.928 0 3.496 1.484 3.496 3.307 0 1.814-1.569 3.296-3.498 3.308-.048 0-.103-.003-.169-.012-1.215-.442-2.237-1.207-2.508-1.45-.248-.223-.577-.54-.933-.894l-.311-.31-.686-.693a17.474 17.474 0 011.827-1.842c1.374-1.169 2.272-1.414 2.783-1.414zm-12.338 0c.058 0 .128.004.21.014 1.196.45 2.2 1.207 2.467 1.448.354.317.873.829 1.403 1.363l.318.323c.07.07.139.142.209.21a17.474 17.474 0 01-1.827 1.842c-1.374 1.169-2.272 1.414-2.783 1.414-1.927 0-3.495-1.484-3.495-3.307 0-1.814 1.569-3.296 3.498-3.308z"></path>
                        </g>
                    </svg>
                </div>
            </Button>
        );
    };

    return (
        <div className="min-h-page bg-black flex">
            <div className="w-1/2 bg-gradient-to-t from-secondary via-black to-black relative flex items-center justify-center">
                <div className="flex flex-col items-center gap-8 text-center z-10">
                    <div className="flex flex-col gap-4">
                        <h1 className="text-2xl text-primary-text">
                            Secure your payments now with{" "}
                            <span className="text-secondary font-bold ">
                                EleScrow
                            </span>
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
                <div className="flex flex-col items-center gap-8 w-full max-w-md">
                    <div className="flex flex-col items-center gap-2">
                        <h1 className="text-2xl text-primary-text">EleScrow</h1>
                        <p className="text-sm text-primary-text">
                            Sign up to EleScrow.
                        </p>
                    </div>

                    <div className="flex flex-col gap-6 w-2/3">
                        <Form submit={handleSignUp}>
                            <Input
                                fcId="username"
                                label="Username"
                                type="text"
                                placeholder="Username"
                                size="sm"
                                validators={[
                                    FormValidators.required,
                                    FormValidators.minLength(5),
                                    FormValidators.maxLength(20),
                                ]}
                            />

                            <Input
                                fcId="displayName"
                                label="Display Name"
                                type="text"
                                placeholder="Display Name"
                                size="sm"
                                validators={[
                                    FormValidators.required,
                                    FormValidators.minLength(5),
                                    FormValidators.maxLength(20),
                                ]}
                            />

                            <Input
                                fcId="email"
                                label="Email"
                                type="email"
                                placeholder="email@gmail.com"
                                size="sm"
                                validators={[
                                    FormValidators.required,
                                    FormValidators.email,
                                ]}
                            />

                            <Input
                                fcId="password"
                                label="Password"
                                type="password"
                                placeholder="Password"
                                size="sm"
                                validators={[
                                    FormValidators.required,
                                    FormValidators.minLength(8),
                                    FormValidators.maxLength(20),
                                    FormValidators.includes(
                                        " ",
                                        false,
                                        "No whitespaces allowed"
                                    ),
                                ]}
                            />

                            {icpButton()}

                            <div className="text-left">
                                <span className="text-primary-text text-sm">
                                    Have an account?{" "}
                                    <Link href="/login">Sign In</Link>
                                </span>
                            </div>

                            <SubmitButton
                                variant="outlined"
                                className="w-full py-3"
                            >
                                Sign Up
                            </SubmitButton>
                        </Form>
                    </div>
                </div>
            </div>
        </div>
    );
}

export default RegisterPage;
