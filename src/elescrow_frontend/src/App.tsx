import React, { useState, useEffect, useRef, useCallback } from "react";
import Navbar from "./shared/components/Navbar";
import HomePage from "./pages/home/HomePage";
import LoginPage from "./pages/login/LoginPage";
import RegisterPage from "./pages/register/RegisterPage";
import { Route, Routes } from "react-router-dom";
import { MantineProvider } from "@mantine/core";
import { Notifications } from "@mantine/notifications";
import "@mantine/core/styles.css";
import "@mantine/notifications/styles.css";
import DashboardPage from "./pages/dashboard/DashboardPage";
import ChatPage from "./pages/chat/ChatPage";
import CreateTransactionPage from "./pages/transactions/CreateTransactionPage";
import TransactionsPage from "./pages/transactions/TransactionsPage";
import TransactionDetailPage from "./pages/transactions/TransactionDetailPage";
import { Slide, ToastContainer } from "react-toastify";

function App(): JSX.Element {
    return (
        <MantineProvider>
            <Notifications />
            <main className="bg-primary text-primary-text text-base">
                <Navbar />
                <Routes>
                    <Route path="/" element={<HomePage />} />
                    <Route path="/login" element={<LoginPage />} />
                    <Route path="/register" element={<RegisterPage />} />
                    <Route path="/dashboard" element={<DashboardPage />} />
                    <Route path="/chat" element={<ChatPage />} />
                    <Route
                        path="/transactions"
                        element={<TransactionsPage />}
                    />
                    <Route
                        path="/transaction/create"
                        element={<CreateTransactionPage />}
                    />
                    <Route
                        path="/transaction/:id"
                        element={<TransactionDetailPage />}
                    />
                </Routes>
                <ToastContainer
                    position="top-right"
                    autoClose={1000}
                    hideProgressBar={false}
                    newestOnTop
                    closeOnClick
                    rtl={false}
                    pauseOnFocusLoss
                    draggable={false}
                    pauseOnHover={false}
                    theme="dark"
                    transition={Slide}
                ></ToastContainer>
            </main>
        </MantineProvider>
    );
}

export default App;
