import React, { useEffect, useState } from "react";
import { IFormComponentProps } from "../../../models/form/form-component-props.i";
import { IFormValidatorsData } from "../../../models/form/form-validators-data.i";
import { FormComponentUtils } from "./form-component-utils.c";
import { getSize } from "@mantine/core";
import { useFormStoreActions, useIsFormDirty } from "../../store/form-store";

export function useFormComponent(props: IFormComponentProps) {
    const isFormDirty = useIsFormDirty();
    const { initComponent, onChange } = useFormStoreActions();
    const [value, setValue] = useState<any>(props.value ?? "");
    const [isValid, setIsValid] = useState<boolean>(
        props.validators ? true : false
    );
    const [errorMsg, setErrorMsg] = useState<string | null>(null);

    useEffect(() => {
        initComponent(props.fcId, isValid);
    }, []);

    const getSizeClasses = () => {
        switch (props.size) {
            case "sm":
                return "px-3 py-2 text-xs";
            case "lg":
                return "px-6 py-4 text-base";
            case "xl":
                return "px-8 py-5 text-lg";
            default:
                return "px-4 py-3 text-sm";
        }
    };

    const getLabelSizeClasses = () => {
        switch (props.size) {
            case "sm":
                return "text-xs";
            case "lg":
                return "text-base";
            case "xl":
                return "text-lg";
            default:
                return "text-sm";
        }
    };

    const bindValidatorsData = (validatorsData: IFormValidatorsData) => {
        console.log(validatorsData.isValid);
        setIsValid(validatorsData.isValid);

        if (validatorsData.errorMsg) {
            setErrorMsg(validatorsData.errorMsg);
        }
    };

    useEffect(() => {
        bindValidatorsData(
            FormComponentUtils.checkValidators(props.validators, value)
        );
    }, [isFormDirty]);

    const handleOnChange = (value: any): void => {
        setValue(value);

        const validatorsData = FormComponentUtils.checkValidators(
            props.validators,
            value
        );

        bindValidatorsData(validatorsData);
        onChange(props.fcId, value, validatorsData.errors);
    };

    useEffect(() => {
        if (isValid && errorMsg) {
            setTimeout(() => {
                setErrorMsg(null);
            }, 150);
        }
    }, [isValid]);

    const showErrorMessage = (): JSX.Element | null => {
        if (isFormDirty) {
            if (isValid) {
                return (
                    <span className="absolute -top-2 left-2 bg-primary text-error px-1 text-xs animate-fade-out">
                        {errorMsg}
                    </span>
                );
            }

            return (
                <span className="absolute -top-2 left-2 bg-primary text-error px-1 text-xs animate-fade-in">
                    {errorMsg}
                </span>
            );
        }

        return null;
    };

    return [
        value,
        isValid,
        isFormDirty,
        getSizeClasses,
        getLabelSizeClasses,
        handleOnChange,
        showErrorMessage,
    ] as const;
}
