import { create } from "zustand";

interface GeneralPageStoreState {
  loading: boolean;
  errorMessage: string;
}

interface GeneralPageStoreActions {
  updateLoading: (loading: boolean) => void;
  updateErrorMessage: (errorMessage: string) => void;
}

const useGeneralPageStore = create<
  GeneralPageStoreState & GeneralPageStoreActions
>()((set) => ({
  loading: false,
  errorMessage: "",

  updateLoading: (loading) => set(() => ({ loading: loading })),
  updateErrorMessage: (errorMessage) =>
    set(() => ({ errorMessage: errorMessage })),
}));

export const useLoading = () => useGeneralPageStore((state) => state.loading);
export const useErrorMessage = () =>
  useGeneralPageStore((state) => state.errorMessage);
