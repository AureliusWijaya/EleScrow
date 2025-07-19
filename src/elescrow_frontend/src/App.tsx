import React, { useState } from "react";
import { elescrow_backend } from "../../declarations/elescrow_backend";
import Navbar from "./shared/components/Navbar";
import HomePage from "./pages/HomePage";
import LoginPage from "./pages/LoginPage";
import RegisterPage from "./pages/RegisterPage";
import { Route, Routes } from "react-router-dom";
import { MantineProvider } from "@mantine/core";
import "@mantine/core/styles.css";

function App(): JSX.Element {
  return (
    <MantineProvider>
      <main className="bg-primary text-primary-text text-base">
        <Navbar />
        <Routes>
          <Route path="/" element={<HomePage />} />
          <Route path="/login" element={<LoginPage />} />
          <Route path="/register" element={<RegisterPage />} />
        </Routes>
      </main>
    </MantineProvider>
  );
}

export default App;
