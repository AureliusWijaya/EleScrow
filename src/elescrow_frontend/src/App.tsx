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
import { icpService } from "./shared/services/icp-service";
import { useUserStoreActions } from "./shared/store/user-store";

function App(): JSX.Element {
  const { setLoggedInUserPrincipal } = useUserStoreActions();

  useEffect(() => {
    const initializeApp = async () => {
      await icpService.initialize();
      
      if (icpService.isAuthenticated()) {
        const principal = icpService.getPrincipal();
        if (principal) {
          setLoggedInUserPrincipal(principal);
        }
      }
    };
    
    initializeApp();
  }, [setLoggedInUserPrincipal]);

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
          <Route path="/transactions" element={<TransactionsPage />} />
          <Route
            path="/transaction/create"
            element={<CreateTransactionPage />}
          />
          <Route path="/transaction/:id" element={<TransactionDetailPage />} />
        </Routes>
      </main>
    </MantineProvider>
  );
}

export default App;
