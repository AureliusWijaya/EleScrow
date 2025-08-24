import React, { useEffect, useState } from "react";
import "./input.css";
import { IFormComponentProps } from "../../../models/form/form-component-props.i";
import { FormComponentUtils } from "../../utils/form/form-component-utils.c";
import { IFormValidatorsData } from "../../../models/form/form-validators-data.i";
import { useFormComponent } from "../../utils/form/form-component-hooks";

interface IProps extends IFormComponentProps {
    type: React.HTMLInputTypeAttribute;
    placeholder?: string;
}

function Input(props: IProps): JSX.Element {
    const [
        value,
        isValid,
        isFormDirty,
        getSizeClasses,
        getLabelSizeClasses,
        handleOnChange,
        showErrorMessage,
    ] = useFormComponent(props);

    return (
        <div className={`flex flex-col gap-2 ${props.className || ""}`}>
            <label
                htmlFor={props.fcId}
                className={`text-primary-text font-medium ${getLabelSizeClasses()}`}
            >
                {props.label}
            </label>

            <div className="relative">
                <input
                    id={props.fcId}
                    name={props.fcId}
                    type={props.type}
                    placeholder={props.placeholder}
                    value={value}
                    onChange={(e) => handleOnChange(e.target.value)}
                    className={`w-full bg-transparent border-2 border-secondary-text rounded-lg text-primary-text placeholder-secondary-text focus:outline-none focus:border-secondary-hover ${getSizeClasses()}${
                        isFormDirty && !isValid ? " !border-error" : ""
                    }`}
                />

                {showErrorMessage()}
            </div>
        </div>
    );
}

export default Input;
