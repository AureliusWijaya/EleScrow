import React from "react";
import { IBaseComponentProps } from "../../models/base/base-component-props.i";
import { useFormStoreActions, useIsFormDirty } from "../store/form-store";

interface IProps extends IBaseComponentProps {
    variant?: "outlined";
    onClick?: (event?: any) => any;
}

function SubmitButton(props: IProps): JSX.Element {
    const isFormDirty = useIsFormDirty();
    const { onSubmit, setIsFormDirty } = useFormStoreActions();

    const baseClasses = "px-7 py-2 rounded-lg font-medium transition-colors";

    const getVariantClasses = () => {
        if (props.variant === "outlined") {
            return "border border-secondary text-secondary hover:bg-secondary hover:text-primary-text";
        }
        return "bg-secondary hover:bg-secondary-hover text-primary-text";
    };

    const handleOnClick = (event: any) => {
        if (props.onClick) {
            props.onClick(event);
        }

        onSubmit();

        if (!isFormDirty) {
            setIsFormDirty(true);
        }
    };

    return (
        <button
            type="submit"
            className={`${baseClasses} ${getVariantClasses()} ${
                props.className || ""
            }`}
            onClick={handleOnClick}
        >
            {props.children}
        </button>
    );
}

export default SubmitButton;
