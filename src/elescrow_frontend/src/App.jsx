import { useState } from "react";
import { elescrow_backend } from "declarations/elescrow_backend";
import { BrowserRouter, Route, Routes } from "react-router-dom";
import Navbar from "./shared/components/navbar";
import HomePage from "./pages/home-page";
import LoginPage from "./pages/login-page";
import RegisterPage from "./pages/register-page";

function App() {
  return (
    <main className="flex flex-col w-full h-full bg-primary text-primary-text">
      <Navbar />
      <Routes>
        <Route path="/" element={HomePage()} />
        <Route path="/login" element={LoginPage()} />
        <Route path="/register" element={RegisterPage()} />
      </Routes>
    </main>
  );
}

export default App;
