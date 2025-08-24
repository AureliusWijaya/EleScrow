import React, { useContext } from "react";
import { IFormError } from "../../models/form/form-error.i";
import { FormStoreContext } from "../utils/form/form-store-context";
import { useStore } from "zustand";

interface IFormStoreActions {
    setIsFormDirty: (isFormDirty: boolean) => void;
    setIsFormValid: (isFormValid: boolean) => void;
    initComponent: (fcId: string, isValid: boolean) => void;
    onSubmit: () => void;
    onChange: (
        fcId: string,
        value: any,
        errors: Array<IFormError> | null
    ) => void;
}

export interface IFormStore {
    data: {
        [key in string]: any;
    };
    isFormDirty: boolean;
    isFormValid: boolean;
    components: Array<{
        fcId: string;
        isValid: boolean;
    }>;
    actions: IFormStoreActions;
}

const useFormStore = (selector: (state: IFormStore) => any) => {
    const store = useContext(FormStoreContext);

    if (!store) {
        throw new Error("Missing FormStoreContext");
    }

    return useStore(store, selector);
};

export const useFormData = (): {
    [key in string]: any;
} => useFormStore((state) => state.data);
export const useIsFormDirty = (): boolean =>
    useFormStore((state) => state.isFormDirty);
export const useIsFormValid = (): boolean =>
    useFormStore((state) => state.isFormValid);
export const useComponents = (): Array<{
    fcId: string;
    isValid: boolean;
}> => useFormStore((state) => state.components);
export const useFormStoreActions = (): IFormStoreActions =>
    useFormStore((state) => state.actions);
