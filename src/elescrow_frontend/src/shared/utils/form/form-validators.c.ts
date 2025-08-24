import { IFormValidatorFn } from "../../../models/form/form-validator-fn.i";

export class FormValidators {
    /**
     * Validator that checks if the value is null, undefined or has a length of 0.
     *
     * @returns `IFormValidatorFn` - Validator function that returns
     * ```ts
     * { data: { required: true }, message: "Required (*)" }
     * ```
     * if the validation check fails, otherwise `null`.
     *
     * @example
     * ```ts
     * value = "";
     * { data: { required: true }, message: "Required (*)" }
     *
     * value = "test";
     * null
     * ```
     *
     */
    public static required: IFormValidatorFn = (value: any) => {
        if (
            (typeof value === "string" || value instanceof Array) &&
            value.length === 0
        ) {
            return { data: { required: true }, message: "Required (*)" };
        }

        if (value === null || value === undefined) {
            return { data: { required: true }, message: "Required (*)" };
        }

        return null;
    };

    private static readonly emailRegex: RegExp = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;

    /**
     * Validator that checks if the value is a valid email address.
     *
     * Value must be of type `string`.
     *
     * @returns `IFormValidatorFn` - Validator function that returns
     * ```ts
     * { data: { email: true }, message: "Email not valid" }
     * ```
     *
     * if the validation check fails, otherwise `null`.
     *
     * @example
     * ```ts
     * value = "invalid-email-address";
     * { data: { email: true }, message: "Email not valid" }
     *
     * value = "example@gmail.com";
     * null
     * ```
     *
     */
    public static email: IFormValidatorFn = (value: any) => {
        if (typeof value === "string" && !this.emailRegex.test(value)) {
            return {
                data: {
                    email: true,
                },
                message: "Email not valid",
            };
        }

        return null;
    };

    /**
     * Validator that checks if the value is less than the provided number.
     *
     * Value must be of type `number` or parsable `string` as number.
     *
     * @param number - number that is checked.
     *
     * @returns `IFormValidatorFn` - Validator function that returns
     * ```ts
     * { data: { min: { required: number, current: number } }, message: `Min. ${number}` }
     * ```
     * if the validation check fails, otherwise `null`.
     *
     * @example
     * ```tsx
     * const number = 5;
     *
     * value = 4;
     * { data: { min: { required: 5, current: 4 } }, message: "Min. 5" }
     *
     * value = 5;
     * null
     * ```
     *
     */
    public static min(number: number): IFormValidatorFn {
        return (value: any) => {
            if (typeof value === "number" && value < number) {
                return {
                    min: {
                        required: number,
                        current: value,
                    },
                    message: `Min. ${number}`,
                };
            }

            if (
                typeof value === "string" &&
                !Number.isNaN(value) &&
                Number.parseInt(value) < number
            ) {
                return {
                    min: {
                        required: number,
                        current: value,
                    },
                    message: `Min. ${number}`,
                };
            }

            return null;
        };
    }

    /**
     * Validator that checks if the value is more than the provided number.
     *
     * Value must be of type `number` or parsable `string` as number.
     *
     * @param number - number that is checked.
     *
     * @returns `IFormValidatorFn` - Validator function that returns
     * ```ts
     * { data: { max: { required: number, current: number } }, message: `Max. ${number}` }
     * ```
     * if the validation check fails, otherwise `null`.
     *
     * @example
     * ```tsx
     * const number = 5;
     *
     * value = 6;
     * { data: { max: { required: 5, current: 6 } }, message: "Max. 5" }
     *
     * value = 5;
     * null
     * ```
     *
     */
    public static max(number: number): IFormValidatorFn {
        return (value: any) => {
            if (typeof value === "number" && value > number) {
                return {
                    max: {
                        required: number,
                        current: value,
                    },
                    message: `Max. ${number}`,
                };
            }

            if (
                typeof value === "string" &&
                !Number.isNaN(value) &&
                Number.parseInt(value) > number
            ) {
                return {
                    max: {
                        required: number,
                        current: value,
                    },
                    message: `Max. ${number}`,
                };
            }

            return null;
        };
    }

    /**
     * Validator that checks if the length of the value is less than the provided length.
     *
     * Value must be of type `string` or `array`.
     *
     * @param length - Length of characters in a `string` or items in an `array`.
     *
     * @returns `IFormValidatorFn` - Validator function that returns
     * ```ts
     * // if value is string
     * { data: { minLength: { requiredLength: number, currentLength: number } }, message: `Min. ${length} characters` }
     *
     * // if value is array
     * { data: { minLength: { requiredLength: number, currentLength: number } }, message: `Min. ${length} items` }
     * ```
     * if the validation check fails, otherwise `null`.
     *
     * @example
     * ```tsx
     * const length = 6;
     *
     * value = "length";
     * { data: { minLength: { requiredLength: 6, currentLength: 5 } }, message: "Min. 6 characters" }
     *
     * value = "minLength";
     * null
     *
     * value = [1, 2, 3];
     * { data: { minLength: { requiredLength: 6, currentLength: 3 } }, message: "Min. 6 items" }
     *
     * value = [1, 2, 3, 4, 5, 6];
     * null
     * ```
     *
     */
    public static minLength(length: number): IFormValidatorFn {
        return (value: any) => {
            if (typeof value === "string" && value.length < length) {
                return {
                    data: {
                        minLength: {
                            requiredLength: length,
                            currentLength: value.length,
                        },
                    },
                    message: `Min. ${length} characters`,
                };
            }

            if (value instanceof Array && value.length < length) {
                return {
                    data: {
                        minLength: {
                            requiredLength: length,
                            currentLength: value.length,
                        },
                    },
                    message: `Min. ${length} items`,
                };
            }

            return null;
        };
    }

