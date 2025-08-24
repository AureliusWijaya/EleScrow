import { createContext, ReactNode, useState } from "react";
import { createStore, StoreApi } from "zustand";
import { IFormStore } from "../../store/form-store";

export const FormStoreContext = createContext<StoreApi<IFormStore> | undefined>(
    undefined
);

interface IProps {
    children?: ReactNode;
    submit?: (data: any) => void;
}

export const FormStoreProvider = (props: IProps) => {
    const [store] = useState(() =>
        createStore<IFormStore>((set, get) => ({
            data: {},
            isFormDirty: false,
            isFormValid: false,
            components: new Array(),
            actions: {
                setIsFormDirty: (isFormDirty) =>
                    set(() => ({ isFormDirty: isFormDirty })),
                setIsFormValid: (isFormValid) =>
                    set(() => ({ isFormValid: isFormValid })),
                initComponent: (fcId, isValid) =>
                    set((state) => ({
                        data: { ...state.data, [fcId]: null },
                        components: [
                            ...state.components,
                            { fcId: fcId, isValid: isValid },
                        ],
                    })),
                onSubmit: () => {
                    var isFormDirty: boolean = get().isFormDirty;

                    if (!isFormDirty) {
                        isFormDirty = true;
                    }

                    if (props.submit) {
                        props.submit(get().data);
                    }

                    set(() => ({ isFormDirty: isFormDirty }));
                },
                onChange: (fcId, value, errors) => {
                    const components = get().components;
                    const currentComponent = components.find(
                        (x) => x.fcId === fcId
                    )!;
                    var isFormValid: boolean = get().isFormValid;
                    var isComponentValid: boolean = currentComponent.isValid;

                    if (errors && currentComponent.isValid) {
                        isComponentValid = false;
                    } else {
                        isFormValid = !components.some((x) => !x.isValid);
                    }

                    set((state) => ({
                        data: { ...state.data, [fcId]: value },
                        components: state.components.map((x) => {
                            if (x.fcId === fcId) {
                                return {
                                    fcId: fcId,
                                    isValid: isComponentValid,
                                };
                            }

                            return x;
                        }),
                        isFormValid: isFormValid,
                    }));
                },
            },
        }))
    );

    return (
        <FormStoreContext.Provider value={store}>
            {props.children}
        </FormStoreContext.Provider>
    );
};
