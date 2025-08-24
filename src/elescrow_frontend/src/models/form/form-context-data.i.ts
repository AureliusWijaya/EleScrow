import { IFormError } from "./form-error.i";

export interface IFormContextData {
    submitButtonOnClick: () => void;
    onChange: (value: any, errors: Array<IFormError> | null) => void;
}
