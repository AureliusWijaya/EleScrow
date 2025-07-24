import { create } from "zustand";
import { User } from "../../../../declarations/elescrow_backend/elescrow_backend.did";
import { Principal } from "@dfinity/principal";

interface UserStoreState {
  loggedInUserPrincipal: Principal | null;
  actions: UserStoreActions;
}

interface UserStoreActions {
  setLoggedInUserPrincipal: (principal: Principal) => void;
}

const useUserStore = create<UserStoreState>()((set) => ({
  loggedInUserPrincipal: null,
  actions: {
    setLoggedInUserPrincipal: (principal) =>
      set(() => ({ loggedInUserPrincipal: principal })),
  },
}));

export const useLoggedInUserPrincipal = () =>
  useUserStore((state) => state.loggedInUserPrincipal);
export const useUserStoreActions = () => useUserStore((state) => state.actions);
