import { create } from "zustand";
import { User } from "../../../../declarations/elescrow_backend/elescrow_backend.did";
import { Principal } from "@dfinity/principal";
import { createJSONStorage, persist } from "zustand/middleware";
import { HttpAgent, Identity } from "@dfinity/agent";
import { AuthClient } from "@dfinity/auth-client";

interface AuthStoreState {
    principal: string | null;
    username: string;
    isAuthenticated: boolean;
}

interface AuthStoreActions {
    setPrincipal: (principal: AuthStoreState["principal"]) => void;

    setUsername: (username: string) => void;

    setIsAuthenticated: (
        isAuthenticated: AuthStoreState["isAuthenticated"]
    ) => void;
}

export const useAuthStore = create<AuthStoreState>()(
    persist(
        (set) => ({
            principal: null,
            username: "",
            isAuthenticated: false,
        }),
        {
            name: "user-store",
            storage: createJSONStorage(() => sessionStorage),
        }
    )
);

export const authActions: AuthStoreActions = {
    setPrincipal: (principal: AuthStoreState["principal"]) =>
        useAuthStore.setState({ principal: principal }),

    setUsername: (username: string) =>
        useAuthStore.setState({ username: username }),

    setIsAuthenticated: (isAuthenticated: AuthStoreState["isAuthenticated"]) =>
        useAuthStore.setState({ isAuthenticated }),
};

export const usePrincipal = () => useAuthStore((state) => state.principal);
export const useUsername = () => useAuthStore((state) => state.username);
export const useIsAuthenticated = () =>
    useAuthStore((state) => state.isAuthenticated);
