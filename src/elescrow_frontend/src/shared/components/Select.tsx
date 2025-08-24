import React, { useEffect, useState } from "react";
import { IFormComponentProps } from "../../models/form/form-component-props.i";
import { IFormValidatorsData } from "../../models/form/form-validators-data.i";
import { FormComponentUtils } from "../utils/form/form-component-utils.c";
import { useFormComponent } from "../utils/form/form-component-hooks";

interface IProps extends IFormComponentProps {}

function Select(props: IProps): JSX.Element {
    const [
        value,
        isValid,
        isFormDirty,
        getSizeClasses,
        getLabelSizeClasses,
        handleOnChange,
        showErrorMessage,
    ] = useFormComponent(props);

    const [isSelectOpen, setIsSelectOpen] = useState<boolean>(false);

    const toggleSelect = () => {
        setIsSelectOpen(!isSelectOpen);
    };

    return (
        <div className={`flex flex-col gap-2 ${props.className || ""}`}>
            {props.label && (
                <label
                    htmlFor={props.fcId}
                    className={`text-primary-text font-medium ${getLabelSizeClasses()}`}
                >
                    {props.label}
                </label>
            )}

            <div className="relative">
                <select
                    id={props.fcId}
                    name={props.fcId}
                    value={value}
                    onChange={(e) => handleOnChange(e.target.value)}
                    onClick={toggleSelect}
                    onBlur={() => setIsSelectOpen(false)}
                    className={`w-full bg-transparent border-2 appearance-none relative z-10 border-secondary-text rounded-lg text-primary-text focus:outline-none focus:border-secondary-hover ${getSizeClasses()}${
                        isFormDirty && !isValid ? " !border-error" : ""
                    }`}
                >
                    {props.children}
                </select>

                <i
                    className={`bi bi-caret-down-fill absolute right-4 top-1/2 -translate-y-1/2 z-0 transition-transform${
                        isSelectOpen ? " rotate-180" : ""
                    }`}
                ></i>

                {showErrorMessage()}
            </div>
        </div>
    );
}

export default Select;
