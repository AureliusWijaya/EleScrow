import { IFormError } from "./form-error.i";

export interface IFormValidatorFn {
    (value: any): IFormError | null;
}
