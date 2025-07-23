import React, { useState, useEffect, useRef, useCallback } from "react";
import Navbar from "./shared/components/Navbar";
import HomePage from "./pages/home/HomePage";
import LoginPage from "./pages/login/LoginPage";
import RegisterPage from "./pages/register/RegisterPage";
import { Route, Routes } from "react-router-dom";
import { MantineProvider } from "@mantine/core";
import "@mantine/core/styles.css";
import DashboardPage from "./pages/dashboard/DashboardPage";
import ChatPage from "./pages/chat/ChatPage";

function App(): JSX.Element {
  return (
    <MantineProvider>
      <main className="bg-primary text-primary-text text-base">
        <Navbar />
        <Routes>
          <Route path="/" element={<HomePage />} />
          <Route path="/login" element={<LoginPage />} />
          <Route path="/register" element={<RegisterPage />} />
          <Route path="/dashboard" element={<DashboardPage />} />
          <Route path="/chat" element={<ChatPage />} />
        </Routes>
      </main>
    </MantineProvider>
  );
}

export default App;
