import { ReactNode } from "react";

export interface IBaseComponentProps {
    children?: ReactNode;
    size?: "sm" | "lg" | "xl";
    className?: string;
}
