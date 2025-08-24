import React from "react";
import { IFormValidatorFn } from "./form-validator-fn.i";
import { IFormError } from "./form-error.i";
import { IBaseComponentProps } from "../base/base-component-props.i";

export interface IFormComponentProps extends IBaseComponentProps {
    fcId: string;
    value?: any;
    label?: string;
    validators?: IFormValidatorFn | Array<IFormValidatorFn>;
    onChange?: (
        fcId: string,
        value?: any,
        errors?: Array<IFormError> | null
    ) => any;
}
