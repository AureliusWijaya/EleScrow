import { Slide, toast, ToastOptions } from "react-toastify";

export class ToastUtils {
    private static readonly options: ToastOptions = {
        position: "bottom-right",
        autoClose: 1000,
        hideProgressBar: false,
        closeOnClick: true,
        pauseOnHover: false,
        draggable: false,
        theme: "dark",
        transition: Slide,
    };

    public static createToast(message: string): void {
        toast(message, ToastUtils.options);
    }

    public static createInfoToast(message: string): void {
        toast.info(message, ToastUtils.options);
    }

    public static createSuccessToast(message: string): void {
        toast.success(message, ToastUtils.options);
    }

    public static createWarningToast(message: string): void {
        toast.warning(message, ToastUtils.options);
    }

    public static createErrorToast(message: string): void {
        toast.error(message, ToastUtils.options);
    }
}
