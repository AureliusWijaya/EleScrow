import { create } from "zustand";
import { User } from "../../../../declarations/elescrow_backend/elescrow_backend.did";
import { Principal } from "@dfinity/principal";

interface UserStoreState {
  loggedInUserPrincipal: Principal | null;
  actions: UserStoreActions;
}

interface UserStoreActions {
  setLoggedInUserPrincipal: (principal: Principal) => void;
  clearLoggedInUserPrincipal: () => void;
}

const useUserStore = create<UserStoreState>()((set) => ({
  loggedInUserPrincipal: null,
  actions: {
    setLoggedInUserPrincipal: (principal) =>
      set(() => ({ loggedInUserPrincipal: principal })),
    clearLoggedInUserPrincipal: () =>
      set(() => ({ loggedInUserPrincipal: null })),
  },
}));

export const useLoggedInUserPrincipal = () =>
  useUserStore((state) => state.loggedInUserPrincipal);
export const useUserStoreActions = () => useUserStore((state) => state.actions);