    /**
     * Validator that checks if the length of the value is greater than the provided length.
     *
     * Value must be of type `string` or `array`.
     *
     * @param length - Length of characters in a `string` or items in an `array`.
     *
     * @returns `IFormValidatorFn` - Validator function that returns
     * ```ts
     * // if value is string
     * { data: { maxLength: { requiredLength: number, currentLength: number } }, message: `Max. ${length} characters` }
     *
     * // if value is array
     * { data: { maxLength: { requiredLength: number, currentLength: number } }, message: `Max. ${length} items` }
     * ```
     * if the validation check fails, otherwise `null`.
     *
     * @example
     * ```tsx
     * const length = 6;
     *
     * value = "maxLength";
     * { data: { maxLength: { requiredLength: 6, currentLength: 9 } }, message: "Max. 6 characters" }
     *
     * value = "length";
     * null
     *
     * value = [1, 2, 3, 4, 5, 6, 7];
     * { data: { maxLength: { requiredLength: 6, currentLength: 7 } }, message: "Max. 6 items" }
     *
     * value = [1, 2, 3];
     * null
     * ```
     *
     */
    public static maxLength(length: number): IFormValidatorFn {
        return (value: any) => {
            if (typeof value === "string" && value.length > length) {
                return {
                    data: {
                        maxLength: {
                            requiredLength: length,
                            currentLength: value.length,
                        },
                    },
                    message: `Max. ${length} characters`,
                };
            }

            if (value instanceof Array && value.length > length) {
                return {
                    data: {
                        maxLength: {
                            requiredLength: length,
                            currentLength: value.length,
                        },
                    },
                    message: `Max. ${length} items`,
                };
            }

            return null;
        };
    }

    /**
     * Validator that checks if the value includes / does not include the provided string.
     *
     * Value must be of type `string`.
     *
     * @param string - string that is checked.
     * @param shouldInclude - should the value include the string or not.
     * @param message - message that is returned/displayed.
     *
     * @returns `IFormValidatorFn` - Validator function that returns
     * ```ts
     * { data: { includes: { required: string, current: string } }, message: string }
     * ```
     * if the validation check fails, otherwise `null`.
     *
     * @example
     * ```tsx
     * const string = "dolor";
     * const shouldInclude = true;
     * const message = "Must include 'dolor'"
     *
     * value = "Lorem ipsum";
     * { data: { includes: { required: "dolor", current: "Lorem ipsum" } }, message: "Must include 'dolor'" }
     *
     * value = "Lorem ipsum dolor";
     * null
     * ```
     *
     */
    public static includes(
        string: string,
        shouldInclude: boolean,
        message: string
    ): IFormValidatorFn {
        return (value: any) => {
            if (typeof value === "string") {
                const doesInclude: boolean = value.includes(string);

                if (
                    (shouldInclude && !doesInclude) ||
                    (!shouldInclude && doesInclude)
                ) {
                    return {
                        data: {
                            includes: {
                                required: string,
                                current: value,
                            },
                        },
                        message: message,
                    };
                }
            }

            return null;
        };
    }

    /**
     * Validator that checks if the value matches the provided regex pattern.
     *
     * Value must be of type `string`.
     *
     * @param pattern - regex pattern that is checked.
     * @param shouldMatch - should the value match or not.
     * @param message - message that is returned/displayed.
     *
     * @returns `IFormValidatorFn` - Validator function that returns
     * ```ts
     * { data: { pattern: { required: string, current: string } }, message: string }
     * ```
     * if the validation check fails, otherwise `null`.
     *
     * @example
     * ```tsx
     * const pattern = "[a-zA-Z ]*";
     * const shouldMatch = true;
     * const message = "Alphabetic with no spaces"
     *
     * value = "123 ";
     * { data: { pattern: { required: "[a-zA-Z ]*", current: "123 " } }, message: "Alphabetic with no spaces" }
     *
     * value = "pattern";
     * null
     * ```
     *
     */
    public static pattern(
        pattern: string | RegExp,
        shouldMatch: boolean,
        message: string
    ): IFormValidatorFn {
        return (value: any) => {
            if (typeof value === "string") {
                const doesMatch: boolean = new RegExp(pattern).test(value);

                if (
                    (shouldMatch && !doesMatch) ||
                    (!shouldMatch && doesMatch)
                ) {
                    return {
                        data: {
                            pattern: pattern.toString(),
                        },
                        message: message,
                    };
                }
            }

            return null;
        };
    }
}
