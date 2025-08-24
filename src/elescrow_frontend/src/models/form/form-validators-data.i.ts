import { IFormError } from "./form-error.i";

export interface IFormValidatorsData {
    errors: Array<IFormError> | null;
    isValid: boolean;
    errorMsg: string | null;
}
