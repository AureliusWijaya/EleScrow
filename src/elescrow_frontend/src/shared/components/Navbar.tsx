import React, { useEffect, useState } from "react";
import { Link, useNavigate } from "react-router-dom";
import Button from "./Button";
import { useIsAuthenticated, usePrincipal } from "../store/auth-store";
import { useNotificationList } from "../store/notification-store";
import { useLogout } from "../service/auth-service";

function Navbar(): JSX.Element {
    const navigate = useNavigate();
    const { logout } = useLogout();
    const isAuthenticated = useIsAuthenticated();
    const notificationList = useNotificationList();
    const [isUsernameDropdownOpen, setIsUsernameDropdownOpen] =
        useState<boolean>(false);

    const onSignUpClick = () => {
        navigate("/register");
    };

    const toggleUsernameDropdown = () => {
        setIsUsernameDropdownOpen(!isUsernameDropdownOpen);
    };

    const onSignOutClick = async () => {
        await logout();
    };

    const usernameDropdown = (): JSX.Element | null => {
        if (isUsernameDropdownOpen) {
            return (
                <div className="absolute z-10 top-full mt-2 right-0 min-w-32 text-sm rounded-md shadow-sm bg-primary border border-[#3F3D3D]">
                    <div
                        className="w-full py-2 px-3 hover:bg-primary-hover cursor-pointer select-none"
                        onClick={onSignOutClick}
                    >
                        Sign Out
                    </div>
                </div>
            );
        }

        return null;
    };

    const navbarElement = (): JSX.Element => {
        if (!isAuthenticated) {
            return (
                <div className="flex w-full h-[80px] items-center justify-between py-5 px-7 bg-[#070312] border-[#3F3D3D] border-b z-50 mb-50">
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
                        <Button onClick={onSignUpClick}>Sign Up</Button>
                    </div>
                </div>
            );
        }

        return (
            <div className="flex w-full h-[80px] items-center justify-between py-5 px-7 bg-[#070312] border-[#3F3D3D] border-b z-50 mb-50">
                <div className="flex items-center gap-10">
                    <Link to="/" className="text-3xl">
                        EleScrow
                    </Link>
                    <div className="flex items-center gap-5">
                        <Link
                            to="/dashboard"
                            className="hover:text-primary-text-hover transition-colors"
                        >
                            Dashboard
                        </Link>
                        <Link
                            to="/transactions"
                            className="hover:text-primary-text-hover transition-colors"
                        >
                            Transactions
                        </Link>
                    </div>
                </div>
                <div className="flex items-center gap-5">
                    <div className="flex items-center gap-3">
                        <Link to="/notifications">
                            <div className="flex relative w-8 h-8 items-center justify-center border border-gray-500 rounded-lg cursor-pointer hover:border-gray-400 transition-colors">
                                <i className="bi bi-bell"></i>
                                {notificationList.length > 0 ? (
                                    <div className="absolute w-2 h-2 bg-red-900 -top-1 -right-1 rounded-full"></div>
                                ) : (
                                    ""
                                )}
                            </div>
                        </Link>
                        <Link to="/chat">
                            <div className="flex w-8 h-8 items-center justify-center border border-gray-500 rounded-lg cursor-pointer hover:border-gray-400 transition-colors">
                                <i className="bi bi-chat-left-text"></i>
                            </div>
                        </Link>
                    </div>

                    <div className="relative">
                        <div className="flex gap-3 items-center">
                            Username
                            <i
                                className={`bi bi-caret-down-fill text-sm transition-transform cursor-pointer${
                                    isUsernameDropdownOpen ? " rotate-180" : ""
                                }`}
                                onClick={toggleUsernameDropdown}
                            ></i>
                        </div>

                        {usernameDropdown()}
                    </div>
                </div>
            </div>
        );
    };

    return navbarElement();
}

export default Navbar;
