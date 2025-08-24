import { IFormError } from "../../../models/form/form-error.i";
import { IFormValidatorFn } from "../../../models/form/form-validator-fn.i";
import { IFormValidatorsData } from "../../../models/form/form-validators-data.i";

export class FormComponentUtils {
    private static checkValidator(
        value: any,
        validatorFn: IFormValidatorFn,
        errors: Array<IFormError>
    ): void {
        const error = validatorFn(value);

        if (error != null) {
            errors.push(error);
        }
    }

    public static checkValidators(
        validators: IFormValidatorFn | Array<IFormValidatorFn> | undefined,
        value: any
    ): IFormValidatorsData {
        const errors: Array<IFormError> = new Array();
        var isFormValid: boolean = false;
        var errorMsg: string | null = null;

        if (validators) {
            if (Array.isArray(validators)) {
                validators.forEach((fn) => {
                    this.checkValidator(value, fn, errors);
                });
            } else {
                const error = this.checkValidator(value, validators, errors);
                if (error != null) {
                    errors.push(error);
                }
            }
        }

        if (errors.length === 0) {
            isFormValid = true;
        } else if (errors.length > 0) {
            errorMsg = errors[0].message;
        }

        return {
            errors: errors.length > 0 ? errors : null,
            isValid: isFormValid,
            errorMsg: errorMsg,
        };
    }
}
