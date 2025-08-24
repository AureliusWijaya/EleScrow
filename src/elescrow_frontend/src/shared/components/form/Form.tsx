import React, { ReactElement, ReactNode, useEffect, useState } from "react";
import Input from "../input/Input";
import { IFormError } from "../../../models/form/form-error.i";
import Button from "../Button";
import { IFormComponentProps } from "../../../models/form/form-component-props.i";
import { IBaseComponentProps } from "../../../models/base/base-component-props.i";
import { IFormValidatorFn } from "../../../models/form/form-validator-fn.i";
import { FormStoreProvider } from "../../utils/form/form-store-context";

interface IProps extends IBaseComponentProps {
    submit?: (data: any) => void;
}

function Form(props: IProps): ReactNode {
    return (
        <FormStoreProvider submit={props.submit}>
            {props.children}
        </FormStoreProvider>
    );
}

export default Form;
